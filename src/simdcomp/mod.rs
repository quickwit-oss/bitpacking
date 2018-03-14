
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

macro_rules! pack_unpack_with_bits {

    ($name:ident, $n:expr) => {

        mod $name {

            use super::SIMD_BLOCK_LEN;
            use std::arch::x86_64::{
                __m128i,
                _mm_loadu_si128,
                _mm_cvtsi128_si32,
                _mm_srli_si128,
                _mm_set1_epi32,
                _mm_slli_epi32,
                _mm_srli_epi32,
                _mm_storeu_si128,
                _mm_or_si128,
                _mm_and_si128
                };

            const NUM_BITS: usize = $n;
            const NUM_BYTES_PER_BLOCK: usize = NUM_BITS * super::SIMD_BLOCK_LEN / 8;

            #[inline(always)]
            pub fn pack(input_arr: &[u32], output_arr: &mut [u8]) {

                assert_eq!(input_arr.len(), SIMD_BLOCK_LEN);
                assert!(output_arr.len() >= NUM_BYTES_PER_BLOCK);


                let input_ptr = input_arr.as_ptr() as *const __m128i;
                let mut output_ptr = output_arr.as_mut_ptr() as *mut __m128i;

                unsafe {
                    let mut out_register: __m128i = _mm_loadu_si128(input_ptr);

                    unroll! {
                        for iter in 0..31 {
                            const i: usize = 1 + iter;
                            let in_register: __m128i = _mm_loadu_si128(input_ptr.offset(i as isize));
                            const bits_filled: usize = i * NUM_BITS;
                            const inner_cursor: usize = bits_filled % 32;
                            const inner_capacity: usize = 32 - inner_cursor;
                            if inner_cursor > 0 {
                                out_register = _mm_or_si128(out_register, _mm_slli_epi32(in_register, inner_cursor as i32));
                            } else {
                                out_register = in_register;
                            }
                            if inner_capacity <= NUM_BITS {
                                _mm_storeu_si128(output_ptr, out_register);
                                output_ptr = output_ptr.offset(1);
                                if inner_capacity < NUM_BITS {
                                    out_register = _mm_srli_epi32(in_register, inner_capacity as i32);
                                }
                            }
                        }
                    }
                }
            }


            #[inline(always)]
            pub fn unpack(compressed: &[u8], output: &mut [u32]) {

                assert!(compressed.len() >= NUM_BYTES_PER_BLOCK);
                assert_eq!(output.len(), SIMD_BLOCK_LEN);

                let mut input_ptr = compressed.as_ptr() as *const __m128i;
                let output_ptr = output.as_mut_ptr()  as *mut __m128i;

                let mask_scalar: u32 = ((1u64 << NUM_BITS) - 1u64) as u32;
                unsafe {
                    let mask = _mm_set1_epi32(mask_scalar as i32);

                    let mut in_register: __m128i = _mm_loadu_si128(input_ptr);
                    input_ptr = input_ptr.offset(1);

                    let out_register = _mm_and_si128(  in_register, mask);
                    _mm_storeu_si128(output_ptr, out_register);

                    unroll! {
                        for iter in 0..31 {
                            const i: usize = iter + 1;

                            const inner_cursor: usize = (i * NUM_BITS) % 32;
                            const inner_capacity: usize = 32 - inner_cursor;

                            let mut out_register: __m128i =
                                if inner_cursor == 0 {
                                    _mm_and_si128(in_register, mask)
                                } else {
                                    _mm_and_si128(_mm_srli_epi32(in_register, inner_cursor as i32), mask)
                                };

                            if inner_capacity <= NUM_BITS && i != 31 {
                                in_register = _mm_loadu_si128(input_ptr);
                                input_ptr = input_ptr.offset(1);
                                if inner_capacity < NUM_BITS {
                                    out_register = _mm_or_si128(out_register, _mm_and_si128(_mm_slli_epi32(in_register, inner_capacity as i32), mask))
                                }

                            }

                            _mm_storeu_si128(output_ptr.offset(i as isize), out_register);
                        }
                    }
                }
            }
        }
    }
}



pack_unpack_with_bits!(pack_unpack_with_bits_1, 1);
pack_unpack_with_bits!(pack_unpack_with_bits_2, 2);
pack_unpack_with_bits!(pack_unpack_with_bits_3, 3);
pack_unpack_with_bits!(pack_unpack_with_bits_4, 4);
pack_unpack_with_bits!(pack_unpack_with_bits_5, 5);
pack_unpack_with_bits!(pack_unpack_with_bits_6, 6);
pack_unpack_with_bits!(pack_unpack_with_bits_7, 7);
pack_unpack_with_bits!(pack_unpack_with_bits_8, 8);
pack_unpack_with_bits!(pack_unpack_with_bits_9, 9);
pack_unpack_with_bits!(pack_unpack_with_bits_10, 10);
pack_unpack_with_bits!(pack_unpack_with_bits_11, 11);
pack_unpack_with_bits!(pack_unpack_with_bits_12, 12);
pack_unpack_with_bits!(pack_unpack_with_bits_13, 13);
pack_unpack_with_bits!(pack_unpack_with_bits_14, 14);
pack_unpack_with_bits!(pack_unpack_with_bits_15, 15);
pack_unpack_with_bits!(pack_unpack_with_bits_16, 16);
pack_unpack_with_bits!(pack_unpack_with_bits_17, 17);
pack_unpack_with_bits!(pack_unpack_with_bits_18, 18);
pack_unpack_with_bits!(pack_unpack_with_bits_19, 19);
pack_unpack_with_bits!(pack_unpack_with_bits_20, 20);
pack_unpack_with_bits!(pack_unpack_with_bits_21, 21);
pack_unpack_with_bits!(pack_unpack_with_bits_22, 22);
pack_unpack_with_bits!(pack_unpack_with_bits_23, 23);
pack_unpack_with_bits!(pack_unpack_with_bits_24, 24);
pack_unpack_with_bits!(pack_unpack_with_bits_25, 25);
pack_unpack_with_bits!(pack_unpack_with_bits_26, 26);
pack_unpack_with_bits!(pack_unpack_with_bits_27, 27);
pack_unpack_with_bits!(pack_unpack_with_bits_28, 28);
pack_unpack_with_bits!(pack_unpack_with_bits_29, 29);
pack_unpack_with_bits!(pack_unpack_with_bits_30, 30);
pack_unpack_with_bits!(pack_unpack_with_bits_31, 31);
pack_unpack_with_bits!(pack_unpack_with_bits_32, 32);


pub struct SIMDBitPacker;

impl BitPacker for SIMDBitPacker {

    const BLOCK_LEN: usize = 128;

    fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8) {
        match num_bits {
            0 => {}
            1 => pack_unpack_with_bits_1::pack(decompressed, compressed),
            2 => pack_unpack_with_bits_2::pack(decompressed, compressed),
            3 => pack_unpack_with_bits_3::pack(decompressed, compressed),
            4 => pack_unpack_with_bits_4::pack(decompressed, compressed),
            5 => pack_unpack_with_bits_5::pack(decompressed, compressed),
            6 => pack_unpack_with_bits_6::pack(decompressed, compressed),
            7 => pack_unpack_with_bits_7::pack(decompressed, compressed),
            8 => pack_unpack_with_bits_8::pack(decompressed, compressed),
            9 => pack_unpack_with_bits_9::pack(decompressed, compressed),
            10 => pack_unpack_with_bits_10::pack(decompressed, compressed),
            11 => pack_unpack_with_bits_11::pack(decompressed, compressed),
            12 => pack_unpack_with_bits_12::pack(decompressed, compressed),
            13 => pack_unpack_with_bits_13::pack(decompressed, compressed),
            14 => pack_unpack_with_bits_14::pack(decompressed, compressed),
            15 => pack_unpack_with_bits_15::pack(decompressed, compressed),
            16 => pack_unpack_with_bits_16::pack(decompressed, compressed),
            17 => pack_unpack_with_bits_17::pack(decompressed, compressed),
            18 => pack_unpack_with_bits_18::pack(decompressed, compressed),
            19 => pack_unpack_with_bits_19::pack(decompressed, compressed),
            20 => pack_unpack_with_bits_20::pack(decompressed, compressed),
            21 => pack_unpack_with_bits_21::pack(decompressed, compressed),
            22 => pack_unpack_with_bits_22::pack(decompressed, compressed),
            23 => pack_unpack_with_bits_23::pack(decompressed, compressed),
            24 => pack_unpack_with_bits_24::pack(decompressed, compressed),
            25 => pack_unpack_with_bits_25::pack(decompressed, compressed),
            26 => pack_unpack_with_bits_26::pack(decompressed, compressed),
            27 => pack_unpack_with_bits_27::pack(decompressed, compressed),
            28 => pack_unpack_with_bits_28::pack(decompressed, compressed),
            29 => pack_unpack_with_bits_29::pack(decompressed, compressed),
            30 => pack_unpack_with_bits_30::pack(decompressed, compressed),
            31 => pack_unpack_with_bits_31::pack(decompressed, compressed),
            32 => pack_unpack_with_bits_32::pack(decompressed, compressed),
            _ => {}
        }
    }

    fn decompress(compressed: &[u8], decompressed: &mut [u32], num_bits: u8) {
        assert_eq!(decompressed.len(), Self::BLOCK_LEN);
        assert!(compressed.len() >= (num_bits as usize) * Self::BLOCK_LEN / 8);
        assert!(num_bits <= 32u8);
        match num_bits {
            0 => {
                for el in decompressed.iter_mut() {
                    *el = 0u32;
                }
            },
            1 => pack_unpack_with_bits_1::unpack(compressed, decompressed),
            2 => pack_unpack_with_bits_2::unpack(compressed, decompressed),
            3 => pack_unpack_with_bits_3::unpack(compressed, decompressed),
            4 => pack_unpack_with_bits_4::unpack(compressed, decompressed),
            5 => pack_unpack_with_bits_5::unpack(compressed, decompressed),
            6 => pack_unpack_with_bits_6::unpack(compressed, decompressed),
            7 => pack_unpack_with_bits_7::unpack(compressed, decompressed),
            8 => pack_unpack_with_bits_8::unpack(compressed, decompressed),
            9 => pack_unpack_with_bits_9::unpack(compressed, decompressed),
            10 => pack_unpack_with_bits_10::unpack(compressed, decompressed),
            11 => pack_unpack_with_bits_11::unpack(compressed, decompressed),
            12 => pack_unpack_with_bits_12::unpack(compressed, decompressed),
            13 => pack_unpack_with_bits_13::unpack(compressed, decompressed),
            14 => pack_unpack_with_bits_14::unpack(compressed, decompressed),
            15 => pack_unpack_with_bits_15::unpack(compressed, decompressed),
            16 => pack_unpack_with_bits_16::unpack(compressed, decompressed),
            17 => pack_unpack_with_bits_17::unpack(compressed, decompressed),
            18 => pack_unpack_with_bits_18::unpack(compressed, decompressed),
            19 => pack_unpack_with_bits_19::unpack(compressed, decompressed),
            20 => pack_unpack_with_bits_20::unpack(compressed, decompressed),
            21 => pack_unpack_with_bits_21::unpack(compressed, decompressed),
            22 => pack_unpack_with_bits_22::unpack(compressed, decompressed),
            23 => pack_unpack_with_bits_23::unpack(compressed, decompressed),
            24 => pack_unpack_with_bits_24::unpack(compressed, decompressed),
            25 => pack_unpack_with_bits_25::unpack(compressed, decompressed),
            26 => pack_unpack_with_bits_26::unpack(compressed, decompressed),
            27 => pack_unpack_with_bits_27::unpack(compressed, decompressed),
            28 => pack_unpack_with_bits_28::unpack(compressed, decompressed),
            29 => pack_unpack_with_bits_29::unpack(compressed, decompressed),
            30 => pack_unpack_with_bits_30::unpack(compressed, decompressed),
            31 => pack_unpack_with_bits_31::unpack(compressed, decompressed),
            32 => pack_unpack_with_bits_32::unpack(compressed, decompressed),
            _ => {}
        }
    }

    fn num_bits(decompressed: &[u32]) -> u8 {
        assert_eq!(decompressed.len(), Self::BLOCK_LEN);
        let input_ptr = decompressed.as_ptr() as *const __m128i;
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

    use tests::test_suite_compress_decompress;
    use super::SIMDBitPacker;

    #[test]
    fn test_bitpacker() {
        test_suite_compress_decompress::<SIMDBitPacker>()
    }

     bench_suite!(SIMDBitPacker);



    /*
    #[test]
    fn test_generated_vs_macro() {
        unsafe {


            let mut data = vec![13, 31, 4, 0, 13, 26, 14, 30, 13, 26, 14, 30, 13];
            data.resize(128, 1u32);

            let mut compressed = vec![0u8; 5*16];

                        let mut result_a = vec![0u32; 128];
            let mut result_b = vec![0u32; 128];

            __SIMD_fastpackwithoutmask5_32(
                data.as_ptr() as *const __m128i,
                compressed[..].as_mut_ptr() as *mut __m128i
            );

            let input_ptr = compressed.as_ptr() as *const __m128i;

            unpack5(compressed.as_ptr() as *const __m128i, result_a[..].as_mut_ptr());
            __SIMD_fastunpack5_32(compressed.as_ptr() as *const __m128i, result_b[..].as_mut_ptr());

            for i in 0..128 {
//                assert_eq!(result_a[i], data[i], "Not the same at index {}", i);
                assert_eq!(result_a[i], data[i], "Not the same at index {}", i);
            }
        }
    }


    #[test]
    fn test_comp() {
        unsafe {

            let PATTERN = [13, 31, 4, 0, 13, 26, 14, 30, 13, 26, 14, 30, 13];
            let data: Vec<u32> = (0..128)
                .map(|el| PATTERN[el % 13])
                .collect();

            let mut compressed = vec![0u8; 5*16];

            __SIMD_fastpackwithoutmask5_32(
                data.as_ptr() as *const __m128i,
                compressed[..].as_mut_ptr() as *mut __m128i
            );

            let input_ptr = compressed.as_ptr() as *const __m128i;
            let mut result_a = vec![0u32; 128];
            let mut result_b = vec![0u32; 128];
            unpack5(compressed.as_ptr() as *const __m128i, result_a[..].as_mut_ptr());

            let mut pos: Vec<usize> = (0..128)
                .filter(|&el| result_a[el] != data[el])
                .collect();
            for i in 0..128 {
//                assert_eq!(result_a[i], data[i], "Not the same at index {}", i);
                assert_eq!(pos.len(),0, "Not the same at index {:?}", pos);
            }
        }
    }
    */
}



