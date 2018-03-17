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


///
/// ```
/// extern crate bitpacking;
///
/// use bitpacking::{SSE3BitPacker, BitPacker};
///
/// # fn main() {
/// let fake_data = vec![7, 7, 7, 7, 11, 10, 15, 13, 6, 5, 3, 14, 5, 7,
///     15, 12, 1, 10, 8, 10, 12, 14, 13, 1, 10, 1, 1, 10, 4, 15, 12,
///     1, 2, 0, 8, 5, 14, 5, 2, 4, 1, 6, 14, 13, 5, 10, 10, 1, 6, 4,
///     1, 12, 1, 1, 5, 15, 15, 2, 8, 6, 4, 3, 10, 8, 8, 9, 2, 6, 10,
///     5, 7, 9, 0, 13, 15, 5, 13, 10, 0, 2, 10, 14, 5, 9, 12, 8, 5, 10,
///     8, 8, 10, 5, 13, 8, 11, 14, 7, 14, 4, 2, 9, 12, 14, 5, 15, 12, 0,
///     12, 13, 3, 13, 5, 4, 15, 9, 8, 9, 3, 3, 3, 1, 12, 0, 6, 11, 11, 12, 4];
/// let num_bits: u8 = SSE3BitPacker::num_bits(&fake_data);
/// assert_eq!(num_bits, 4);
/// let mut compressed = vec![0u8; (num_bits as usize) * SSE3BitPacker::BLOCK_LEN / 8];
/// SSE3BitPacker::compress(&fake_data, &mut compressed[..], num_bits);
///
/// // Decompressing
/// let mut decompressed = vec![0u32; SSE3BitPacker::BLOCK_LEN];
/// SSE3BitPacker::decompress(&compressed, &mut decompressed[..], num_bits);
///
/// assert_eq!(&fake_data, &decompressed);
/// # }
/// ```
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
