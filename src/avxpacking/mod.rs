
use std::slice;
use std::u32;
use super::BitPacker;
use std::arch::x86_64::{__m256i, _mm256_set1_epi32, _mm256_srli_si256, _mm256_or_si256,
                        _mm256_srli_epi32, _mm256_slli_epi32, _mm256_storeu_si256,
                        _mm256_extract_epi32, _mm256_lddqu_si256, _mm256_and_si256};

use super::most_significant_bit;


const AVX_BLOCK_SIZE: usize = 256;

fn maxbits_u32x8(accumulator: __m256i) -> u8 {
    let ans: u32 = unsafe {
        let tmp1 = _mm256_or_si256(_mm256_srli_si256(accumulator, 8), accumulator);
        let tmp2 = _mm256_or_si256(_mm256_srli_si256(tmp1, 4), tmp1);
        let ans1 = _mm256_extract_epi32(tmp2,0) as u32;
        let ans2 = _mm256_extract_epi32(tmp2,4) as u32;
        ans1.max(ans2)
    };
    most_significant_bit(ans)
}

fn avxmaxbits(arr: &[u32; AVX_BLOCK_SIZE]) -> u8 {
    let data: *const __m256i = arr.as_ptr() as *const __m256i;
    let mut accumulator = unsafe { _mm256_lddqu_si256(data) };
    unroll! {
        for iter in 0..31 {
            let i = iter + 1;
            let newvec = unsafe { _mm256_lddqu_si256(data.offset(i as isize)) };
            accumulator = unsafe { _mm256_or_si256(accumulator,newvec) };
        }
    }
    maxbits_u32x8(accumulator)
}


fn avxpackblock0(input_ptr: *const __m256i, _compressed: *mut __m256i) {
}



unsafe fn avxunpackblock0(compressed: *const __m256i, out: *mut __m256i) {
    let output_slice: &mut [u32] = slice::from_raw_parts_mut(out as *mut u32, 256);
    for el in output_slice.iter_mut() {
        *el = 0u32;
    }
}

include!("generated.rs");

pub struct AVXBitPacker;

impl BitPacker for AVXBitPacker {

    const BLOCK_LEN: usize = AVX_BLOCK_SIZE;

    fn compress(uncompressed: &[u32], compressed: &mut [u8], num_bits: u8) {
        assert_eq!(uncompressed.len(), Self::BLOCK_LEN);
        let uncompressed_ptr = uncompressed.as_ptr() as *const __m256i;
        let compressed_ptr = compressed.as_mut_ptr() as *mut __m256i;
        unsafe {
            avxpackblock(uncompressed_ptr, compressed_ptr, num_bits);
        }
    }

    fn uncompress(compressed: &[u8], uncompressed: &mut [u32], num_bits: u8) {
        assert_eq!(uncompressed.len(), Self::BLOCK_LEN);
        let compressed_ptr = compressed.as_ptr() as *const __m256i;
        let uncompressed_ptr = uncompressed.as_mut_ptr() as *mut __m256i;
        unsafe { avxunpackblock(compressed_ptr, uncompressed_ptr, num_bits); }
    }

    fn num_bits(uncompressed: &[u32]) -> u8 {
        assert_eq!(uncompressed.len(), Self::BLOCK_LEN);
        let input_ptr = uncompressed.as_ptr() as *const __m256i;
        (0..32)
            .map(|i| unsafe {
                let v = _mm256_lddqu_si256(input_ptr.offset(i));
                maxbits_u32x8(v)
            })
            .max()
            .unwrap_or(0u8)
    }
}


#[cfg(test)]
mod test {
    use tests::test_suite_compress_uncompress;
    use super::AVXBitPacker;

    #[test]
    fn test_bitpacker() {
        test_suite_compress_uncompress::<AVXBitPacker>()
    }

    bench_suite!(AVXBitPacker);
}



