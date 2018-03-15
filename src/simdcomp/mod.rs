const BLOCK_LEN: usize = 128;

use std::arch::x86_64::__m128i as DataType;
use std::arch::x86_64::_mm_set1_epi32 as set1;
use std::arch::x86_64::_mm_srli_epi32 as right_shift_32;
use std::arch::x86_64::_mm_slli_epi32 as left_shift_32;
use std::arch::x86_64::_mm_or_si128 as op_or;
use std::arch::x86_64::_mm_and_si128 as op_and;
use std::arch::x86_64::_mm_lddqu_si128 as load_unaligned;
use std::arch::x86_64::_mm_storeu_si128 as store_unaligned;

use std::arch::x86_64::{_mm_srli_si128, _mm_cvtsi128_si32};

fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    unsafe {
        let _tmp1 = op_or(_mm_srli_si128(accumulator, 8), accumulator); // (A,B,C,D) xor (0,0,A,B) = (A,B,C xor A,D xor B)
        let _tmp2 = op_or(_mm_srli_si128(_tmp1, 4), _tmp1); //  (A,B,C xor A,D xor B) xor  (0,0,0,C xor A)
        _mm_cvtsi128_si32(_tmp2) as u32
    }
}

declare_bitpacker!(SIMDBitPacker);



