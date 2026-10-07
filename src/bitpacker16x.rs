use super::{BitPacker, UnsafeBitPacker};

#[cfg(target_arch = "x86_64")]
use crate::Available;

const BLOCK_LEN: usize = 32 * 16;

#[cfg(target_arch = "x86_64")]
mod avx512 {

    use super::BLOCK_LEN;
    use crate::Available;

    use std::arch::x86_64::__m512i as DataType;
    use std::arch::x86_64::_mm512_and_si512 as op_and;
    use std::arch::x86_64::_mm512_or_si512 as op_or;
    use std::arch::x86_64::_mm512_set1_epi32 as set1;

    use std::arch::x86_64::{
        _mm512_add_epi32, _mm512_alignr_epi32, _mm512_castsi512_si128, _mm512_loadu_si512,
        _mm512_permutexvar_epi32, _mm512_setzero_si512, _mm512_sll_epi32, _mm512_srl_epi32,
        _mm512_storeu_si512, _mm512_sub_epi32, _mm_cvtsi128_si32, _mm_cvtsi32_si128,
    };

    // Variable-count shift: the immediate `_mm512_slli_epi32` wants `const u32`
    // but the macro passes `const i32`; with `N` constant this folds to an immediate.
    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn left_shift_32<const N: i32>(el: DataType) -> DataType {
        _mm512_sll_epi32(el, _mm_cvtsi32_si128(N))
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn right_shift_32<const N: i32>(el: DataType) -> DataType {
        _mm512_srl_epi32(el, _mm_cvtsi32_si128(N))
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn load_unaligned(addr: *const DataType) -> DataType {
        _mm512_loadu_si512(addr.cast())
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn store_unaligned(addr: *mut DataType, data: DataType) {
        _mm512_storeu_si512(addr.cast(), data);
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        // OR-rotate the whole register four times so lane 0 holds all 16 lanes.
        let mut v = accumulator;
        v = op_or(v, _mm512_alignr_epi32::<8>(v, v));
        v = op_or(v, _mm512_alignr_epi32::<4>(v, v));
        v = op_or(v, _mm512_alignr_epi32::<2>(v, v));
        v = op_or(v, _mm512_alignr_epi32::<1>(v, v));
        _mm_cvtsi128_si32(_mm512_castsi512_si128(v)) as u32
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        // builds {prev[15], curr[0], .., curr[14]}
        let prev_shifted = _mm512_alignr_epi32::<15>(curr, prev);
        _mm512_sub_epi32(curr, prev_shifted)
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn integrate_delta(prev: DataType, delta: DataType) -> DataType {
        // Prefix-sum the lanes, then add the running offset prev[15].
        // `alignr(x, zero, 16 - s)` shifts lanes right by `s` (so 15/14/12/8 = 1/2/4/8).
        let zero = _mm512_setzero_si512();
        let offset = _mm512_permutexvar_epi32(set1(15), prev);
        let mut x = delta;
        x = _mm512_add_epi32(x, _mm512_alignr_epi32::<15>(x, zero));
        x = _mm512_add_epi32(x, _mm512_alignr_epi32::<14>(x, zero));
        x = _mm512_add_epi32(x, _mm512_alignr_epi32::<12>(x, zero));
        x = _mm512_add_epi32(x, _mm512_alignr_epi32::<8>(x, zero));
        _mm512_add_epi32(x, offset)
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn add(left: DataType, right: DataType) -> DataType {
        _mm512_add_epi32(left, right)
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn sub(left: DataType, right: DataType) -> DataType {
        _mm512_sub_epi32(left, right)
    }

    declare_bitpacker!(target_feature(enable = "avx512f"));

    impl Available for UnsafeBitPackerImpl {
        fn available() -> bool {
            is_x86_feature_detected!("avx512f")
        }
    }
}

mod scalar {

    use super::BLOCK_LEN;
    use crate::Available;
    use std::ptr;

    type DataType = [u32; 16];

    fn set1(el: i32) -> DataType {
        [el as u32; 16]
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
            el[8] >> N,
            el[9] >> N,
            el[10] >> N,
            el[11] >> N,
            el[12] >> N,
            el[13] >> N,
            el[14] >> N,
            el[15] >> N,
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
            el[8] << N,
            el[9] << N,
            el[10] << N,
            el[11] << N,
            el[12] << N,
            el[13] << N,
            el[14] << N,
            el[15] << N,
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
            left[8] | right[8],
            left[9] | right[9],
            left[10] | right[10],
            left[11] | right[11],
            left[12] | right[12],
            left[13] | right[13],
            left[14] | right[14],
            left[15] | right[15],
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
            left[8] & right[8],
            left[9] & right[9],
            left[10] & right[10],
            left[11] & right[11],
            left[12] & right[12],
            left[13] & right[13],
            left[14] & right[14],
            left[15] & right[15],
        ]
    }

    unsafe fn load_unaligned(addr: *const DataType) -> DataType {
        ptr::read_unaligned(addr)
    }

    unsafe fn store_unaligned(dst: *mut DataType, data: DataType) {
        ptr::write_unaligned(dst, data);
    }

    fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        (((accumulator[0] | accumulator[1]) | (accumulator[2] | accumulator[3]))
            | ((accumulator[4] | accumulator[5]) | (accumulator[6] | accumulator[7])))
            | (((accumulator[8] | accumulator[9]) | (accumulator[10] | accumulator[11]))
                | ((accumulator[12] | accumulator[13]) | (accumulator[14] | accumulator[15])))
    }

    fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        [
            curr[0].wrapping_sub(prev[15]),
            curr[1].wrapping_sub(curr[0]),
            curr[2].wrapping_sub(curr[1]),
            curr[3].wrapping_sub(curr[2]),
            curr[4].wrapping_sub(curr[3]),
            curr[5].wrapping_sub(curr[4]),
            curr[6].wrapping_sub(curr[5]),
            curr[7].wrapping_sub(curr[6]),
            curr[8].wrapping_sub(curr[7]),
            curr[9].wrapping_sub(curr[8]),
            curr[10].wrapping_sub(curr[9]),
            curr[11].wrapping_sub(curr[10]),
            curr[12].wrapping_sub(curr[11]),
            curr[13].wrapping_sub(curr[12]),
            curr[14].wrapping_sub(curr[13]),
            curr[15].wrapping_sub(curr[14]),
        ]
    }

    fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
        let el0 = offset[15].wrapping_add(delta[0]);
        let el1 = el0.wrapping_add(delta[1]);
        let el2 = el1.wrapping_add(delta[2]);
        let el3 = el2.wrapping_add(delta[3]);
        let el4 = el3.wrapping_add(delta[4]);
        let el5 = el4.wrapping_add(delta[5]);
        let el6 = el5.wrapping_add(delta[6]);
        let el7 = el6.wrapping_add(delta[7]);
        let el8 = el7.wrapping_add(delta[8]);
        let el9 = el8.wrapping_add(delta[9]);
        let el10 = el9.wrapping_add(delta[10]);
        let el11 = el10.wrapping_add(delta[11]);
        let el12 = el11.wrapping_add(delta[12]);
        let el13 = el12.wrapping_add(delta[13]);
        let el14 = el13.wrapping_add(delta[14]);
        let el15 = el14.wrapping_add(delta[15]);
        [
            el0, el1, el2, el3, el4, el5, el6, el7, el8, el9, el10, el11, el12, el13, el14, el15,
        ]
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
            left[8].wrapping_add(right[8]),
            left[9].wrapping_add(right[9]),
            left[10].wrapping_add(right[10]),
            left[11].wrapping_add(right[11]),
            left[12].wrapping_add(right[12]),
            left[13].wrapping_add(right[13]),
            left[14].wrapping_add(right[14]),
            left[15].wrapping_add(right[15]),
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
            left[8].wrapping_sub(right[8]),
            left[9].wrapping_sub(right[9]),
            left[10].wrapping_sub(right[10]),
            left[11].wrapping_sub(right[11]),
            left[12].wrapping_sub(right[12]),
            left[13].wrapping_sub(right[13]),
            left[14].wrapping_sub(right[14]),
            left[15].wrapping_sub(right[15]),
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
    AVX512,
    Scalar,
}

/// `BitPacker16x` packs integers in groups of 16. This gives an opportunity
/// to leverage `AVX-512` instructions to encode and decode the stream.
/// One block must contain `512 integers`.
#[derive(Clone, Copy)]
pub struct BitPacker16x(InstructionSet);

impl BitPacker for BitPacker16x {
    const BLOCK_LEN: usize = BLOCK_LEN;

    fn new() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if avx512::UnsafeBitPackerImpl::available() {
                return BitPacker16x(InstructionSet::AVX512);
            }
        }
        BitPacker16x(InstructionSet::Scalar)
    }

    fn compress(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX512 => {
                    avx512::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits)
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
                InstructionSet::AVX512 => avx512::UnsafeBitPackerImpl::compress_sorted(
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
                InstructionSet::AVX512 => avx512::UnsafeBitPackerImpl::compress_strictly_sorted(
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
                InstructionSet::AVX512 => {
                    avx512::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits)
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
                InstructionSet::AVX512 => avx512::UnsafeBitPackerImpl::decompress_sorted(
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
                InstructionSet::AVX512 => avx512::UnsafeBitPackerImpl::decompress_strictly_sorted(
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
                InstructionSet::AVX512 => avx512::UnsafeBitPackerImpl::num_bits(decompressed),
                InstructionSet::Scalar => scalar::UnsafeBitPackerImpl::num_bits(decompressed),
            }
        }
    }

    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8 {
        unsafe {
            match self.0 {
                #[cfg(target_arch = "x86_64")]
                InstructionSet::AVX512 => {
                    avx512::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed)
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
                InstructionSet::AVX512 => {
                    avx512::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed)
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
    use super::{avx512, scalar};
    use crate::tests::test_util_compatible;
    use crate::Available;

    #[test]
    fn test_compatible() {
        if avx512::UnsafeBitPackerImpl::available() {
            test_util_compatible::<scalar::UnsafeBitPackerImpl, avx512::UnsafeBitPackerImpl>(
                BLOCK_LEN,
            );
        }
    }
}
