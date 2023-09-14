use super::{BitPacker, UnsafeBitPacker};

const BLOCK_LEN: usize = 32;

mod scalar {
    use std::ptr::read_unaligned as load_unaligned;
    use std::ptr::write_unaligned as store_unaligned;

    use super::BLOCK_LEN;
    use crate::Available;

    type DataType = u32;

    fn set1(el: i32) -> DataType {
        el as u32
    }

    fn right_shift_32<const N: i32>(el: DataType) -> DataType {
        el >> N
    }

    fn left_shift_32<const N: i32>(el: DataType) -> DataType {
        el << N
    }

    fn op_or(left: DataType, right: DataType) -> DataType {
        left | right
    }

    fn op_and(left: DataType, right: DataType) -> DataType {
        left & right
    }

    fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        accumulator
    }

    fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        curr.wrapping_sub(prev)
    }

    fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
        offset.wrapping_add(delta)
    }

    fn add(left: DataType, right: DataType) -> DataType {
        left.wrapping_add(right)
    }

    fn sub(left: DataType, right: DataType) -> DataType {
        left.wrapping_sub(right)
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

/// `BitPacker1x` is standard bitpacking : the integer representation over
/// `b` bits are simply concatenated one after the other.
///
/// One block must contain `32 integers`.
#[derive(Clone, Copy)]
pub struct BitPacker1x;

impl BitPacker for BitPacker1x {
    const BLOCK_LEN: usize = BLOCK_LEN;

    fn new() -> BitPacker1x {
        BitPacker1x
    }

    fn compress(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
        unsafe { scalar::UnsafeBitPackerImpl::compress(decompressed, compressed, num_bits) }
    }

    fn compress_sorted(
        &self,
        initial: u32,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize {
        unsafe {
            scalar::UnsafeBitPackerImpl::compress_sorted(
                initial,
                decompressed,
                compressed,
                num_bits,
            )
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
            scalar::UnsafeBitPackerImpl::compress_strictly_sorted(
                initial,
                decompressed,
                compressed,
                num_bits,
            )
        }
    }

    fn decompress(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize {
        unsafe { scalar::UnsafeBitPackerImpl::decompress(compressed, decompressed, num_bits) }
    }

    fn decompress_sorted(
        &self,
        initial: u32,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize {
        unsafe {
            scalar::UnsafeBitPackerImpl::decompress_sorted(
                initial,
                compressed,
                decompressed,
                num_bits,
            )
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
            scalar::UnsafeBitPackerImpl::decompress_strictly_sorted(
                initial,
                compressed,
                decompressed,
                num_bits,
            )
        }
    }

    fn num_bits(&self, decompressed: &[u32]) -> u8 {
        unsafe { scalar::UnsafeBitPackerImpl::num_bits(decompressed) }
    }

    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8 {
        unsafe { scalar::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed) }
    }

    fn num_bits_strictly_sorted(&self, initial: Option<u32>, decompressed: &[u32]) -> u8 {
        unsafe { scalar::UnsafeBitPackerImpl::num_bits_strictly_sorted(initial, decompressed) }
    }
}
