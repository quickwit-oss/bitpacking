const NUM_INTS_PER_REGISTER: usize = 4;

use std::arch::x86_64::__m128i as DataType;
use std::arch::x86_64::_mm_set1_epi32 as set1;
use std::arch::x86_64::_mm_srli_epi32 as right_shift_32;
use std::arch::x86_64::_mm_slli_epi32 as left_shift_32;
use std::arch::x86_64::_mm_or_si128 as op_or;
use std::arch::x86_64::_mm_and_si128 as op_and;
use std::arch::x86_64::_mm_lddqu_si128 as load_unaligned;
use std::arch::x86_64::_mm_storeu_si128 as store_unaligned;

use std::arch::x86_64::{_mm_add_epi32,  _mm_shuffle_epi32, _mm_srli_si128, _mm_sub_epi32, _mm_slli_si128, _mm_cvtsi128_si32};

#[allow(non_snake_case)]
unsafe fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    let a__b__c__d_ = accumulator;
    let ______a__b_ = _mm_srli_si128(a__b__c__d_, 8);
    let a__b__ca_db = op_or(a__b__c__d_, ______a__b_);
    let ___a__b__ca = _mm_srli_si128(a__b__ca_db, 4);
    let _______cadb = op_or(a__b__ca_db, ___a__b__ca);
    _mm_cvtsi128_si32(_______cadb) as u32
}

#[target_feature(enable="sse3")]
unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
    _mm_sub_epi32(curr,
                  op_or(_mm_slli_si128(curr, 4),
                        _mm_srli_si128(prev, 12))
    )
}


#[target_feature(enable="sse3")]
#[allow(non_snake_case)]
unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
    let offset = _mm_shuffle_epi32(prev, 0xff);
    let a__b__c__d_ = delta;
    let ______a__b_ = _mm_slli_si128(delta, 8);
    let a__b__ca_db = _mm_add_epi32(______a__b_, a__b__c__d_);
    let ___a__b__ca = _mm_slli_si128(a__b__ca_db, 4);
    let a_ab_abc_abcd: DataType = _mm_add_epi32(___a__b__ca, a__b__ca_db);
    _mm_add_epi32(offset, a_ab_abc_abcd)
}


declare_bitpacker!(SSE3BitPacker, 128,  target_feature(enable="sse3"));



