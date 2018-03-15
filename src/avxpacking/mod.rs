const BLOCK_LEN: usize = 256;

use std::arch::x86_64::__m256i as DataType;
use std::arch::x86_64::_mm256_set1_epi32 as set1;
use std::arch::x86_64::_mm256_srli_epi32 as right_shift_32;
use std::arch::x86_64::_mm256_slli_epi32 as left_shift_32;
use std::arch::x86_64::_mm256_or_si256 as op_or;
use std::arch::x86_64::_mm256_and_si256 as op_and;
use std::arch::x86_64::_mm256_lddqu_si256 as load_unaligned;
use std::arch::x86_64::_mm256_storeu_si256 as store_unaligned;

use std::arch::x86_64::{_mm256_extract_epi32, _mm256_srli_si256};

fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    unsafe {
        let tmp1 = op_or(_mm256_srli_si256(accumulator, 8), accumulator);
        let tmp2 = op_or(_mm256_srli_si256(tmp1, 4), tmp1);
        let ans1 = _mm256_extract_epi32(tmp2,0) as u32;
        let ans2 = _mm256_extract_epi32(tmp2,4) as u32;
        ans1 | ans2
    }
}

declare_bitpacker!(AVXBitPacker);
