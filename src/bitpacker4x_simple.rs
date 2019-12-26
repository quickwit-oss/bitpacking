use super::{BitPacker, UnsafeBitPacker};

const BLOCK_LEN: usize = 32 * 4;

mod scalar {
    use super::BLOCK_LEN;
    use std::ptr;

    type DataType = [u32; 4];

    fn set1(el: i32) -> DataType {
        [el as u32; 4]
    }

    fn right_shift_32(el: DataType, shift: i32) -> DataType {
        [
            el[0] >> shift,
            el[1] >> shift,
            el[2] >> shift,
            el[3] >> shift,
        ]
    }

    fn left_shift_32(el: DataType, shift: i32) -> DataType {
        [
            el[0] << shift,
            el[1] << shift,
            el[2] << shift,
            el[3] << shift,
        ]
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

    declare_bitpacker_simple!(cfg(any(debug, not(debug))));
}

/// `BitPacker4x` packs integers in groups of 4. This gives an opportunity
/// to leverage `SSE3` instructions to encode and decode the stream.
///
/// One block must contain `128 integers`.
#[derive(Clone, Copy)]
pub struct BitPacker4x;

impl BitPacker for BitPacker4x {
    const BLOCK_LEN: usize = BLOCK_LEN;

    /// Returns the best available implementation for the current CPU.
    fn new() -> Self {
        BitPacker4x
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

    fn num_bits(&self, decompressed: &[u32]) -> u8 {
        unsafe { scalar::UnsafeBitPackerImpl::num_bits(decompressed) }
    }

    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8 {
        unsafe { scalar::UnsafeBitPackerImpl::num_bits_sorted(initial, decompressed) }
    }
}
