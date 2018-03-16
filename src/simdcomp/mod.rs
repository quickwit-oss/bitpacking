const BLOCK_LEN: usize = 128;

use std::arch::x86_64::__m128i as DataType;
use std::arch::x86_64::_mm_set1_epi32 as set1;
use std::arch::x86_64::_mm_srli_epi32 as right_shift_32;
use std::arch::x86_64::_mm_slli_epi32 as left_shift_32;
use std::arch::x86_64::_mm_or_si128 as op_or;
use std::arch::x86_64::_mm_and_si128 as op_and;
use std::arch::x86_64::_mm_lddqu_si128 as load_unaligned;
use std::arch::x86_64::_mm_storeu_si128 as store_unaligned;


use std::arch::x86_64::{_mm_srli_si128, _mm_sub_epi32, _mm_slli_si128, _mm_cvtsi128_si32};

unsafe fn delta(curr: DataType, prev: DataType) -> DataType {
    _mm_sub_epi32(curr,
                  op_or(_mm_slli_si128(curr, 4),
                               _mm_srli_si128(prev, 12))
    )
}

#[allow(non_snake_case)]
fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    unsafe {
        let a__b__c__d_ = accumulator;
        let ______a__b_ = _mm_srli_si128(a__b__c__d_, 8);
        let a__b__ca_db = op_or(a__b__c__d_, ______a__b_);
        let ___a__b__ca = _mm_srli_si128(a__b__ca_db, 4);
        let _______cadb = op_or(a__b__ca_db, ___a__b__ca);
        _mm_cvtsi128_si32(_______cadb) as u32
    }
}

declare_bitpacker!(SIMDBitPacker);



