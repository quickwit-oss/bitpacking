#![feature(stdsimd)]
#![feature(test)]

#[macro_use]
extern crate crunchy;

#[cfg(test)]
#[macro_use]
pub(crate) mod tests;


mod avxpacking;
mod simdcomp;

pub use avxpacking::AVXBitPacker;
pub use simdcomp::SIMDBitPacker;


pub trait BitPacker {

    // Integers are compressed in pack of `BLOCK_LEN` `u32`-integers.
    //
    // `BLOCK_LEN` is required to be a power of 2, greater than 8.
    // A high `BLOCK_LEN` may negatively impact the compression rate.
    //
    // Indeed all integers of a given block will take as many bits as the
    // most significant bit of the largest integer.
    const BLOCK_LEN: usize;

    fn compress(uncompressed: &[u32], compressed: &mut [u8], num_bits: u8);

    fn uncompress(compressed: &[u8], uncompressed: &mut [u32], num_bits: u8);

    fn num_bits(uncompressed: &[u32]) -> u8;
}



/// Returns the most significant bit.
fn most_significant_bit(v: u32) -> u8 {
    if v == 0 {
        0
    } else {
        32u8 - (v.leading_zeros() as u8)
    }
}


//
//#[cfg(test)]
//pub(crate) mod tests {
//
//
//}


