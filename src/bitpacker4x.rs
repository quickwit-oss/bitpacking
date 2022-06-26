use super::{BitPacker, UnsafeBitPacker};

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use crate::Available;

const BLOCK_LEN: usize = 32 * 4;

#[cfg(any(target_arch = "x86_64"))]
mod sse3 {

    use super::BLOCK_LEN;
    use crate::Available;

    use std::arch::x86_64::__m128i as DataType;
    use std::arch::x86_64::_mm_and_si128 as op_and;
    use std::arch::x86_64::_mm_lddqu_si128 as load_unaligned;
    use std::arch::x86_64::_mm_or_si128 as op_or;
    use std::arch::x86_64::_mm_set1_epi32 as set1;
    use std::arch::x86_64::_mm_slli_epi32 as left_shift_32;
    use std::arch::x86_64::_mm_srli_epi32 as right_shift_32;
    use std::arch::x86_64::_mm_storeu_si128 as store_unaligned;
    use std::arch::x86_64::{
        _mm_add_epi32, _mm_cvtsi128_si32, _mm_shuffle_epi32, _mm_slli_si128, _mm_srli_si128,
        _mm_sub_epi32,
    };

    #[allow(non_snake_case)]
    #[inline]
    unsafe fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        let a__b__c__d_ = accumulator;
        let ______a__b_ = _mm_srli_si128(a__b__c__d_, 8);
        let a__b__ca_db = op_or(a__b__c__d_, ______a__b_);
        let ___a__b__ca = _mm_srli_si128(a__b__ca_db, 4);
        let _______cadb = op_or(a__b__ca_db, ___a__b__ca);
        _mm_cvtsi128_si32(_______cadb) as u32
    }

    #[target_feature(enable = "sse3")]
    unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        _mm_sub_epi32(
            curr,
            op_or(_mm_slli_si128(curr, 4), _mm_srli_si128(prev, 12)),
        )
    }

    #[target_feature(enable = "sse3")]
    #[allow(non_snake_case)]
    #[inline]
    unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
        let offset = _mm_shuffle_epi32(prev, 0xff);
        let a__b__c__d_ = delta;
        let ______a__b_ = _mm_slli_si128(delta, 8);
        let a__b__ca_db = _mm_add_epi32(______a__b_, a__b__c__d_);
        let ___a__b__ca = _mm_slli_si128(a__b__ca_db, 4);
        let a_ab_abc_abcd: DataType = _mm_add_epi32(___a__b__ca, a__b__ca_db);
        _mm_add_epi32(offset, a_ab_abc_abcd)
    }

    declare_bitpacker!(target_feature(enable = "sse3"));

    impl Available for UnsafeBitPackerImpl {
        fn available() -> bool {
            is_x86_feature_detected!("sse3")
        }
    }
}

#[cfg(any(target_arch = "aarch64"))]
mod aarch64 {

    use super::BLOCK_LEN;
    use crate::Available;

    use std::arch::aarch64::uint32x4_t as DataType;
    use std::arch::aarch64::vandq_u32 as op_and;
    use std::arch::aarch64::vorrq_u32 as op_or;
    use std::arch::aarch64::{
        vaddq_u32, vdupq_laneq_u32, vextq_u32, vgetq_lane_u32, vld1q_u32, vmovq_n_s32, vmovq_n_u32,
        vshlq_u32, vst1q_u32, vsubq_u32,
    };
    unsafe fn set1(v: i32) -> DataType {
        vmovq_n_u32(v as u32)
    }
    unsafe fn load_unaligned(p: *const DataType) -> DataType {
        let up = std::mem::transmute::<_, *const u32>(p);
        vld1q_u32(up)
    }
    unsafe fn store_unaligned(p: *const DataType, v: DataType) {
        let op = std::mem::transmute::<_, *mut u32>(p);
        vst1q_u32(op, v)
    }
    use std::arch::aarch64::vshlq_n_u32 as left_shift_32;
    unsafe fn right_shift_32<const N: i32>(v: DataType) -> DataType {
        // Ideally we'd use vshrq_n_u32() here but it does not allow zero values and AFAICT there
        // is no way to allow it to compile so long as macros pass N=0. Instead use vshlq_u32()
        // with a negative value (which results in shift-right).
        vshlq_u32(v, vmovq_n_s32(-N))
    }

    #[target_feature(enable = "neon")]
    #[allow(non_snake_case)]
    #[inline]
    unsafe fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        let a__b__c__d_ = accumulator;
        let c__d__b__a_ = vextq_u32(a__b__c__d_, a__b__c__d_, 2);
        let ca_db_ca_db = op_or(a__b__c__d_, c__d__b__a_);
        let db_ca_db_ca = vextq_u32(ca_db_ca_db, ca_db_ca_db, 1);
        vgetq_lane_u32(op_or(ca_db_ca_db, db_ca_db_ca), 0)
    }

    #[target_feature(enable = "neon")]
    unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        vsubq_u32(curr, vextq_u32(prev, curr, 3))
    }

    #[target_feature(enable = "neon")]
    #[allow(non_snake_case)]
    #[inline]
    unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
        let base = vdupq_laneq_u32(prev, 3);
        let zero = vmovq_n_u32(0);
        let a__b__c__d_ = delta;
        let ______a__b_ = vextq_u32(zero, a__b__c__d_, 2);
        let a__b__ca_db = vaddq_u32(______a__b_, a__b__c__d_);
        let ___a__b__ca = vextq_u32(zero, a__b__ca_db, 3);
        let a_ab_abc_abcd: DataType = vaddq_u32(___a__b__ca, a__b__ca_db);
        vaddq_u32(base, a_ab_abc_abcd)
    }

    declare_bitpacker!(target_feature(enable = "neon"));

    impl Available for UnsafeBitPackerImpl {
        fn available() -> bool {
            std::arch::is_aarch64_feature_detected!("neon")
        }
    }
}

mod scalar {

    use super::BLOCK_LEN;
    use crate::Available;
    use std::ptr;

    type DataType = [u32; 4];

    fn set1(el: i32) -> DataType {
        [el as u32; 4]
    }

    fn right_shift_32<const N: i32>(el: DataType) -> DataType {
        [el[0] >> N, el[1] >> N, el[2] >> N, el[3] >> N]
    }

    fn left_shift_32<const N: i32>(el: DataType) -> DataType {
        [el[0] << N, el[1] << N, el[2] << N, el[3] << N]
    }

    fn op_or(left: DataType, right: DataType) -> DataType {
        [
            left[0] | right[0],
            left[1] | right[1],
            left[2] | right[2],
            left[3] | right[3],
        ]
    }

    fn op_and(left: DataType, right: DataType) -> DataType {
        [
            left[0] & right[0],
            left[1] & right[1],
            left[2] & right[2],
            left[3] & right[3],
        ]
    }

    unsafe fn load_unaligned(addr: *const DataType) -> DataType {
        ptr::read_unaligned(addr)
    }

    unsafe fn store_unaligned(addr: *mut DataType, data: DataType) {
        ptr::write_unaligned(addr, data);
    }

    fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        (accumulator[0] | accumulator[1]) | (accumulator[2] | accumulator[3])
    }

    fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        [
            curr[0].wrapping_sub(prev[3]),
            curr[1].wrapping_sub(curr[0]),
            curr[2].wrapping_sub(curr[1]),
            curr[3].wrapping_sub(curr[2]),
        ]
    }

    fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
        let el0 = offset[3].wrapping_add(delta[0]);
        let el1 = el0.wrapping_add(delta[1]);
        let el2 = el1.wrapping_add(delta[2]);
        let el3 = el2.wrapping_add(delta[3]);
        [el0, el1, el2, el3]
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
    SSE3,
    #[cfg(target_arch = "aarch64")]
    NEON,
    Scalar,
}

/// `BitPacker4x` packs integers in groups of 4. This gives an opportunity
/// to leverage `SSE3` instructions to encode and decode the stream.
///
/// One block must contain `128 integers`.
#[derive(Clone, Copy)]
pub struct BitPacker4x(InstructionSet);

impl BitPacker for BitPacker4x {
    const BLOCK_LEN: usize = BLOCK_LEN;

    /// Returns the best available implementation for the current CPU.
    fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if sse3::UnsafeBitPackerImpl::available() {
                return BitPacker4x(InstructionSet::SSE3);
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            if aarch64::UnsafeBitPackerImpl::available() {
                return BitPacker4x(InstructionSet::NEON);
            }
        }
        BitPacker4x(InstructionSet::Scalar)
    }

    fn compress(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::SSE3 => {
                    sse3::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits)
                }
                #[cfg(target_arch = "aarch64")]
                InstructionSet::NEON => {
                    aarch64::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits)
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
                InstructionSet::SSE3 => sse3::UnsafeBitPackerImpl::compress_sorted(
                    initial,
                    decompressed,
                    compressed,
                    num_bits,
                ),
                #[cfg(target_arch = "aarch64")]
                InstructionSet::NEON => aarch64::UnsafeBitPackerImpl::compress_sorted(
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

    fn decompress(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::SSE3 => {
                    sse3::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
                }
                #[cfg(target_arch = "aarch64")]
                InstructionSet::NEON => {
                    aarch64::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
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
                InstructionSet::SSE3 => sse3::UnsafeBitPackerImpl::decompress_sorted(
                    initial,
                    compressed,
                    decompressed,
                    num_bits,
                ),
                #[cfg(target_arch = "aarch64")]
                InstructionSet::NEON => aarch64::UnsafeBitPackerImpl::decompress_sorted(
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

    fn num_bits(&self, decompressed: &[u32]) -> u8 {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::SSE3 => sse3::UnsafeBitPackerImpl::num_bits(decompressed),
                #[cfg(target_arch = "aarch64")]
                InstructionSet::NEON => aarch64::UnsafeBitPackerImpl::num_bits(decompressed),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::num_bits(decompressed),
            }
        }
    }

    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8 {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::SSE3 => {
                    sse3::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
                }
                #[cfg(target_arch = "aarch64")]
                InstructionSet::NEON => {
                    aarch64::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
                }
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[cfg(test)]
mod tests {
    use super::BLOCK_LEN;
    use super::{scalar, sse3};
    use crate::tests::test_util_compatible;
    use crate::Available;
    use crate::{BitPacker, BitPacker4x};

    #[test]
    fn test_compatible() {
        if sse3::UnsafeBitPackerImpl::available() {
            test_util_compatible::<scalar::UnsafeBitPackerImpl, sse3::UnsafeBitPackerImpl>(
                BLOCK_LEN,
            );
        }
    }

    #[test]
    fn test_delta_bit_width_32() {
        let values = vec![i32::max_value() as u32 + 1; BitPacker4x::BLOCK_LEN];
        let bit_packer = BitPacker4x::new();
        let bit_width = bit_packer.num_bits_sorted(0, &values);
        assert_eq!(bit_width, 32);

        let mut block = vec![0u8; BitPacker4x::compressed_block_size(bit_width)];
        bit_packer.compress_sorted(0, &values, &mut block, bit_width);

        let mut decoded_values = vec![0x10101010; BitPacker4x::BLOCK_LEN];
        bit_packer.decompress_sorted(0, &block, &mut decoded_values, bit_width);

        assert_eq!(values, decoded_values);
    }

    #[test]
    fn test_bit_width_32() {
        let mut values = vec![i32::max_value() as u32 + 1; BitPacker4x::BLOCK_LEN];
        values[0] = 0;
        let bit_packer = BitPacker4x::new();
        let bit_width = bit_packer.num_bits(&values);
        assert_eq!(bit_width, 32);

        let mut block = vec![0u8; BitPacker4x::compressed_block_size(bit_width)];
        bit_packer.compress(&values, &mut block, bit_width);

        let mut decoded_values = vec![0x10101010; BitPacker4x::BLOCK_LEN];
        bit_packer.decompress(&block, &mut decoded_values, bit_width);

        assert_eq!(values, decoded_values);
    }
}
