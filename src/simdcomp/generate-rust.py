import requests
from string import Template
import re

url = "https://raw.githubusercontent.com/lemire/simdcomp/759170f9d29c2719de8bc7a8519d074e0adedb15/src/simdbitpacking.c"
data = requests.get(url).text

REMOVE_COMMENTS = re.compile("/\*[^/]+\*/")
data = REMOVE_COMMENTS.sub("", data)

lines = [line for line in data.split("\n")]


def define_variables(variables, **kwargs):
    lines = []
    for varname in variables.split(","):
        varname = varname.strip()
        line = "let mut %s: __m128i;" % varname
        lines.append(line)
    return "\n".join(lines)

PTNS = [
    ("#include.*", None),
    (
        "static void SIMD_nullunpacker(?P<numbits>\d+)\(const __m128i \* _in  , uint32_t \*    out\) \{", 
        "unsafe fn SIMD_nullpacker$numbits(_in: *const __m128i, out: *mut u32) {"
    ),
    (
        "static void __SIMD_fastpack(?P<numbits>\d+)_32\(.*\) \{", 
        "unsafe fn __SIMD_fastpack${numbits}_32(_in: *const u32, mut out: *mut __m128i) {"
    ),
    (
        "static void __SIMD_fastunpack(?P<numdigits>\d+)_32\(const  __m128i\*   in, uint32_t \*    _out\) \{",
        "unsafe fn __SIMD_fastunpack${numdigits}_32(mut input_ptr: *const __m128i, _out: *mut u32) {"
    ),
    (
        "__m128i\*\s*out\s*=\s*\(__m128i\*\)\(_out\);",
        "let mut out = _out as *mut __m128i;"
    ),
    ( 
        "const __m128i\s*mask =\s*_mm_set1_epi32\(1\);",
        "let mask = _mm_set1_epi32(1i32);"
    ),
    ( 
        "const __m128i\s*mask =  _mm_set1_epi32\(\(1U<<(?P<shift>\d+)\)-1\);",
        "let mask = _mm_set1_epi32(((1u32 << $shift) - 1u32) as i32);"
    ),
    ( 
        "const __m128i\s*\*\s*in\s*=\s*\(const __m128i\*\)\(_in\);",
        "let mut input_ptr: *const __m128i = _in as *const __m128i;"
    ),
    (
        "uint32_t i, shift = 0;",
        "let mut shift = 0i32;"
    ),
    (
        "for \(i = 0; i < 8; \+\+i\) \{",
        "for _ in 0..8 {"
    ),
    (
        "__m128i\s+(?P<variables>[a-zA-Z0-9,\s]+);",
        define_variables
    ),
    (
        "static void __SIMD_fastpackwithoutmask(?P<numbits>\d+)_32.*",
        "unsafe fn __SIMD_fastpackwithoutmask${numbits}_32(_in: *const __m128i, mut out: *mut __m128i) {"
    ),
    (
        "\w+\s*=\s*[_a-zA-Z0-9]+\([_a-zA-Z0-9]+\(\w+\),\s*\w+\);",
        "$original"
    ),
    (
        "InReg = _mm_and_si128\(_mm_loadu_si128\(in\+(?P<numbits>\d+)\), mask\);",
        "InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset($numbits)), mask);"
    ),
    (
        "OutReg = _mm_and_si128\(\s*InReg\s*,\s*mask\);",
        "$original"
    ),
    (
        "_mm_storeu_si128\(out\+\+, OutReg\);",
        "_mm_storeu_si128(out, OutReg);\nout = out.offset(1);"
    ),
    (
        "_mm_storeu_si128\(out\+\+, OutReg(?P<digit>\d)\);",
        "_mm_storeu_si128(out, OutReg$digit);\nout = out.offset(1);"
    ),
    (
        "OutReg(?P<numout>\d) = _mm_and_si128\(\s*_mm_srli_epi32\(InReg(?P<numin>\d),shift\+\+\)\s*,\s*mask\);",
        "OutReg$numout = _mm_and_si128(_mm_srli_epi32(InReg$numin, shift), mask); shift += 1i32;"
    ),
    (
        "OutReg =\s*_mm_srli_epi32\(InReg,\d+\)\s*;",
        "$original"
    ),
    (
        "OutReg = _mm_or_si128\(OutReg, _mm_and_si128\(_mm_slli_epi32\(InReg,\s*[\d+\-]+\), mask\)\);",
        "$original"
    ),
    (
        "__m128i\s+(?P<destval>[a-zA-Z0-9]+) = (?P<right>_mm[a-zA-Z0-9_]+\(.*\);)",
        "let mut $destval: __m128i = $right"
    ),
    ("__m128i\s+(?P<vname>[_a-zA-Z0-9]+);", "let mut $vname: __128i;"),
    (
        "__m128i\s+(?P<vname>[_a-zA-Z0-9]+)\s*=\s*(?P<fname>_mm[_a-zA-Z0-9]+)\((?P<vinputname>[_a-zA-Z0-9]+)\);", 
        "let mut $vname: __128i = $fname($vinputname);"
    ),
    (
        "(?P<vname>[_a-zA-Z0-9]+)\s*=\s*(?P<fname>_mm[_a-zA-Z0-9]+)\((?P<vinputname>[_a-zA-Z0-9]+)\+(?P<offset>\d+)\);", 
        "$vname = $fname($vinputname.offset($offset));"
    ),
    (
        "(?P<vname>[_a-zA-Z0-9]+)\s*=\s*(?P<fname>_mm[_a-zA-Z0-9]+)\((?P<vinputname>[_a-zA-Z0-9]+)\);", 
        "$vname = $fname($vinputname);"
    ),
    (
        "(?P<vname>[_a-zA-Z0-9]+)\s*=\s*_mm_and_si128\(_mm_loadu_si128\(\+\+in\), mask\);", 
        "input_ptr = input_ptr.offset(1);\n$vname = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);"
    ),
    (
        "(?P<vname>[_a-zA-Z0-9]+)\s*=\s*(?P<fname>_mm[_a-zA-Z0-9]+)\(\+\+in\);", 
        "input_ptr = input_ptr.offset(1);\n$vname=$fname(input_ptr);"
    ),
    (
        "OutReg\s*=\s*_mm_or_si128\(OutReg,_mm_slli_epi32\(InReg, \d+\)\);",
        "$original"
    ),
    (
        "OutReg = _mm_or_si128\(OutReg, _mm_slli_epi32\(InReg,.*\)\);",
        "$original"
    ),
    (
            "__m128i\s+(?P<destvar>[a-zA-Z0-9_]+)\s*=\s*(?P<origvar>[a-zA-Z0-9_]+);",
            "let mut $destvar: __m128i = $origvar ;"
    ),
    ("OutReg = InReg;", "$original"),
    ("\(void\) _in;", None),
    (
        "memset\(out,.*\);", 
        "$original"
    ),
    (
        "_mm_storeu_si128\(out, OutReg\);",
        "$original"
    ),
    (
        "\+\+out;",
        "out = out.offset(1);"
    ),
    (
        "in\+=(?P<offset>\d+);",
        "input_ptr = input_ptr.offset($offset);"
    ),
    (
        "OutReg = _mm_srli_epi32\(InReg,.*\);",
        "$original"
    ),
    (
        "uint32_t outer;",
        "let mut outer: u32;"
    ),
    (
        "for\(outer=0; outer< (?P<iter>\d+) ;\+\+outer\) {",
        "for outer in 0..$iter {"
    ),
    ('\}', "}"),
    (
        "OutReg = _mm_and_si128\(\s*_mm_srli_epi32\(InReg,\d+\)\s*,\s*mask\);", 
        "$original"
    ),
]


def donothing(*args, **kargs):
    return None

def make_translate(translation):
    if translation is None:
        return donothing
    if callable(translation):
        return translation
    tmpl = Template(translation)
    return tmpl.substitute

PTNS = [
   (re.compile("^" + ptn + "$"), make_translate(translation)) 
   for (ptn, translation) in PTNS
]


import sys

INDENT_PTN = re.compile("^(\s*)(\S.*)?$")

def rename(name):
    if name == "in":
        return "input_ptr"
    else:
        return name

for (nline, line) in enumerate(lines):
    indent_match = INDENT_PTN.match(line)
    indent = indent_match.group(1)
    line = indent_match.group(2)
    if line is None:
        print ""
        continue
    for (ptn, translation) in PTNS: 
        m = ptn.match(line)
        if m is not None:
            if translation is not None:
                params = m.groupdict()
                params["original"] = line
                params = {
                    k: rename(v)
                    for k,v in params.items()
                }
                message = translation(**params)
                if message is not None:
                    message = message.replace("(in)", "(input_ptr)")
                    for message_line in message.split("\n"):
                        print indent + message_line
                break
            else:
                break
    else:
        print >> sys.stderr, "Failed to find translation for this line: ", nline
        print >> sys.stderr, line
        raise


