use super::{BitPacker, UnsafeBitPacker};

#[cfg(target_arch = "x86_64")]
use crate::Available;

const BLOCK_LEN: usize = 32 * 8;

#[cfg(target_arch = "x86_64")]
mod avx2 {

    use super::BLOCK_LEN;
    use crate::Available;

    use std::arch::x86_64::__m256i as DataType;
    use std::arch::x86_64::_mm256_and_si256 as op_and;
    use std::arch::x86_64::_mm256_lddqu_si256 as load_unaligned;
    use std::arch::x86_64::_mm256_or_si256 as op_or;
    use std::arch::x86_64::_mm256_set1_epi32 as set1;
    use std::arch::x86_64::_mm256_slli_epi32 as left_shift_32;
    use std::arch::x86_64::_mm256_srli_epi32 as right_shift_32;
    use std::arch::x86_64::_mm256_storeu_si256 as store_unaligned;

    use std::arch::x86_64::{
        _mm256_add_epi32, _mm256_extract_epi32, _mm256_permute2f128_si256, _mm256_shuffle_epi32,
        _mm256_slli_si256, _mm256_srli_si256, _mm256_sub_epi32,
    };

    #[allow(non_snake_case)]
    unsafe fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        let a__b__c__d__e__f__g__h_ = accumulator;
        let ______a__b________e__f = _mm256_srli_si256(a__b__c__d__e__f__g__h_, 8);
        let a__b__ca_db_e__f__ge_hf = op_or(a__b__c__d__e__f__g__h_, ______a__b________e__f);
        let ___a__b__ca____e__f__ge = _mm256_srli_si256(a__b__ca_db_e__f__ge_hf, 4);
        let _________cadb______gehf = op_or(a__b__ca_db_e__f__ge_hf, ___a__b__ca____e__f__ge);
        let cadb = _mm256_extract_epi32(_________cadb______gehf, 0);
        let gehf = _mm256_extract_epi32(_________cadb______gehf, 4);
        (cadb | gehf) as u32
    }

    unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        let left_shift = _mm256_slli_si256(curr, 4);
        let curr_shift = _mm256_srli_si256(curr, 12);
        let curr_right_only = _mm256_permute2f128_si256(curr_shift, curr_shift, 8);
        let prev_shift = _mm256_srli_si256(prev, 12);
        let sub_left = _mm256_permute2f128_si256(prev_shift, prev_shift, 3 | (8 << 4));
        let diff = op_or(left_shift, op_or(curr_right_only, sub_left));
        _mm256_sub_epi32(curr, diff)
    }

    #[allow(non_snake_case)]
    unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
        // There is a probably a better way to implement this...
        let offset_repeat = _mm256_shuffle_epi32(prev, 0xff);
        let offset = _mm256_permute2f128_si256(offset_repeat, offset_repeat, 3 | (8 << 4));
        let a__b__c__d__e__f__g__h__ = delta;
        let ______a__b________e__f__ = _mm256_slli_si256(delta, 8);
        let a__b__ca_db_e__f__ge_fh_ =
            _mm256_add_epi32(a__b__c__d__e__f__g__h__, ______a__b________e__f__);
        let ___a__b__ca____e__f__ge_ = _mm256_slli_si256(a__b__ca_db_e__f__ge_fh_, 4);
        let halved_prefix_sum =
            _mm256_add_epi32(___a__b__ca____e__f__ge_, a__b__ca_db_e__f__ge_fh_);
        let offseted_halved_prefix_sum = _mm256_add_epi32(halved_prefix_sum, offset);
        let select_last_low = _mm256_shuffle_epi32(offseted_halved_prefix_sum, 0xff);
        let high_offset = _mm256_permute2f128_si256(select_last_low, select_last_low, 8 | 0);
        _mm256_add_epi32(high_offset, offseted_halved_prefix_sum)
    }

    unsafe fn add(left: DataType, right: DataType) -> DataType {
        _mm256_add_epi32(left, right)
    }

    unsafe fn sub(left: DataType, right: DataType) -> DataType {
        _mm256_sub_epi32(left, right)
    }

    declare_bitpacker!(target_feature(enable = "avx2"));

    impl Available for UnsafeBitPackerImpl {
        fn available() -> bool {
            is_x86_feature_detected!("avx2")
        }
    }
}

mod scalar {

    use super::BLOCK_LEN;
    use crate::Available;
    use std::ptr;

    type DataType = [u32; 8];

    fn set1(el: i32) -> DataType {
        [el as u32; 8]
    }

    fn right_shift_32<const N: i32>(el: DataType) -> DataType {
        [
            el[0] >> N,
            el[1] >> N,
            el[2] >> N,
            el[3] >> N,
            el[4] >> N,
            el[5] >> N,
            el[6] >> N,
            el[7] >> N,
        ]
    }

    fn left_shift_32<const N: i32>(el: DataType) -> DataType {
        [
            el[0] << N,
            el[1] << N,
            el[2] << N,
            el[3] << N,
            el[4] << N,
            el[5] << N,
            el[6] << N,
            el[7] << N,
        ]
    }

    fn op_or(left: DataType, right: DataType) -> DataType {
        [
            left[0] | right[0],
            left[1] | right[1],
            left[2] | right[2],
            left[3] | right[3],
            left[4] | right[4],
            left[5] | right[5],
            left[6] | right[6],
            left[7] | right[7],
        ]
    }

    fn op_and(left: DataType, right: DataType) -> DataType {
        [
            left[0] & right[0],
            left[1] & right[1],
            left[2] & right[2],
            left[3] & right[3],
            left[4] & right[4],
            left[5] & right[5],
            left[6] & right[6],
            left[7] & right[7],
        ]
    }

    unsafe fn load_unaligned(addr: *const DataType) -> DataType {
        ptr::read_unaligned(addr)
    }

    unsafe fn store_unaligned(dst: *mut DataType, data: DataType) {
        ptr::write_unaligned(dst, data);
    }

    fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        ((accumulator[0] | accumulator[1]) | (accumulator[2] | accumulator[3]))
            | ((accumulator[4] | accumulator[5]) | (accumulator[6] | accumulator[7]))
    }

    fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        [
            curr[0].wrapping_sub(prev[7]),
            curr[1].wrapping_sub(curr[0]),
            curr[2].wrapping_sub(curr[1]),
            curr[3].wrapping_sub(curr[2]),
            curr[4].wrapping_sub(curr[3]),
            curr[5].wrapping_sub(curr[4]),
            curr[6].wrapping_sub(curr[5]),
            curr[7].wrapping_sub(curr[6]),
        ]
    }

    fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
        let el0 = offset[7].wrapping_add(delta[0]);
        let el1 = el0.wrapping_add(delta[1]);
        let el2 = el1.wrapping_add(delta[2]);
        let el3 = el2.wrapping_add(delta[3]);
        let el4 = el3.wrapping_add(delta[4]);
        let el5 = el4.wrapping_add(delta[5]);
        let el6 = el5.wrapping_add(delta[6]);
        let el7 = el6.wrapping_add(delta[7]);
        [el0, el1, el2, el3, el4, el5, el6, el7]
    }

    fn add(left: DataType, right: DataType) -> DataType {
        [
            left[0].wrapping_add(right[0]),
            left[1].wrapping_add(right[1]),
            left[2].wrapping_add(right[2]),
            left[3].wrapping_add(right[3]),
            left[4].wrapping_add(right[4]),
            left[5].wrapping_add(right[5]),
            left[6].wrapping_add(right[6]),
            left[7].wrapping_add(right[7]),
        ]
    }

    fn sub(left: DataType, right: DataType) -> DataType {
        [
            left[0].wrapping_sub(right[0]),
            left[1].wrapping_sub(right[1]),
            left[2].wrapping_sub(right[2]),
            left[3].wrapping_sub(right[3]),
            left[4].wrapping_sub(right[4]),
            left[5].wrapping_sub(right[5]),
            left[6].wrapping_sub(right[6]),
            left[7].wrapping_sub(right[7]),
        ]
    }

    // The `cfg(any(debug, not(debug)))` is here to put an attribute that has no effect.
    //
    // For other bitpacker, we enable specific CPU instruction set, but for the
    // scalar bitpacker none is required.
    declare_bitpacker!(cfg(any(debug, not(debug))));

    impl Available for UnsafeBitPackerImpl {
        fn available() -> bool {
            true
        }
    }
}

#[derive(Clone, Copy)]
enum InstructionSet {
    #[cfg(target_arch = "x86_64")]
    AVX2,
    Scalar,
}

/// `BitPacker8x` packs integers in groups of 8. This gives an opportunity
/// to leverage `AVX2` instructions to encode and decode the stream.
/// One block must contain `256 integers`.
#[derive(Clone, Copy)]
pub struct BitPacker8x(InstructionSet);

impl BitPacker for BitPacker8x {
    const BLOCK_LEN: usize = BLOCK_LEN;

    fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if avx2::UnsafeBitPackerImpl::available() {
                return BitPacker8x(InstructionSet::AVX2);
            }
        }
        BitPacker8x(InstructionSet::Scalar)
    }

    fn compress(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => {
                    avx2::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits)
                }
            }
        }
    }

    fn compress_sorted(
        &self,
        initial: u32,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => avx2::UnsafeBitPackerImpl::compress_sorted(
                    initial,
                    decompressed,
                    compressed,
                    num_bits,
                ),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::compress_sorted(
                    initial,
                    decompressed,
                    compressed,
                    num_bits,
                ),
            }
        }
    }

    fn compress_strictly_sorted(
        &self,
        initial: Option<u32>,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => avx2::UnsafeBitPackerImpl::compress_strictly_sorted(
                    initial,
                    decompressed,
                    compressed,
                    num_bits,
                ),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::compress_strictly_sorted(
                    initial,
                    decompressed,
                    compressed,
                    num_bits,
                ),
            }
        }
    }

    fn decompress(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => {
                    avx2::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
                }
            }
        }
    }

    fn decompress_sorted(
        &self,
        initial: u32,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => avx2::UnsafeBitPackerImpl::decompress_sorted(
                    initial,
                    compressed,
                    decompressed,
                    num_bits,
                ),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::decompress_sorted(
                    initial,
                    compressed,
                    decompressed,
                    num_bits,
                ),
            }
        }
    }

    fn decompress_strictly_sorted(
        &self,
        initial: Option<u32>,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => avx2::UnsafeBitPackerImpl::decompress_strictly_sorted(
                    initial,
                    compressed,
                    decompressed,
                    num_bits,
                ),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::decompress_strictly_sorted(
                    initial,
                    compressed,
                    decompressed,
                    num_bits,
                ),
            }
        }
    }

    fn num_bits(&self, decompressed: &[u32]) -> u8 {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => avx2::UnsafeBitPackerImpl::num_bits(decompressed),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::num_bits(decompressed),
            }
        }
    }

    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8 {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => {
                    avx2::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
                }
            }
        }
    }

    fn num_bits_strictly_sorted(&self, initial: Option<u32>, decompressed: &[u32]) -> u8 {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX2 => {
                    avx2::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed)
                }
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[cfg(test)]
mod tests {
    use super::BLOCK_LEN;
    use super::{avx2, scalar};
    use crate::tests::test_util_compatible;
    use crate::Available;

    #[test]
    fn test_compatible() {
        if avx2::UnsafeBitPackerImpl::available() {
            test_util_compatible::<scalar::UnsafeBitPackerImpl, avx2::UnsafeBitPackerImpl>(
                BLOCK_LEN,
            );
        }
    }
}
