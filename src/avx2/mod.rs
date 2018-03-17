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

#[allow(non_snake_case)]
fn or_collapse_to_u32(accumulator: DataType) -> u32 {
    unsafe {
        let a__b__c__d__e__f__g__h_ = accumulator;
        let ______a__b__c__d__e__f =_mm256_srli_si256(a__b__c__d__e__f__g__h_, 8);
        let a__b__ca_db_ce_df_ge_hf = op_or(a__b__c__d__e__f__g__h_, ______a__b__c__d__e__f);
        let ___a__b__ca_db_ce_df_ge = _mm256_srli_si256(a__b__ca_db_ce_df_ge_hf, 4);
        let _________cadb_________gehf = op_or(a__b__ca_db_ce_df_ge_hf, ___a__b__ca_db_ce_df_ge);
        let cadb = _mm256_extract_epi32(_________cadb_________gehf, 4);
        let gehf = _mm256_extract_epi32(_________cadb_________gehf, 4);
        (cadb | gehf) as u32
    }
}

declare_bitpacker!(AVX2BitPacker);
