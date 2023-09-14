use super::{BitPacker, UnsafeBitPacker};

#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_endian = "little")
))]
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

    #[target_feature(enable = "sse3")]
    #[inline]
    unsafe fn add(left: DataType, right: DataType) -> DataType {
        _mm_add_epi32(left, right)
    }

    unsafe fn sub(left: DataType, right: DataType) -> DataType {
        _mm_sub_epi32(left, right)
    }

    declare_bitpacker!(target_feature(enable = "sse3"));

    impl Available for UnsafeBitPackerImpl {
        fn available() -> bool {
            is_x86_feature_detected!("sse3")
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_endian = "little"))]
mod neon {

    use super::BLOCK_LEN;
    use crate::Available;

    use super::scalar::add;
    use super::scalar::left_shift_32;
    use super::scalar::load_unaligned;
    use super::scalar::op_and;
    use super::scalar::op_or;
    use super::scalar::or_collapse_to_u32;
    use super::scalar::right_shift_32;
    use super::scalar::set1;
    use super::scalar::store_unaligned;
    use super::scalar::sub;
    use super::scalar::DataType;
    use std::arch::aarch64::{vaddq_u32, vdupq_n_u32, vextq_u32, vld1q_u32, vst1q_u32, vsubq_u32};

    #[target_feature(enable = "neon")]
    unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        let c = vld1q_u32(curr.as_ptr());
        let p = vld1q_u32(prev.as_ptr());
        let mut r = set1(0);
        vst1q_u32(r.as_mut_ptr(), vsubq_u32(c, vextq_u32(p, c, 3)));
        r
    }

    #[target_feature(enable = "neon")]
    #[allow(non_snake_case)]
    #[inline]
    unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
        let base = vdupq_n_u32(prev[3]);
        let zero = vdupq_n_u32(0);
        let a__b__c__d_ = vld1q_u32(delta.as_ptr());
        let ______a__b_ = vextq_u32(zero, a__b__c__d_, 2);
        let a__b__ca_db = vaddq_u32(______a__b_, a__b__c__d_);
        let ___a__b__ca = vextq_u32(zero, a__b__ca_db, 3);
        let a_ab_abc_abcd = vaddq_u32(___a__b__ca, a__b__ca_db);
        let mut r = set1(0);
        vst1q_u32(r.as_mut_ptr(), vaddq_u32(base, a_ab_abc_abcd));
        r
    }

    // TODO trinity-1686a: I believe add/sub are easy enough for the compiler to optimize on its
    // own, and suspect hand-rolled impl would force (un)loading registers and make things slower
    // overall

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

    pub(crate) type DataType = [u32; 4];

    pub(crate) fn set1(el: i32) -> DataType {
        [el as u32; 4]
    }

    pub(crate) fn right_shift_32<const N: i32>(el: DataType) -> DataType {
        [el[0] >> N, el[1] >> N, el[2] >> N, el[3] >> N]
    }

    pub(crate) fn left_shift_32<const N: i32>(el: DataType) -> DataType {
        [el[0] << N, el[1] << N, el[2] << N, el[3] << N]
    }

    pub(crate) fn op_or(left: DataType, right: DataType) -> DataType {
        [
            left[0] | right[0],
            left[1] | right[1],
            left[2] | right[2],
            left[3] | right[3],
        ]
    }

    pub(crate) fn op_and(left: DataType, right: DataType) -> DataType {
        [
            left[0] & right[0],
            left[1] & right[1],
            left[2] & right[2],
            left[3] & right[3],
        ]
    }

    pub(crate) unsafe fn load_unaligned(addr: *const DataType) -> DataType {
        ptr::read_unaligned(addr)
    }

    pub(crate) unsafe fn store_unaligned(addr: *mut DataType, data: DataType) {
        ptr::write_unaligned(addr, data);
    }

    pub(crate) fn or_collapse_to_u32(accumulator: DataType) -> u32 {
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

    pub(crate) fn add(left: DataType, right: DataType) -> DataType {
        [
            left[0].wrapping_add(right[0]),
            left[1].wrapping_add(right[1]),
            left[2].wrapping_add(right[2]),
            left[3].wrapping_add(right[3]),
        ]
    }

    pub(crate) fn sub(left: DataType, right: DataType) -> DataType {
        [
            left[0].wrapping_sub(right[0]),
            left[1].wrapping_sub(right[1]),
            left[2].wrapping_sub(right[2]),
            left[3].wrapping_sub(right[3]),
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
        #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
        {
            if neon::UnsafeBitPackerImpl::available() {
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
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => {
                    neon::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits)
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
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => neon::UnsafeBitPackerImpl::compress_sorted(
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
                InstructionSet::SSE3 => sse3::UnsafeBitPackerImpl::compress_strictly_sorted(
                    initial,
                    decompressed,
                    compressed,
                    num_bits,
                ),
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => neon::UnsafeBitPackerImpl::compress_strictly_sorted(
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
                InstructionSet::SSE3 => {
                    sse3::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
                }
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => {
                    neon::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
                }
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
                InstructionSet::SSE3 => sse3::UnsafeBitPackerImpl::decompress_strictly_sorted(
                    initial,
                    compressed,
                    decompressed,
                    num_bits,
                ),
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => neon::UnsafeBitPackerImpl::decompress_strictly_sorted(
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
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => neon::UnsafeBitPackerImpl::decompress_sorted(
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
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => neon::UnsafeBitPackerImpl::num_bits(decompressed),
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
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => {
                    neon::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
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
                InstructionSet::SSE3 => {
                    sse3::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed)
                }
                #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
                InstructionSet::NEON => {
                    neon::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed)
                }
                InstructionSet::Scalar => {
                    scalar::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed)
                }
            }
        }
    }
}

#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_endian = "little")
))]
#[cfg(test)]
mod tests {
    use super::scalar;
    use super::BLOCK_LEN;
    use crate::tests::test_util_compatible;
    use crate::Available;
    use crate::{BitPacker, BitPacker4x};

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_compatible_sse3() {
        use super::sse3;
        if sse3::UnsafeBitPackerImpl::available() {
            test_util_compatible::<scalar::UnsafeBitPackerImpl, sse3::UnsafeBitPackerImpl>(
                BLOCK_LEN,
            );
        }
    }

    #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
    #[test]
    fn test_compatible_neon() {
        use super::neon;
        if neon::UnsafeBitPackerImpl::available() {
            test_util_compatible::<scalar::UnsafeBitPackerImpl, neon::UnsafeBitPackerImpl>(
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
