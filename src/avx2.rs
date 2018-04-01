const NUM_INTS_PER_REGISTER: usize = 8;

use std::arch::x86_64::__m256i as DataType;
use std::arch::x86_64::_mm256_set1_epi32 as set1;
use std::arch::x86_64::_mm256_srli_epi32 as right_shift_32;
use std::arch::x86_64::_mm256_slli_epi32 as left_shift_32;
use std::arch::x86_64::_mm256_or_si256 as op_or;
use std::arch::x86_64::_mm256_and_si256 as op_and;
use std::arch::x86_64::_mm256_lddqu_si256 as load_unaligned;
use std::arch::x86_64::_mm256_storeu_si256 as store_unaligned;

use std::arch::x86_64::{_mm256_permute2f128_si256, _mm256_extract_epi32, _mm256_sub_epi32, _mm256_add_epi32, _mm256_srli_si256, _mm256_slli_si256, _mm256_shuffle_epi32};

#[allow(non_snake_case)]
unsafe fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    let a__b__c__d__e__f__g__h_ = accumulator;
    let ______a__b__c__d__e__f =_mm256_srli_si256(a__b__c__d__e__f__g__h_, 8);
    let a__b__ca_db_ce_df_ge_hf = op_or(a__b__c__d__e__f__g__h_, ______a__b__c__d__e__f);
    let ___a__b__ca_db_ce_df_ge = _mm256_srli_si256(a__b__ca_db_ce_df_ge_hf, 4);
    let _________cadb_________gehf = op_or(a__b__ca_db_ce_df_ge_hf, ___a__b__ca_db_ce_df_ge);
    let cadb = _mm256_extract_epi32(_________cadb_________gehf, 4);
    let gehf = _mm256_extract_epi32(_________cadb_________gehf, 4);
    (cadb | gehf) as u32
}

unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
    let left_shift = _mm256_slli_si256(curr, 4);
    let curr_shift = _mm256_srli_si256(curr, 12);
    let curr_right_only = _mm256_permute2f128_si256(curr_shift, curr_shift, 8    );
    let prev_shift = _mm256_srli_si256(prev, 12);
    let sub_left = _mm256_permute2f128_si256(prev_shift, prev_shift, 3 | (8 << 4)   );
    let diff = op_or(left_shift, op_or(curr_right_only, sub_left));
    _mm256_sub_epi32(curr,diff)
}

#[allow(non_snake_case)]
unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
    // There is a probably a better way to implement this...
    let offset_repeat = _mm256_shuffle_epi32(prev, 0xff);
    let offset = _mm256_permute2f128_si256(offset_repeat, offset_repeat, 3 | (8 << 4)   );
    let a__b__c__d__e__f__g__h__ = delta;
    let ______a__b________e__f__ = _mm256_slli_si256(delta, 8);
    let a__b__ca_db_e__f__ge_fh_ = _mm256_add_epi32(a__b__c__d__e__f__g__h__, ______a__b________e__f__);
    let ___a__b__ca____e__f__ge_ = _mm256_slli_si256(a__b__ca_db_e__f__ge_fh_, 4);
    let halved_prefix_sum = _mm256_add_epi32(___a__b__ca____e__f__ge_, a__b__ca_db_e__f__ge_fh_);
    let offseted_halved_prefix_sum = _mm256_add_epi32(halved_prefix_sum, offset);
    let select_last_low = _mm256_shuffle_epi32(offseted_halved_prefix_sum, 0xff);
    let high_offset = _mm256_permute2f128_si256(select_last_low, select_last_low, 8 | 0   );
    _mm256_add_epi32(high_offset, offseted_halved_prefix_sum)
}

declare_bitpacker!(AVX2BitPacker, 256, target_feature(enable="avx2"));
