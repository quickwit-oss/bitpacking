macro_rules! pack_unpack_with_bits {

    ($name:ident, $n:expr) => {

        mod $name {
            use super::BLOCK_LEN;
            use super::{DataType,
                set1,
                right_shift_32,
                left_shift_32,
                op_or,
                op_and,
                load_unaligned,
                store_unaligned};

            const NUM_BITS: usize = $n;
            const NUM_BYTES_PER_BLOCK: usize = NUM_BITS * super::BLOCK_LEN / 8;

            #[inline(always)]
            pub fn pack(input_arr: &[u32], output_arr: &mut [u8]) {

                assert_eq!(input_arr.len(), BLOCK_LEN);
                assert!(output_arr.len() >= NUM_BYTES_PER_BLOCK);

                let input_ptr = input_arr.as_ptr() as *const DataType;
                let mut output_ptr = output_arr.as_mut_ptr() as *mut DataType;

                unsafe {
                    let mut out_register: DataType = load_unaligned(input_ptr);

                    unroll! {
                        for iter in 0..30 {
                            const i: usize = 1 + iter;

                            const bits_filled: usize = i * NUM_BITS;
                            const inner_cursor: usize = bits_filled % 32;
                            const remaining: usize = 32 - inner_cursor;

                            let offset_ptr = input_ptr.offset(i as isize);
                            let in_register: DataType = load_unaligned(offset_ptr);

                            out_register =
                                if inner_cursor > 0 {
                                    let shifted = left_shift_32(in_register, inner_cursor as i32);
                                    op_or(out_register, shifted)
                                } else {
                                    in_register
                                };

                            if remaining <= NUM_BITS {
                                store_unaligned(output_ptr, out_register);
                                output_ptr = output_ptr.offset(1);
                                if remaining < NUM_BITS {
                                    out_register = right_shift_32(in_register, remaining as i32);
                                }
                            }
                        }
                    }
                    let in_register: DataType = load_unaligned(input_ptr.offset(31 as isize));
                    out_register =
                        if NUM_BITS != 32 {
                            let shifted = left_shift_32(in_register, 32 - NUM_BITS as i32);
                            op_or(out_register, shifted)
                        } else {
                            in_register
                        };
                    store_unaligned(output_ptr, out_register);
                }
            }


            #[inline(always)]
            pub fn unpack(compressed: &[u8], output: &mut [u32]) {

                assert!(compressed.len() >= NUM_BYTES_PER_BLOCK);
                assert_eq!(output.len(), BLOCK_LEN);

                let mut input_ptr = compressed.as_ptr() as *const DataType;
                let output_ptr = output.as_mut_ptr()  as *mut DataType;

                let mask_scalar: u32 = ((1u64 << NUM_BITS) - 1u64) as u32;
                unsafe {
                    let mask = set1(mask_scalar as i32);

                    let mut in_register: DataType = load_unaligned(input_ptr);

                    let out_register = op_and(in_register, mask);
                    store_unaligned(output_ptr, out_register);

                    unroll! {
                        for iter in 0..31 {
                            const i: usize = iter + 1;

                            const inner_cursor: usize = (i * NUM_BITS) % 32;
                            const inner_capacity: usize = 32 - inner_cursor;

                            let mut out_register: DataType =
                                if inner_cursor == 0 {
                                    op_and(in_register, mask)
                                } else {
                                    op_and(right_shift_32(in_register, inner_cursor as i32), mask)
                                };
                            if inner_capacity <= NUM_BITS && i != 31 {
                                input_ptr = input_ptr.offset(1);
                                in_register = load_unaligned(input_ptr);
                                if inner_capacity < NUM_BITS {
                                    let shifted = left_shift_32(in_register, inner_capacity as i32);
                                    let masked = op_and(shifted, mask);
                                    out_register = op_or(out_register, masked);
                                }
                            }
                            store_unaligned(output_ptr.offset(i as isize), out_register);
                        }
                    }
                }
            }
        }
    }
}

macro_rules! declare_bitpacker {

    ($bitpacker_name:ident) => {

        use super::BitPacker;
        use super::most_significant_bit;

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

        pub struct $bitpacker_name;

        impl BitPacker for $bitpacker_name {

            const BLOCK_LEN: usize = BLOCK_LEN;

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
                let data: *const DataType = decompressed.as_ptr() as *const DataType;
                let mut accumulator = unsafe { load_unaligned(data) };
                unroll! {
                    for iter in 0..31 {
                        let i = iter + 1;
                        let newvec = unsafe { load_unaligned(data.offset(i as isize)) };
                        accumulator = unsafe { op_or(accumulator, newvec) };
                    }
                }
                most_significant_bit(or_collapse_to_u32(accumulator))
            }
        }


        #[cfg(test)]
        mod test {
            use tests::test_suite_compress_decompress;
            use super::$bitpacker_name;

            #[test]
            fn test_bitpacker() {
                test_suite_compress_decompress::<$bitpacker_name>()
            }

            bench_suite!($bitpacker_name);
        }

    }

}
