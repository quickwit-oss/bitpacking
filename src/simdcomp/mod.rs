
use std::slice;
use std::u32;
use std::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_cvtsi128_si32, _mm_srli_si128, _mm_set1_epi32, _mm_srli_epi32, _mm_storeu_si128, _mm_or_si128, _mm_and_si128, _mm_slli_epi32};
use super::most_significant_bit;
use BitPacker;

const SIMD_BLOCK_LEN: usize = 128;


fn maxbits_u32x4(mut accumulator: __m128i) -> u8 {
    unsafe {
        let _tmp1 = _mm_or_si128(_mm_srli_si128(accumulator, 8), accumulator); // (A,B,C,D) xor (0,0,A,B) = (A,B,C xor A,D xor B)
        let _tmp2 = _mm_or_si128(_mm_srli_si128(_tmp1, 4), _tmp1); //  (A,B,C xor A,D xor B) xor  (0,0,0,C xor A)
        let ans =  _mm_cvtsi128_si32(_tmp2);
        most_significant_bit(ans as u32)
    }
}

unsafe fn memset(out: *mut u32, fill_val: u32, len: usize) {
    let out_slice = slice::from_raw_parts_mut(out, len);
    for val in out_slice.iter_mut() {
        *val = fill_val;
    }
}

unsafe fn __SIMD_fastunpack1_32(mut input_ptr: *const __m128i, _out: *mut u32) {
    let mut out = _out as *mut __m128i;
    let mut InReg1: __m128i = _mm_loadu_si128(input_ptr);
    let mut InReg2: __m128i = InReg1 ;
    let mut OutReg1: __m128i;
    let mut OutReg2: __m128i;
    let mut OutReg3: __m128i;
    let mut OutReg4: __m128i;
    let mask = _mm_set1_epi32(1i32);

    let mut shift = 0i32;

    unroll! {
        for i in 0..8 {
            const i_i32: i32 = i as i32;
            OutReg1 = _mm_and_si128(_mm_srli_epi32(InReg1, 4 * i_i32), mask);
            OutReg2 = _mm_and_si128(_mm_srli_epi32(InReg2, 4 * i_i32+1), mask);
            OutReg3 = _mm_and_si128(_mm_srli_epi32(InReg1, 4 * i_i32+2), mask);
            OutReg4 = _mm_and_si128(_mm_srli_epi32(InReg2, 4 * i_i32+3), mask);
            _mm_storeu_si128(out, OutReg1);
            out = out.offset(1);
            _mm_storeu_si128(out, OutReg2);
            out = out.offset(1);
            _mm_storeu_si128(out, OutReg3);
            out = out.offset(1);
            _mm_storeu_si128(out, OutReg4);
            out = out.offset(1);
        }
    }
}

include!("generated.rs");


unsafe fn __SIMD_fastunpack32_32(input_ptr: *const __m128i, _out: *mut u32) {
    let out: *mut __m128i = _out as *mut __m128i;
    unroll! {
        for i in 0..32 {
            _mm_storeu_si128(out.offset(i as isize), _mm_loadu_si128(input_ptr.offset(i as isize)));
        }
    }
}

fn SIMD_nullunpacker32(out: &mut [u32]) {
    for el in out.iter_mut() {
        *el = 0u32;
    }
}


pub struct SIMDBitPacker;

impl BitPacker for SIMDBitPacker {

    const BLOCK_LEN: usize = 128;

    fn compress(uncompressed: &[u32], compressed: &mut [u8], num_bits: u8) {
        assert_eq!(uncompressed.len(), SIMD_BLOCK_LEN);
        assert!(compressed.len() >= (num_bits as usize) * SIMD_BLOCK_LEN / 8);
        let uncompressed_ptr = uncompressed.as_ptr() as *const __m128i;
        let compressed_ptr = compressed.as_mut_ptr() as *mut __m128i;
        match num_bits {
            0 => { return }
            1 => unsafe { __SIMD_fastpackwithoutmask1_32(uncompressed_ptr, compressed_ptr); }
            2 => unsafe { __SIMD_fastpackwithoutmask2_32(uncompressed_ptr, compressed_ptr); }
            3 => unsafe { __SIMD_fastpackwithoutmask3_32(uncompressed_ptr, compressed_ptr); }
            4 => unsafe { __SIMD_fastpackwithoutmask4_32(uncompressed_ptr, compressed_ptr); }
            5 => unsafe { __SIMD_fastpackwithoutmask5_32(uncompressed_ptr, compressed_ptr); }
            6 => unsafe { __SIMD_fastpackwithoutmask6_32(uncompressed_ptr, compressed_ptr); }
            7 => unsafe { __SIMD_fastpackwithoutmask7_32(uncompressed_ptr, compressed_ptr); }
            8 => unsafe { __SIMD_fastpackwithoutmask8_32(uncompressed_ptr, compressed_ptr); }
            9 => unsafe { __SIMD_fastpackwithoutmask9_32(uncompressed_ptr, compressed_ptr); }
            10 => unsafe { __SIMD_fastpackwithoutmask10_32(uncompressed_ptr, compressed_ptr); }
            11 => unsafe { __SIMD_fastpackwithoutmask11_32(uncompressed_ptr, compressed_ptr); }
            12 => unsafe { __SIMD_fastpackwithoutmask12_32(uncompressed_ptr, compressed_ptr); }
            13 => unsafe { __SIMD_fastpackwithoutmask13_32(uncompressed_ptr, compressed_ptr); }
            14 => unsafe { __SIMD_fastpackwithoutmask14_32(uncompressed_ptr, compressed_ptr); }
            15 => unsafe { __SIMD_fastpackwithoutmask15_32(uncompressed_ptr, compressed_ptr); }
            16 => unsafe { __SIMD_fastpackwithoutmask16_32(uncompressed_ptr, compressed_ptr); }
            17 => unsafe { __SIMD_fastpackwithoutmask17_32(uncompressed_ptr, compressed_ptr); }
            18 => unsafe { __SIMD_fastpackwithoutmask18_32(uncompressed_ptr, compressed_ptr); }
            19 => unsafe { __SIMD_fastpackwithoutmask19_32(uncompressed_ptr, compressed_ptr); }
            20 => unsafe { __SIMD_fastpackwithoutmask20_32(uncompressed_ptr, compressed_ptr); }
            21 => unsafe { __SIMD_fastpackwithoutmask21_32(uncompressed_ptr, compressed_ptr); }
            22 => unsafe { __SIMD_fastpackwithoutmask22_32(uncompressed_ptr, compressed_ptr); }
            23 => unsafe { __SIMD_fastpackwithoutmask23_32(uncompressed_ptr, compressed_ptr); }
            24 => unsafe { __SIMD_fastpackwithoutmask24_32(uncompressed_ptr, compressed_ptr); }
            25 => unsafe { __SIMD_fastpackwithoutmask25_32(uncompressed_ptr, compressed_ptr); }
            26 => unsafe { __SIMD_fastpackwithoutmask26_32(uncompressed_ptr, compressed_ptr); }
            27 => unsafe { __SIMD_fastpackwithoutmask27_32(uncompressed_ptr, compressed_ptr); }
            28 => unsafe { __SIMD_fastpackwithoutmask28_32(uncompressed_ptr, compressed_ptr); }
            29 => unsafe { __SIMD_fastpackwithoutmask29_32(uncompressed_ptr, compressed_ptr); }
            30 => unsafe { __SIMD_fastpackwithoutmask30_32(uncompressed_ptr, compressed_ptr); }
            31 => unsafe { __SIMD_fastpackwithoutmask31_32(uncompressed_ptr, compressed_ptr); }
            32 => unsafe { __SIMD_fastpackwithoutmask32_32(uncompressed_ptr, compressed_ptr); }
            _ => {}
        }
    }

    fn uncompress(compressed: &[u8], uncompressed: &mut [u32], num_bits: u8) {
        assert_eq!(uncompressed.len(), Self::BLOCK_LEN);
        assert!(compressed.len() >= (num_bits as usize) * Self::BLOCK_LEN / 8);
        assert!(num_bits <= 32u8);
        let compressed_ptr = compressed.as_ptr() as *const __m128i;
        let uncompressed_ptr = uncompressed.as_mut_ptr();
        match num_bits {
            0 => SIMD_nullunpacker32(uncompressed),
            1 => unsafe { __SIMD_fastunpack1_32(compressed_ptr, uncompressed_ptr) },
            2 => unsafe { __SIMD_fastunpack2_32(compressed_ptr, uncompressed_ptr) },
            3 => unsafe { __SIMD_fastunpack3_32(compressed_ptr, uncompressed_ptr) },
            4 => unsafe { __SIMD_fastunpack4_32(compressed_ptr, uncompressed_ptr) },
            5 => unsafe { __SIMD_fastunpack5_32(compressed_ptr, uncompressed_ptr) },
            6 => unsafe { __SIMD_fastunpack6_32(compressed_ptr, uncompressed_ptr) },
            7 => unsafe { __SIMD_fastunpack7_32(compressed_ptr, uncompressed_ptr) },
            8 => unsafe { __SIMD_fastunpack8_32(compressed_ptr, uncompressed_ptr) },
            9 => unsafe { __SIMD_fastunpack9_32(compressed_ptr, uncompressed_ptr) },
            10 => unsafe { __SIMD_fastunpack10_32(compressed_ptr, uncompressed_ptr) },
            11 => unsafe { __SIMD_fastunpack11_32(compressed_ptr, uncompressed_ptr) },
            12 => unsafe { __SIMD_fastunpack12_32(compressed_ptr, uncompressed_ptr) },
            13 => unsafe { __SIMD_fastunpack13_32(compressed_ptr, uncompressed_ptr) },
            14 => unsafe { __SIMD_fastunpack14_32(compressed_ptr, uncompressed_ptr) },
            15 => unsafe { __SIMD_fastunpack15_32(compressed_ptr, uncompressed_ptr) },
            16 => unsafe { __SIMD_fastunpack16_32(compressed_ptr, uncompressed_ptr) },
            17 => unsafe { __SIMD_fastunpack17_32(compressed_ptr, uncompressed_ptr) },
            18 => unsafe { __SIMD_fastunpack18_32(compressed_ptr, uncompressed_ptr) },
            19 => unsafe { __SIMD_fastunpack19_32(compressed_ptr, uncompressed_ptr) },
            20 => unsafe { __SIMD_fastunpack20_32(compressed_ptr, uncompressed_ptr) },
            21 => unsafe { __SIMD_fastunpack21_32(compressed_ptr, uncompressed_ptr) },
            22 => unsafe { __SIMD_fastunpack22_32(compressed_ptr, uncompressed_ptr) },
            23 => unsafe { __SIMD_fastunpack23_32(compressed_ptr, uncompressed_ptr) },
            24 => unsafe { __SIMD_fastunpack24_32(compressed_ptr, uncompressed_ptr) },
            25 => unsafe { __SIMD_fastunpack25_32(compressed_ptr, uncompressed_ptr) },
            26 => unsafe { __SIMD_fastunpack26_32(compressed_ptr, uncompressed_ptr) },
            27 => unsafe { __SIMD_fastunpack27_32(compressed_ptr, uncompressed_ptr) },
            28 => unsafe { __SIMD_fastunpack28_32(compressed_ptr, uncompressed_ptr) },
            29 => unsafe { __SIMD_fastunpack29_32(compressed_ptr, uncompressed_ptr) },
            30 => unsafe { __SIMD_fastunpack30_32(compressed_ptr, uncompressed_ptr) },
            31 => unsafe { __SIMD_fastunpack31_32(compressed_ptr, uncompressed_ptr) },
            32 => unsafe { __SIMD_fastunpack32_32(compressed_ptr, uncompressed_ptr) },
            _ => {}
        }
    }

    fn num_bits(uncompressed: &[u32]) -> u8 {
        assert_eq!(uncompressed.len(), Self::BLOCK_LEN);
        let input_ptr = uncompressed.as_ptr() as *const __m128i;
        (0..32)
            .map(|i| unsafe {
                let v = _mm_loadu_si128(input_ptr.offset(i));
                maxbits_u32x4(v)
            })
            .max()
            .unwrap_or(0u8)
    }
}



#[cfg(test)]
mod test {

    use tests::test_suite_compress_uncompress;
    use super::SIMDBitPacker;

    #[test]
    fn test_bitpacker() {
        test_suite_compress_uncompress::<SIMDBitPacker>()
    }

    bench_suite!(SIMDBitPacker);

}



