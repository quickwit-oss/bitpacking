/*!  # Fast Bitpacking algorithms

This crate is a **Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp)**.
It contains different flavor of integers compression via bitpacking :  `BitPacker1x`, `BitPacker4x`, and `BitPacker8x`.

Each produces different formats, and are incompatible one with another,
and requires integers to be encoded in block of different size..

`BitPacker4x` and `BitPacker8x` are designed specifically to leverage `SSE3`
and `AVX2` instructions respectively.

The library will fallback to a scalar implementation if these instruction
sets are not available. For instance :
- because your compilation target architecture is not `x86_64`
- because the CPU you use is from an older generation

I recommend using `BitPacker4x` if you are in doubt.

See the [`BitPacker` trait](./trait.BitPacker.html) for example usage.

*/

#![allow(unused_unsafe)]
#![warn(missing_docs)]

use std::marker::Sized;

#[cfg(test)]
#[macro_use]
pub(crate) mod tests;

#[macro_use]
mod macros;
#[macro_use]
mod macros_simple;

trait Available {
    fn available() -> bool;
}

trait UnsafeBitPacker {
    const BLOCK_LEN: usize;
    unsafe fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;
    unsafe fn compress_sorted(
        initial: u32,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize;
    unsafe fn compress_strictly_sorted(
        initial: Option<u32>,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize;
    unsafe fn decompress(compressed: &[u8], decompressed: &mut [u32], num_bits: u8) -> usize;
    unsafe fn decompress_sorted(
        initial: u32,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize;
    unsafe fn decompress_strictly_sorted(
        initial: Option<u32>,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize;
    unsafe fn num_bits(decompressed: &[u32]) -> u8;
    unsafe fn num_bits_sorted(initial: u32, decompressed: &[u32]) -> u8;
    unsafe fn num_bits_strictly_sorted(initial: Option<u32>, decompressed: &[u32]) -> u8;
}

/// # Examples without delta-encoding
/// ```
/// extern crate bitpacking;
///
/// use bitpacking::{BitPacker4x, BitPacker};
///
/// # fn main() {
/// # let my_data: Vec<u32> = vec![7, 7, 7, 7, 11, 10, 15, 13, 6, 5, 3, 14, 5, 7,
/// #    15, 12, 1, 10, 8, 10, 12, 14, 13, 1, 10, 1, 1, 10, 4, 15, 12,
/// #    1, 2, 0, 8, 5, 14, 5, 2, 4, 1, 6, 14, 13, 5, 10, 10, 1, 6, 4,
/// #    1, 12, 1, 1, 5, 15, 15, 2, 8, 6, 4, 3, 10, 8, 8, 9, 2, 6, 10,
/// #    5, 7, 9, 0, 13, 15, 5, 13, 10, 0, 2, 10, 14, 5, 9, 12, 8, 5, 10,
/// #    8, 8, 10, 5, 13, 8, 11, 14, 7, 14, 4, 2, 9, 12, 14, 5, 15, 12, 0,
/// #    12, 13, 3, 13, 5, 4, 15, 9, 8, 9, 3, 3, 3, 1, 12, 0, 6, 11, 11, 12, 4];
///
/// let bitpacker = BitPacker4x::new();
///
/// let num_bits: u8 = bitpacker.num_bits(&my_data);
///
/// // A block will be take at most 4 bytes per-integers.
/// let mut compressed = vec![0u8; 4 * BitPacker4x::BLOCK_LEN];
///
/// # assert_eq!(num_bits, 4);
/// let compressed_len = bitpacker.compress(&my_data, &mut compressed[..], num_bits);
///
/// assert_eq!((num_bits as usize) *  BitPacker4x::BLOCK_LEN / 8, compressed_len);
///
/// // Decompressing
/// let mut decompressed = vec![0u32; BitPacker4x::BLOCK_LEN];
/// bitpacker.decompress(&compressed[..compressed_len], &mut decompressed[..], num_bits);
///
/// assert_eq!(&my_data, &decompressed);
/// # }
/// ```
///
///
///
/// # Examples with delta-encoding
///
/// Delta-encoding makes it possible to store sorted integers in an efficient manner.
/// Rather than encoding the integers directly, the interval (or deltas) between each of them
/// are computed and then encoded.
///
/// Decoding then requires to first decode the deltas and then operate a cumulative sum (also called
/// integration or prefix sum) on them.
///
/// ```
/// extern crate bitpacking;
///
/// use bitpacking::{BitPacker4x, BitPacker};
///
/// # fn main() {
/// # let my_data: Vec<u32> = vec![0, 5, 6, 8, 12, 21, 30, 38,
/// # 46, 52, 59, 61, 62, 62, 71, 71, 73, 74, 76,
/// # 77, 80, 87, 96, 99, 105, 114, 119, 121, 128,
/// # 133, 138, 145, 152, 161, 161, 166, 175, 176,
/// # 180, 186, 189, 193, 202, 211, 220, 224, 229,
/// # 238, 247, 255, 261, 267, 267, 268, 269, 269,
/// # 270, 271, 279, 283, 285, 291, 297, 303, 305,
/// # 309, 310, 315, 316, 316, 321, 324, 329, 337,
/// # 339, 342, 350, 355, 364, 373, 382, 386, 392,
/// # 400, 408, 414, 423, 431, 433, 436, 441, 444,
/// # 445, 454, 463, 463, 465, 472, 474, 477, 480,
/// # 488, 493, 496, 501, 503, 509, 515, 519, 526,
/// # 526, 532, 539, 542, 542, 542, 549, 557, 565,
/// # 566, 573, 578, 580, 581, 585, 588, 588, 591];
///
///
/// // The initial value is used to compute the first delta.
/// // In most use cases, you will be compressing long increasing
/// // integer sequences.
/// //
/// // You should probably pass an initial value of `0u32` to the
/// // first block if you do not have any information.
/// //
/// // When encoding the second block however, you will want to pass the last
/// // value of the first block.
/// let initial_value = 0u32;
///
/// let bitpacker = BitPacker4x::new();
///
/// let num_bits: u8 = bitpacker.num_bits_sorted(initial_value, &my_data);
///
/// // A block will be take at most 4 bytes per-integers.
/// let mut compressed = vec![0u8; 4 * BitPacker4x::BLOCK_LEN];
///
/// # assert_eq!(num_bits, 4);
///
/// let compressed_len = bitpacker.compress_sorted(initial_value, &my_data, &mut compressed[..], num_bits);
///
/// assert_eq!((num_bits as usize) *  BitPacker4x::BLOCK_LEN / 8, compressed_len);
///
/// // Decompressing
/// let mut decompressed = vec![0u32; BitPacker4x::BLOCK_LEN];
///
/// // The initial value must be the same as the one passed
/// // when compressing the block.
/// bitpacker.decompress_sorted(initial_value, &compressed[..compressed_len], &mut decompressed[..], num_bits);
///
/// assert_eq!(&my_data, &decompressed);
/// # }
pub trait BitPacker: Sized + Clone + Copy {
    /// Number of `u32` per compressed block
    const BLOCK_LEN: usize;

    /// Checks the available instructions set on the current
    /// CPU and returns the best available implementation.
    ///
    /// Calling `.new()` is extremely cheap, and does not
    /// require any heap allocation. It is *not* required to cache
    /// its result too aggressively.
    fn new() -> Self;

    /// Compress a block of `u32`.
    ///
    /// Assumes that the integers are all lower than `2^num_bits`.
    /// The result is undefined if they are larger.
    ///
    /// Returns the amount of bytes of the compressed block.
    ///
    /// # Panics
    ///
    /// - Panics if the compressed destination array is too small
    /// - Panics if `decompressed` length is not exactly the `BLOCK_LEN`.
    fn compress(&self, decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize;

    /// Delta encode and compressed the `decompressed` array.
    ///
    /// Assumes that the elements in the `decompressed` array are sorted.
    /// `initial` will be used to compute the first `delta`.
    ///
    /// # Panics
    ///
    /// - Panics if `initial` is greater than `decompressed[0]`
    /// - Panics if `decompressed` is not sorted
    /// - Panics if `decompressed`'s length is not exactly `BLOCK_LEN`
    /// - Panics if `compressed` is not large enough to receive the compressed data
    /// - Panics if the compressed destination array is too small.
    ///
    /// Returns the amount of bytes of the compressed block.
    ///
    /// # Panics
    ///
    /// - Panics if the compressed array is too short.
    /// - Panics if the decompressed array is not exactly the `BLOCK_LEN`.
    fn compress_sorted(
        &self,
        initial: u32,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize;

    /// Delta encode and compress the `decompressed` array.
    ///
    /// Assumes that the elements in the `decompressed` array are strictly
    /// monotonous, that is, each element is strictly greater than the previous.
    ///
    /// This codec can be more efficient that the simply sorted compressor by up to one bit per
    /// integer. This has an important impact on saturated or nearly saturated datasets (almost
    /// every number appears in sequence), but isn't very different from the sorted compressor
    /// on more sparse datasets.
    ///
    /// # Panics
    ///
    /// - Panics if `initial` is greater or equal to `decompressed[0]`
    /// - Panics if `decompressed` isn't strictly monotonic
    /// - Panics if `decompressed`'s length is not exactly `BLOCK_LEN`
    /// - Panics if `compressed` is not large enough to receive the compressed data
    ///
    /// Returns the amount of bytes in the compressed block.
    fn compress_strictly_sorted(
        &self,
        initial: Option<u32>,
        decompressed: &[u32],
        compressed: &mut [u8],
        num_bits: u8,
    ) -> usize;

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
    /// `initial` must be the value that was passed as the `initial` argument compressing
    /// the block.
    ///
    /// Returns the amount of bytes that have been read.
    ///
    /// # Panics
    ///
    /// - Panics if the compressed array is too short to contain `BLOCK_LEN` elements
    /// - Panics if the decompressed array is too short.
    fn decompress_sorted(
        &self,
        initial: u32,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize;

    /// Decompress the`compress`array to the `decompressed` array.
    /// The `compressed` array is assumed to have been strict-delta-encoded and compressed.
    ///
    /// `initial` must be the value that was passed as the `initial` argument compressing
    /// the block.
    ///
    /// Returns the amount of bytes that have been read.
    ///
    /// # Panics
    ///
    /// - Panics if the compressed array is too short to contain `BLOCK_LEN` elements
    /// - Panics if the decompressed array is too short.

    fn decompress_strictly_sorted(
        &self,
        initial: Option<u32>,
        compressed: &[u8],
        decompressed: &mut [u32],
        num_bits: u8,
    ) -> usize;

    /// Returns the minimum number of bits used to represent the largest integer in the
    /// `decompressed` block.
    ///
    /// # Panics
    ///
    /// Panics if `decompressed`'s len is not exactly `BLOCK_LEN`.
    fn num_bits(&self, decompressed: &[u32]) -> u8;

    /// Returns the minimum number of bits used to represent the largest `delta` in the deltas in the
    /// `decompressed` block.
    ///
    /// # Panics
    ///
    /// Panics if `decompressed`'s len is not exactly `BLOCK_LEN`.
    fn num_bits_sorted(&self, initial: u32, decompressed: &[u32]) -> u8;

    /// Returns the minimum number of bits used to represent the largest `delta-1` in the deltas in the
    /// `decompressed` block.
    ///
    /// # Panics
    ///
    /// Panics if `decompressed`'s len is not exactly `BLOCK_LEN`.
    fn num_bits_strictly_sorted(&self, initial: Option<u32>, decompressed: &[u32]) -> u8;

    /// Returns the size of a compressed block.
    fn compressed_block_size(num_bits: u8) -> usize {
        Self::BLOCK_LEN * (num_bits as usize) / 8
    }
}

/// Returns the most significant bit.&self,
fn most_significant_bit(v: u32) -> u8 {
    if v == 0 {
        0
    } else {
        32u8 - (v.leading_zeros() as u8)
    }
}

#[cfg(all(feature = "bitpacker1x", any(test, not(debug_assertions))))]
mod bitpacker1x;
#[cfg(all(feature = "bitpacker4x", any(test, not(debug_assertions))))]
mod bitpacker4x;

#[cfg(all(feature = "bitpacker1x", debug_assertions))]
mod bitpacker1x_simple;
#[cfg(all(feature = "bitpacker4x", debug_assertions))]
mod bitpacker4x_simple;

#[cfg(feature = "bitpacker8x")]
mod bitpacker8x;

#[cfg(all(feature = "bitpacker1x", not(debug_assertions)))]
pub use bitpacker1x::BitPacker1x;
#[cfg(all(feature = "bitpacker1x", debug_assertions))]
pub use bitpacker1x_simple::BitPacker1x;

#[cfg(all(feature = "bitpacker4x", not(debug_assertions)))]
pub use bitpacker4x::BitPacker4x;
#[cfg(all(feature = "bitpacker4x", debug_assertions))]
pub use bitpacker4x_simple::BitPacker4x;

#[cfg(feature = "bitpacker8x")]
pub use bitpacker8x::BitPacker8x;

#[cfg(test)]
mod tests_unit {
    use super::*;

    #[test]
    #[should_panic(expected = "`decompressed`'s len is not `BLOCK_LEN=128`")]
    fn test_num_bits_block_too_long() {
        let bit_packer = BitPacker4x::new();
        let v = vec![0u32; BitPacker4x::BLOCK_LEN + 1];
        bit_packer.num_bits(&v[..]);
    }

    #[test]
    #[should_panic(expected = "`decompressed`'s len is not `BLOCK_LEN=128`")]
    fn test_num_bits_block_too_short() {
        let bit_packer = BitPacker4x::new();
        let v = vec![0u32; BitPacker4x::BLOCK_LEN - 1];
        bit_packer.num_bits(&v[..]);
    }
}

#[cfg(test)]
mod functional_tests {
    use crate::{BitPacker, BitPacker4x};
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[ignore]
        fn check_block(
            values in prop::collection::vec(u32::arbitrary(), BitPacker4x::BLOCK_LEN),
        ) {
            let bit_packer = BitPacker4x::new();
            let bit_width = bit_packer.num_bits(&*values);

            let mut block = vec![0u8; BitPacker4x::compressed_block_size(bit_width)];
            bit_packer.compress(&values, &mut block, bit_width);

            let mut decoded_values = vec![0x10101010; BitPacker4x::BLOCK_LEN];
            bit_packer.decompress(&block, &mut decoded_values, bit_width);

            prop_assert_eq!(values, decoded_values);
        }

        #[ignore]
        #[test]
        fn check_sorted_block(
            (values, init_value) in prop::collection::vec(u32::arbitrary(), BitPacker4x::BLOCK_LEN).prop_flat_map(|mut values| {
                values.sort();
                let min_value = values[0];
                (Just(values), (0..=min_value))
            }),
        ) {
            let bit_packer = BitPacker4x::new();
            let bit_width = bit_packer.num_bits_sorted(init_value, &*values);

            let mut block = vec![0u8; BitPacker4x::compressed_block_size(bit_width)];
            bit_packer.compress_sorted(init_value, &values, &mut block, bit_width);

            let mut decoded_values = vec![0x10101010; BitPacker4x::BLOCK_LEN];
            bit_packer.decompress_sorted(init_value, &block, &mut decoded_values, bit_width);

            prop_assert_eq!(values, decoded_values);
        }
    }
}
