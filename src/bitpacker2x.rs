use super::{BitPacker, UnsafeBitPacker};

const BLOCK_LEN: usize = 64;

fn compute_mask(num_bits: u64) -> u64  {
    if num_bits == 0 {
        return 0u64;
    }
    let mask = (1u64 << num_bits) - 1u64;
    mask | (mask << 32)
}


fn conv(val: u64) -> [u32; 2] {
    unsafe { std::mem::transmute(val) }
}

fn vonc(val: [u32;2]) -> u64 {
    unsafe { std::mem::transmute(val) }
}

mod scalar {
    use std::ptr::read_unaligned as load_unaligned;
    use std::ptr::write_unaligned as store_unaligned;

    use super::BLOCK_LEN;
    use Available;
    use bitpacker2x::compute_mask;

    type DataType = u64;

    fn set1(el: i32) -> DataType {
        let el = el as u64;
        el | el << 32
    }

    fn right_shift_32(el: DataType, shift: i32) -> DataType {
        let shift = shift as u64;
        let mask = compute_mask(shift);
        (el & !mask) >> shift
    }

    fn left_shift_32(el: DataType, shift2: i32) -> DataType {
        let shift = shift2 as u64;
        let mask = compute_mask(32-shift);
        (el & mask) << shift
    }

    fn op_or(left: DataType, right: DataType) -> DataType {
        left | right
    }

    fn op_and(left: DataType, right: DataType) -> DataType {
        left & right
    }

    fn or_collapse_to_u32(accumulator: DataType) -> u32 {
        let high = accumulator >> 32u64;
        let low = accumulator % (1u64 << 32);
        (high | low) as u32
    }

    fn compute_delta(curr: DataType, prev: DataType) -> DataType {
        unimplemented!();//    curr.wrapping_sub(prev)
    }

    fn integrate_delta(offset: DataType, delta: DataType) -> DataType {
        unimplemented!();
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
pub struct BitPacker2x;

impl BitPacker for BitPacker2x {
    const BLOCK_LEN: usize = BLOCK_LEN;

    fn new() -> BitPacker2x {
        BitPacker2x
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
