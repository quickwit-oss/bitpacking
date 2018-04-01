/*!  # Fast Bitpacking algorithms

This crate is a **Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp)**.
It contains different implementation of integers compression via bitpacking.
Each implementation requires on a different CPU SIMD instruction set,
for state of the art performance.

This crate is used by [`tantivy`](https://github.com/tantivy-search/tantivy).

For instance, with `SSE3`, you can typically expect more than 4 billions ints per seconds.
Check the [Benchmark](#benchmark) for more accurate details.

# Usage

This crate contains different implementation for bitpacking depending on the instruction set available with your processor.
**The resulting format are not compatible one with each other.**

Currently the following are available :
- scalar: implementation not using any SIMD instruction. A block has a size of 32.
This implementation is still more performant naive solutions.
- SSE3: bitpacking 4 integer at once. (block size of 128). *Requires the sse3 feature to be enabled. This feature is enabled by default.*
- AVX: butpacking 8 integers at once. (block size of 256). *Delta integration is comparatively expensive*. Requires to
enable the avx feature.

I recommend using the SSE3 implementation if you are not sure what you are doing and you are targetting x86_64 CPUs that have been produced after 2006.

Make sure to compile with

RUSTFLAGS="-C target-cpu=native"


# Examples without delta-encoding
```
extern crate bitpacking;

use bitpacking::{SSE3BitPacker, BitPacker};

# fn main() {
# let my_data = vec![7, 7, 7, 7, 11, 10, 15, 13, 6, 5, 3, 14, 5, 7,
#    15, 12, 1, 10, 8, 10, 12, 14, 13, 1, 10, 1, 1, 10, 4, 15, 12,
#    1, 2, 0, 8, 5, 14, 5, 2, 4, 1, 6, 14, 13, 5, 10, 10, 1, 6, 4,
#    1, 12, 1, 1, 5, 15, 15, 2, 8, 6, 4, 3, 10, 8, 8, 9, 2, 6, 10,
#    5, 7, 9, 0, 13, 15, 5, 13, 10, 0, 2, 10, 14, 5, 9, 12, 8, 5, 10,
#    8, 8, 10, 5, 13, 8, 11, 14, 7, 14, 4, 2, 9, 12, 14, 5, 15, 12, 0,
#    12, 13, 3, 13, 5, 4, 15, 9, 8, 9, 3, 3, 3, 1, 12, 0, 6, 11, 11, 12, 4];
let num_bits: u8 = SSE3BitPacker::num_bits(&my_data);

// A block will be take at most 4 bytes per-integers.
let mut compressed = vec![0u8; 4 * SSE3BitPacker::BLOCK_LEN];

# assert_eq!(num_bits, 4);
let compressed_len = SSE3BitPacker::compress(&my_data, &mut compressed[..], num_bits);

assert_eq!((num_bits as usize) *  SSE3BitPacker::BLOCK_LEN / 8, compressed_len);

// Decompressing
let mut decompressed = vec![0u32; SSE3BitPacker::BLOCK_LEN];
SSE3BitPacker::decompress(&compressed[..compressed_len], &mut decompressed[..], num_bits);

assert_eq!(&my_data, &decompressed);

# }
```


# Examples with delta-encoding
```
extern crate bitpacking;

use bitpacking::{SSE3BitPacker, BitPacker};

# fn main() {
# let sorted_array = (17..145).collect();
let num_bits: u8 = SSE3BitPacker::num_bits_sorted(16u32, &my_data);

// A block will be take at most 4 bytes per-integers.
let mut compressed = vec![0u8; 4 * SSE3BitPacker::BLOCK_LEN];

# assert_eq!(num_bits, 4);
let compressed_len = SSE3BitPacker::compress_sorted(&sorted_array, &mut compressed[..], num_bits);

assert_eq!((num_bits as usize) *  SSE3BitPacker::BLOCK_LEN / 8, compressed_len);

// Decompressing
let mut decompressed = vec![0u32; SSE3BitPacker::BLOCK_LEN];
SSE3BitPacker::decompress(17, &compressed[..compressed_len], &mut decompressed[..], num_bits);

assert_eq!(&sorted_array, &decompressed);

# }
```


*/


#![allow(unused_unsafe)]
#![feature(stdsimd)]
#![feature(test)]
#![feature(target_feature)]

#[macro_use]
extern crate crunchy;

#[cfg(test)]
#[macro_use]
pub(crate) mod tests;

#[macro_use]
mod macros;


mod scalar;
pub use scalar::ScalarBitPacker;

#[cfg(feature = "sse3")]
mod sse3;
#[cfg(feature = "sse3")]
pub use sse3::SSE3BitPacker;

#[cfg(feature = "avx2")]
mod avx2;
#[cfg(feature = "avx2")]
pub use avx2::AVX2BitPacker;

use std::marker::Sized;

pub trait BitPacker: Sized {

    /// Integers are compressed in pack of `BLOCK_LEN` `u32`-integers.
    ///
    /// `BLOCK_LEN` is required to be a power of 2, greater than 8.
    /// A high `BLOCK_LEN` may negatively impact the compression rate.
    ///
    /// Indeed all integers of a given block will take as many bits as the
    /// most significant bit of the largest integer.
    const BLOCK_LEN: usize;

    /// Type of the register used by the `BitPacker`.
    type DataType;

    fn acquire() -> Option<Self>;

    /// Compress a block of `u32`
    ///
    /// Assumes that the integers are all lower than `2^num_bits`.
    /// The result is undefined if they are larger.
    ///
    /// Returns the amount of bytes of the compressed block.
    ///
    /// # Panics
    ///
    /// Panics if the compressed destination array is assumed to be large enough.
    /// Panics if `decompressed` length is not exactly the `BLOCK_LEN`.
    fn compress(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;

    /// Delta encode and compressed the `decompressed` array.
    ///
    /// Assumes that the `decompressed` array is sorted.
    /// The first element will assume the previous element is `initial`.
    ///
    /// Returns the amount of bytes of the compressed block.
    ///
    /// # Panics
    ///
    /// Panics if the compressed array is too short.
    /// Panics if the decompressed array is not exactly the `BLOCK_LEN`.
    fn compress_sorted(&self, initial: u32,
                       decompressed: &[u32],
                       compressed: &mut [u8],
                       num_bits: u8) -> usize;

    /// Decompresses the `compressed` array and streams registers full of `u32`
    /// to the output functions.
    //unsafe fn decompress_to<Output: Sink<Self::DataType> >(&self, compressed: &[u8], output: Output, num_bits: u8) -> usize;

    /// Decompress the `compress` array to the `decompressed` array.
    ///
    /// Returns the amount of bytes that were consumed.
    ///
    /// # Panics
    ///
    /// Panics if the compressed array is too short, or the decompressed array is too short.
    fn decompress(&self, compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize;


    /// Decompress the`compress`array to the `decompressed` array.
    /// The `compressed` array is assumed to have been delta-encoded and compressed.
    ///
    /// `initial` contains the previous used to delta-decode the first element.
    ///
    /// Returns the amount of bytes that were consumed.
    ///
    /// # Panics
    ///
    /// Panics if the compressed array is too short, or the decompressed array is too short.
    fn decompress_sorted(&self, initial: u32,
                         compressed: &[u8],
                         decompressed: &mut [u32],
                         num_bits: u8) -> usize;

    /// Returns the minimum number of bits used to represent all integers in the
    /// `decompressed` array.
    fn num_bits(&self, decompressed: &[u32]) -> u8;

    /// Returns the minimum number of bits used to represent all the deltas in the
    /// `decompressed` array.
    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8;

    /// Returns the size of a compressed block.
    fn compressed_block_size(&self, num_bits: u8) -> usize {
        Self::BLOCK_LEN * ( num_bits as usize) / 8
    }
}

/// Returns the most significant bit.
fn most_significant_bit(v: u32) -> u8 {
    if v == 0 {
        0
    } else {
        32u8 - (v.leading_zeros() as u8)
    }
}
