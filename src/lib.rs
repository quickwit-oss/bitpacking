#![allow(unused_unsafe)]
#![feature(stdsimd)]
#![feature(test)]

#[macro_use]
extern crate crunchy;

#[cfg(test)]
#[macro_use]
pub(crate) mod tests;

#[macro_use]
mod macros;

mod scalar;
mod sse3;
//mod avx2;

pub use scalar::ScalarBitPacker;
pub use sse3::SSE3BitPacker;
//pub use avx2::AVX2BitPacker;

pub trait BitPacker {

    // Integers are compressed in pack of `BLOCK_LEN` `u32`-integers.
    //
    // `BLOCK_LEN` is required to be a power of 2, greater than 8.
    // A high `BLOCK_LEN` may negatively impact the compression rate.
    //
    // Indeed all integers of a given block will take as many bits as the
    // most significant bit of the largest integer.
    const BLOCK_LEN: usize;

    type DataType;

    fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8);

    fn compress_delta(initial: u32,
                      decompressed: &[u32],
                      compressed: &mut [u8],
                      num_bits: u8);

    fn decompress_to<Output: FnMut(Self::DataType)>(compressed: &[u8], output: Output, num_bits: u8);

    fn decompress(compressed: &[u8], decompressed: &mut [u32], num_bits: u8);

    fn decompress_delta(initial: u32,
                        compressed: &[u8],
                        decompressed: &mut [u32],
                        num_bits: u8);

    fn num_bits(decompressed: &[u32]) -> u8;

    fn num_bits_delta(initial: u32, decompressed: &[u32]) -> u8;
}

/// Returns the most significant bit.
fn most_significant_bit(v: u32) -> u8 {
    if v == 0 {
        0
    } else {
        32u8 - (v.leading_zeros() as u8)
    }
}
