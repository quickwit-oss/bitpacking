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
            pub fn pack<Delta: FnMut(DataType) -> DataType>(input_arr: &[u32], output_arr: &mut [u8], mut delta_computer: Delta) {
                assert_eq!(input_arr.len(), BLOCK_LEN, "Input block too small {}, (expected {})", input_arr.len(), BLOCK_LEN);
                assert!(output_arr.len() >= NUM_BYTES_PER_BLOCK, "Output array too small (numbits {}). {} <= {}", NUM_BITS, output_arr.len(), NUM_BYTES_PER_BLOCK);

                let input_ptr = input_arr.as_ptr() as *const DataType;
                let mut output_ptr = output_arr.as_mut_ptr() as *mut DataType;

                unsafe {
                    let mut out_register: DataType = delta_computer(load_unaligned(input_ptr));

                    unroll! {
                        for iter in 0..30 {
                            const i: usize = 1 + iter;

                            const bits_filled: usize = i * NUM_BITS;
                            const inner_cursor: usize = bits_filled % 32;
                            const remaining: usize = 32 - inner_cursor;

                            let offset_ptr = input_ptr.offset(i as isize);
                            let in_register: DataType = delta_computer(load_unaligned(offset_ptr));

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
                    let in_register: DataType = delta_computer(load_unaligned(input_ptr.offset(31 as isize)));
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
            pub fn unpack<Output: FnMut(DataType)>(compressed: &[u8], mut output: Output) {

                assert!(compressed.len() >= NUM_BYTES_PER_BLOCK, "Compressed array seems too small. ({} < {}) ", compressed.len(), NUM_BYTES_PER_BLOCK);

                let mut input_ptr = compressed.as_ptr() as *const DataType;

                let mask_scalar: u32 = ((1u64 << NUM_BITS) - 1u64) as u32;
                unsafe {
                    let mask = set1(mask_scalar as i32);

                    let mut in_register: DataType = load_unaligned(input_ptr);

                    let out_register = op_and(in_register, mask);
                    output(out_register);

                    unroll! {
                        for iter in 0..31 {
                            const i: usize = iter + 1;

                            const inner_cursor: usize = (i * NUM_BITS) % 32;
                            const inner_capacity: usize = 32 - inner_cursor;

                            // LLVM will not emit the shift operand if
                            // `inner_cursor` is 0.
                            let shifted_in_register = right_shift_32(in_register, inner_cursor as i32);
                            let mut out_register: DataType = op_and(shifted_in_register, mask);

                            // We consumed our current quadruplets entirely.
                            // We therefore read another one.
                            if inner_capacity <= NUM_BITS && i != 31 {
                                input_ptr = input_ptr.offset(1);
                                in_register = load_unaligned(input_ptr);

                                // This quadruplets is actually cutting one of
                                // our `DataType`. We need to read the next one.
                                if inner_capacity < NUM_BITS {
                                    let shifted = left_shift_32(in_register, inner_capacity as i32);
                                    let masked = op_and(shifted, mask);
                                    out_register = op_or(out_register, masked);
                                }
                            }

                            output(out_register);
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

        const BLOCK_LEN: usize = NUM_INTS_PER_REGISTER * 32;

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

        impl $bitpacker_name {

            fn compress_generic<Delta: FnMut(DataType) -> DataType>(decompressed: &[u32], compressed: &mut [u8], num_bits: u8, delta_computer: Delta) {
                match num_bits {
                    0 => {}
                    1 => pack_unpack_with_bits_1::pack(decompressed, compressed, delta_computer),
                    2 => pack_unpack_with_bits_2::pack(decompressed, compressed, delta_computer),
                    3 => pack_unpack_with_bits_3::pack(decompressed, compressed, delta_computer),
                    4 => pack_unpack_with_bits_4::pack(decompressed, compressed, delta_computer),
                    5 => pack_unpack_with_bits_5::pack(decompressed, compressed, delta_computer),
                    6 => pack_unpack_with_bits_6::pack(decompressed, compressed, delta_computer),
                    7 => pack_unpack_with_bits_7::pack(decompressed, compressed, delta_computer),
                    8 => pack_unpack_with_bits_8::pack(decompressed, compressed, delta_computer),
                    9 => pack_unpack_with_bits_9::pack(decompressed, compressed, delta_computer),
                    10 => pack_unpack_with_bits_10::pack(decompressed, compressed, delta_computer),
                    11 => pack_unpack_with_bits_11::pack(decompressed, compressed, delta_computer),
                    12 => pack_unpack_with_bits_12::pack(decompressed, compressed, delta_computer),
                    13 => pack_unpack_with_bits_13::pack(decompressed, compressed, delta_computer),
                    14 => pack_unpack_with_bits_14::pack(decompressed, compressed, delta_computer),
                    15 => pack_unpack_with_bits_15::pack(decompressed, compressed, delta_computer),
                    16 => pack_unpack_with_bits_16::pack(decompressed, compressed, delta_computer),
                    17 => pack_unpack_with_bits_17::pack(decompressed, compressed, delta_computer),
                    18 => pack_unpack_with_bits_18::pack(decompressed, compressed, delta_computer),
                    19 => pack_unpack_with_bits_19::pack(decompressed, compressed, delta_computer),
                    20 => pack_unpack_with_bits_20::pack(decompressed, compressed, delta_computer),
                    21 => pack_unpack_with_bits_21::pack(decompressed, compressed, delta_computer),
                    22 => pack_unpack_with_bits_22::pack(decompressed, compressed, delta_computer),
                    23 => pack_unpack_with_bits_23::pack(decompressed, compressed, delta_computer),
                    24 => pack_unpack_with_bits_24::pack(decompressed, compressed, delta_computer),
                    25 => pack_unpack_with_bits_25::pack(decompressed, compressed, delta_computer),
                    26 => pack_unpack_with_bits_26::pack(decompressed, compressed, delta_computer),
                    27 => pack_unpack_with_bits_27::pack(decompressed, compressed, delta_computer),
                    28 => pack_unpack_with_bits_28::pack(decompressed, compressed, delta_computer),
                    29 => pack_unpack_with_bits_29::pack(decompressed, compressed, delta_computer),
                    30 => pack_unpack_with_bits_30::pack(decompressed, compressed, delta_computer),
                    31 => pack_unpack_with_bits_31::pack(decompressed, compressed, delta_computer),
                    32 => pack_unpack_with_bits_32::pack(decompressed, compressed, delta_computer),
                _ => {}
                }
            }
        }

        impl BitPacker for $bitpacker_name {

            const BLOCK_LEN: usize = BLOCK_LEN;

            type DataType = DataType;

            fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8) {
                let no_delta = |curr| { curr };
                Self::compress_generic(
                    decompressed,
                    compressed,
                    num_bits,
                    no_delta);
            }

            fn compress_sorted(initial: u32,
                      decompressed: &[u32],
                      compressed: &mut [u8],
                      num_bits: u8) {

                let mut previous = unsafe { set1(initial as i32) };
                let delta_computer = |current: Self::DataType| {
                    let result = compute_delta(current, previous);
                    previous = current;
                    result
                };
                Self::compress_generic(
                    decompressed,
                    compressed,
                    num_bits,
                    delta_computer);
            }

            #[inline(always)]
            fn decompress_to<Output: FnMut(Self::DataType)>(compressed: &[u8], mut sink: Output, num_bits: u8) {
                assert!(num_bits <= 32u8, "Num bits must be <= 32. Was {}.", num_bits);
                match num_bits {
                    0 => {
                        let zero = unsafe { set1(0i32) };
                        for _ in 0..32 {
                            sink(zero);
                        }
                    },
                    1 => pack_unpack_with_bits_1::unpack(compressed, sink),
                    2 => pack_unpack_with_bits_2::unpack(compressed, sink),
                    3 => pack_unpack_with_bits_3::unpack(compressed, sink),
                    4 => pack_unpack_with_bits_4::unpack(compressed, sink),
                    5 => pack_unpack_with_bits_5::unpack(compressed, sink),
                    6 => pack_unpack_with_bits_6::unpack(compressed, sink),
                    7 => pack_unpack_with_bits_7::unpack(compressed, sink),
                    8 => pack_unpack_with_bits_8::unpack(compressed, sink),
                    9 => pack_unpack_with_bits_9::unpack(compressed, sink),
                    10 => pack_unpack_with_bits_10::unpack(compressed, sink),
                    11 => pack_unpack_with_bits_11::unpack(compressed, sink),
                    12 => pack_unpack_with_bits_12::unpack(compressed, sink),
                    13 => pack_unpack_with_bits_13::unpack(compressed, sink),
                    14 => pack_unpack_with_bits_14::unpack(compressed, sink),
                    15 => pack_unpack_with_bits_15::unpack(compressed, sink),
                    16 => pack_unpack_with_bits_16::unpack(compressed, sink),
                    17 => pack_unpack_with_bits_17::unpack(compressed, sink),
                    18 => pack_unpack_with_bits_18::unpack(compressed, sink),
                    19 => pack_unpack_with_bits_19::unpack(compressed, sink),
                    20 => pack_unpack_with_bits_20::unpack(compressed, sink),
                    21 => pack_unpack_with_bits_21::unpack(compressed, sink),
                    22 => pack_unpack_with_bits_22::unpack(compressed, sink),
                    23 => pack_unpack_with_bits_23::unpack(compressed, sink),
                    24 => pack_unpack_with_bits_24::unpack(compressed, sink),
                    25 => pack_unpack_with_bits_25::unpack(compressed, sink),
                    26 => pack_unpack_with_bits_26::unpack(compressed, sink),
                    27 => pack_unpack_with_bits_27::unpack(compressed, sink),
                    28 => pack_unpack_with_bits_28::unpack(compressed, sink),
                    29 => pack_unpack_with_bits_29::unpack(compressed, sink),
                    30 => pack_unpack_with_bits_30::unpack(compressed, sink),
                    31 => pack_unpack_with_bits_31::unpack(compressed, sink),
                    32 => pack_unpack_with_bits_32::unpack(compressed, sink),
                    _ => {}
                }
            }

            fn decompress(compressed: &[u8], decompressed: &mut [u32], num_bits: u8) {
                assert!(
                    decompressed.len() >= Self::BLOCK_LEN,
                    "The output array is not large enough : ({} >= {})",
                    decompressed.len(), Self::BLOCK_LEN);
                let mut output_ptr = decompressed.as_mut_ptr()  as *mut DataType;
                let output = |out_register| {
                    unsafe {
                        store_unaligned(output_ptr, out_register);
                        output_ptr = output_ptr.offset(1);
                    }
                };
                Self::decompress_to(compressed, output, num_bits);
            }

            fn decompress_sorted(
                initial: u32,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8)
            {
                unsafe {
                    let mut current = set1(initial as i32);
                    assert!(
                        decompressed.len() >= Self::BLOCK_LEN,
                        "The output array is not large enough : ({} >= {})",
                        decompressed.len(), Self::BLOCK_LEN);
                    let mut output_ptr = decompressed.as_mut_ptr()  as *mut DataType;
                    let output = |delta| {
                        current = integrate_delta(current, delta);
                        store_unaligned(output_ptr, current);
                        output_ptr = output_ptr.offset(1);
                    };
                    Self::decompress_to(compressed, output, num_bits);
                }
            }

            fn num_bits(decompressed: &[u32]) -> u8 {
                assert_eq!(decompressed.len(), BLOCK_LEN, "`decompressed`'s len is not `BLOCK_LEN={}`", BLOCK_LEN);
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

            fn num_bits_sorted(initial: u32, decompressed: &[u32]) -> u8 {
                let initial_vec = unsafe { set1(initial as i32) };
                let data: *const DataType = decompressed.as_ptr() as *const DataType;

                let first = unsafe { load_unaligned(data) };
                let mut accumulator = unsafe { compute_delta(load_unaligned(data), initial_vec) };
                let mut previous = first;

                unroll! {
                    for iter in 0..30 {
                        let i = iter + 1;
                        let current = unsafe { load_unaligned(data.offset(i as isize)) };
                        let delta = unsafe { compute_delta(current, previous) };
                        accumulator =  unsafe { op_or(accumulator, delta) };
                        previous = current;
                    }
                }
                let current = unsafe { load_unaligned(data.offset(31 as isize)) };
                let delta = unsafe { compute_delta(current, previous) };
                accumulator =  unsafe { op_or(accumulator, delta) };
                most_significant_bit(or_collapse_to_u32(accumulator))
            }

        }


        #[cfg(test)]
        mod test {
            use tests::test_suite_compress_decompress;
            use super::$bitpacker_name;

            #[test]
            fn test_bitpacker() {
                test_suite_compress_decompress::<$bitpacker_name>(false);
            }

            #[test]
            fn test_bitpacker_delta() {
                test_suite_compress_decompress::<$bitpacker_name>(true);
            }


            bench_suite!($bitpacker_name);
        }

    }

}
