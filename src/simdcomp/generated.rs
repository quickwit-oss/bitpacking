


unsafe fn SIMD_nullpacker32(_in: *const __m128i, out: *mut u32) {
    memset(out,0,32 * 4 * 4);
}

unsafe fn __SIMD_fastpackwithoutmask1_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask2_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask3_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 3 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 3 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask5_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask6_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask7_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask9_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask10_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask11_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask12_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask13_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask14_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask15_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask17_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask18_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask19_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask20_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask21_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 19);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask22_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask23_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 19);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 21);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask24_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask25_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 19);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 23);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 21);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask26_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask27_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 19);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 26);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 21);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 23);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 25);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask28_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask29_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 26);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 23);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 28);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 25);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 19);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 27);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 21);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask30_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 28);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 26);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 28);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 26);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask31_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 30);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 29);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 28);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 27);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 26);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 25);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 24);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 23);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 22);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 21);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 20);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 19);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 18);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 17);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 16);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 15);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 14);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 13);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 12);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 11);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 10);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 9);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 8);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 7);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 6);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 5);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 5));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 4);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 3);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 3));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 2);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 2));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 1));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask32_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpackwithoutmask4_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i;
    let mut outer: u32;

  for outer in 0..4 {
    InReg = _mm_loadu_si128(input_ptr);
    OutReg = InReg;

    InReg = _mm_loadu_si128(input_ptr.offset(1));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));

    InReg = _mm_loadu_si128(input_ptr.offset(2));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));

    InReg = _mm_loadu_si128(input_ptr.offset(3));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));

    InReg = _mm_loadu_si128(input_ptr.offset(4));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));

    InReg = _mm_loadu_si128(input_ptr.offset(5));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));

    InReg = _mm_loadu_si128(input_ptr.offset(6));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));

    InReg = _mm_loadu_si128(input_ptr.offset(7));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    input_ptr = input_ptr.offset(8);
  }

}



unsafe fn __SIMD_fastpackwithoutmask8_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i;
    let mut outer: u32;

  for outer in 0..8 {
    InReg = _mm_loadu_si128(input_ptr);
    OutReg = InReg;

    InReg = _mm_loadu_si128(input_ptr.offset(1));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));

    InReg = _mm_loadu_si128(input_ptr.offset(2));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));

    InReg = _mm_loadu_si128(input_ptr.offset(3));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    input_ptr = input_ptr.offset(4);
  }

}



unsafe fn __SIMD_fastpackwithoutmask16_32(_in: *const __m128i, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i;
    let mut outer: u32;

  for outer in 0..16 {
    InReg = _mm_loadu_si128(input_ptr);
    OutReg = InReg;

    InReg = _mm_loadu_si128(input_ptr.offset(1));
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    input_ptr = input_ptr.offset(2);
  }

}



unsafe fn __SIMD_fastpack1_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 1) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack2_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 2) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack3_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 3) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 3 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 3 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack5_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 5) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 5 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack6_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 6) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 6 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack7_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 7) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 7 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack9_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 9) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 9 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack10_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 10) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 10 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack11_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 11) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 11 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack12_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 12) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 12 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack13_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 13) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 13 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack14_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 14) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 14 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack15_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 15) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 15 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack17_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 17) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 17 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack18_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 18) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 18 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack19_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 19) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 19 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack20_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 20) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 20 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack21_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 21) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 19);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 21 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack22_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 22) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 22 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack23_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 23) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 19);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 21);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 23 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack24_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 24) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 24 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack25_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 25) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 19);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 23);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 21);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 25 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack26_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 26) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 26 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack27_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 27) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 19);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 26);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 21);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 23);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 25);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 27 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack28_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 28) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 28 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack29_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 29) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 26);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 23);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 28);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 25);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 19);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 27);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 21);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 29 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack30_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 30) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 28);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 26);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 28);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 26);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 30 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack31_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;


    let mask = _mm_set1_epi32(((1u32 << 31) - 1u32) as i32);

    let mut InReg: __m128i = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 31));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 30);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 30));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 29);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 29));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 28);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 27);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 27));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 26);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 26));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 25);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 25));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 24);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 23);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 23));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 22);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 22));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 21);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 21));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 20);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 20));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 19);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 19));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 18);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 18));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 17);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 17));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 16);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 15);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 15));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 14);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 14));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 13);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 13));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 12);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 12));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 11);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 11));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 10);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 10));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 9);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 9));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 8);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 8));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 7);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 7));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 6);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 6));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 5);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 5));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 4);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 4));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 3);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 3));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 2);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 2));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    OutReg = _mm_srli_epi32(InReg, 31 - 1);
    input_ptr = input_ptr.offset(1);
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);

    OutReg =  _mm_or_si128(OutReg,_mm_slli_epi32(InReg, 1));
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack32_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;

    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = InReg;
    _mm_storeu_si128(out, OutReg);


}



unsafe fn __SIMD_fastpack4_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i;
   let mask = _mm_set1_epi32(((1u32 << 4) - 1u32) as i32);
   let mut outer: u32;


  for outer in 0..4 {
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(1)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 4));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(2)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(3)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 12));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(4)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(5)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 20));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(6)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(7)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 28));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    input_ptr = input_ptr.offset(8);
  }

}



unsafe fn __SIMD_fastpack8_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i;
   let mask = _mm_set1_epi32(((1u32 << 8) - 1u32) as i32);
   let mut outer: u32;


  for outer in 0..8 {
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(1)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 8));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(2)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(3)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 24));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    input_ptr = input_ptr.offset(4);
  }

}



unsafe fn __SIMD_fastpack16_32(_in: *const u32, mut out: *mut __m128i) {
    let mut input_ptr: *const __m128i = _in as *const __m128i;
    let mut OutReg: __m128i;
    let mut InReg: __m128i;
   let mask = _mm_set1_epi32(((1u32 << 16) - 1u32) as i32);
   let mut outer: u32;


  for outer in 0..16 {
    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr), mask);
    OutReg = InReg;

    InReg = _mm_and_si128(_mm_loadu_si128(input_ptr.offset(1)), mask);
    OutReg = _mm_or_si128(OutReg, _mm_slli_epi32(InReg, 16));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    input_ptr = input_ptr.offset(2);
  }

}



unsafe fn __SIMD_fastunpack2_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 2) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,26) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,28) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,26) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,28) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack3_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 3) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,21) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,27) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 3-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,19) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,25) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,28) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 3-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,17) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,23) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,26) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack4_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 4) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack5_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 5) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,25) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 5-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,23) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 5-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,21) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,26) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 5-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,19) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 5-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,17) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack6_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 6) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 6-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 6-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 6-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 6-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack7_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 7) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,21) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 7-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,17) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,24) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 7-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 7-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,23) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 7-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,19) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 7-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 7-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack8_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 8) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack9_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 9) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,22) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,17) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,21) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,19) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 9-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack10_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 10) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 10-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack11_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 11) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,17) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,19) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,20) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 11-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack12_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 12) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 12-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack13_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 13) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,17) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,18) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 13-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack14_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 14) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 14-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack15_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 15) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,15) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,16) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 15-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack16_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 16) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack17_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 17) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,14) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,13) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 17-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack18_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 18) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 18-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack19_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 19) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,12) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,11) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 19-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack20_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 20) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 20-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack21_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 21) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,10) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,9) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-19), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 21-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,11) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack22_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 22) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 22-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack23_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 23) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-19), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,11) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,7) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-21), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,8) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 23-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,9) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack24_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 24) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 24-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack25_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 25) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,11) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-19), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,5) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-23), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,9) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,6) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-21), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 25-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,7) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack26_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 26) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 26-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack27_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 27) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,7) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-19), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,9) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,4) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-26), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-21), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,11) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-23), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,3) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-25), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 27-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,5) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack28_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 28) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 28-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack29_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 29) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-26), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-23), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,11) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,5) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,2) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-28), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-25), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-19), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,7) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128(  _mm_srli_epi32(InReg,1) , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-27), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-21), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,9) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 29-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,3) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack30_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 30) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-28), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-26), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,2) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-28), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-26), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 30-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,2) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}




unsafe fn __SIMD_fastunpack31_32(mut input_ptr: *const __m128i, _out: *mut u32) {

    let mut out = _out as *mut __m128i;
    let mut InReg: __m128i = _mm_loadu_si128(input_ptr);
    let mut OutReg: __m128i;
    let mask = _mm_set1_epi32(((1u32 << 31) - 1u32) as i32);

    OutReg = _mm_and_si128( InReg , mask);
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,31) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-30), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,30) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-29), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,29) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-28), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,28) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-27), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,27) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-26), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,26) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-25), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,25) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-24), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,24) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-23), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,23) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-22), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,22) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-21), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,21) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-20), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,20) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-19), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,19) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-18), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,18) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-17), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,17) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-16), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,16) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-15), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,15) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-14), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,14) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-13), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,13) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-12), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,12) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-11), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,11) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-10), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,10) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-9), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,9) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-8), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,8) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-7), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,7) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-6), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,6) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-5), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,5) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-4), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,4) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-3), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,3) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-2), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,2) ;
    input_ptr = input_ptr.offset(1);
    InReg=_mm_loadu_si128(input_ptr);

    OutReg = _mm_or_si128(OutReg, _mm_and_si128(_mm_slli_epi32(InReg, 31-1), mask));
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);

    OutReg =   _mm_srli_epi32(InReg,1) ;
    _mm_storeu_si128(out, OutReg);
    out = out.offset(1);


}


