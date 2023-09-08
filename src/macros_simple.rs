macro_rules! declare_bitpacker_simple {
    ($cpufeature:meta) => {
        fn compute_num_bytes_per_block(num_bits: usize) -> usize {
            (num_bits * BLOCK_LEN) / 8
        }
        use super::UnsafeBitPacker;
        use crate::most_significant_bit;

        pub unsafe fn pack<TDeltaComputer: Transformer>(
            input_arr: &[u32],
            output_arr: &mut [u8],
            num_bits: usize,
            mut delta_computer: TDeltaComputer,
        ) -> usize {
            let num_bytes_per_block = compute_num_bytes_per_block(num_bits);
            assert_eq!(
                input_arr.len(),
                BLOCK_LEN,
                "Input block too small {}, (expected {})",
                input_arr.len(),
                BLOCK_LEN
            );
            assert!(
                output_arr.len() >= num_bytes_per_block,
                "Output array too small (numbits {}). {} <= {}",
                num_bits,
                output_arr.len(),
                num_bytes_per_block
            );

            let input_ptr = input_arr.as_ptr() as *const DataType;
            let mut output_ptr = output_arr.as_mut_ptr() as *mut DataType;
            let mut out_register: DataType = delta_computer.transform(load_unaligned(input_ptr));

            for i in 1..31 {
                let bits_filled: usize = i * num_bits;
                let inner_cursor: usize = bits_filled % 32;
                let remaining: usize = 32 - inner_cursor;

                let offset_ptr = input_ptr.add(i);
                let in_register: DataType = delta_computer.transform(load_unaligned(offset_ptr));

                out_register = if inner_cursor > 0 {
                    let shifted = left_shift_32(in_register, inner_cursor as i32);
                    op_or(out_register, shifted)
                } else {
                    in_register
                };

                if remaining <= num_bits {
                    store_unaligned(output_ptr, out_register);
                    output_ptr = output_ptr.offset(1);
                    if remaining < num_bits {
                        out_register = right_shift_32(in_register, remaining as i32);
                    }
                }
            }

            let in_register: DataType = delta_computer.transform(load_unaligned(input_ptr.add(31)));

            let shifted = left_shift_32(in_register, 32 - num_bits as i32);
            out_register = op_or(out_register, shifted);
            store_unaligned(output_ptr, out_register);
            num_bytes_per_block
        }

        pub unsafe fn pack_32<TDeltaComputer: Transformer>(
            input_arr: &[u32],
            output_arr: &mut [u8],
            mut delta_computer: TDeltaComputer,
        ) -> usize {
            assert_eq!(
                input_arr.len(),
                BLOCK_LEN,
                "Input block too small {}, (expected {})",
                input_arr.len(),
                BLOCK_LEN
            );
            let num_bytes_per_block = compute_num_bytes_per_block(32);
            assert!(
                output_arr.len() >= num_bytes_per_block,
                "Output array too small (numbits {}). {} <= {}",
                32,
                output_arr.len(),
                num_bytes_per_block
            );

            let input_ptr: *const DataType = input_arr.as_ptr() as *const DataType;
            let output_ptr = output_arr.as_mut_ptr() as *mut DataType;
            for i in 0..32 {
                let input_offset_ptr = input_ptr.offset(i as isize);
                let output_offset_ptr = output_ptr.offset(i as isize);
                let input_register = load_unaligned(input_offset_ptr);
                let output_register = delta_computer.transform(input_register);
                store_unaligned(output_offset_ptr, output_register);
            }
            num_bytes_per_block
        }

        pub unsafe fn unpack<Output: Sink>(
            compressed: &[u8],
            mut output: Output,
            num_bits: usize,
        ) -> usize {
            let num_bytes_per_block = compute_num_bytes_per_block(num_bits);
            assert!(
                compressed.len() >= num_bytes_per_block,
                "Compressed array seems too small. ({} < {}) ",
                compressed.len(),
                num_bytes_per_block
            );

            let mut input_ptr = compressed.as_ptr() as *const DataType;

            let mask_scalar: u32 = ((1u64 << num_bits) - 1u64) as u32;
            let mask = set1(mask_scalar as i32);

            let mut in_register: DataType = load_unaligned(input_ptr);

            let out_register = op_and(in_register, mask);
            output.process(out_register);

            for i in 1..32 {
                let inner_cursor: usize = (i * num_bits) % 32;
                let inner_capacity: usize = 32 - inner_cursor;

                // LLVM will not emit the shift operand if
                // `inner_cursor` is 0.
                let shifted_in_register = right_shift_32(in_register, inner_cursor as i32);
                let mut out_register: DataType = op_and(shifted_in_register, mask);

                // We consumed our current quadruplets entirely.
                // We therefore read another one.
                if inner_capacity <= num_bits && i != 31 {
                    input_ptr = input_ptr.add(1);
                    in_register = load_unaligned(input_ptr);

                    // This quadruplets is actually cutting one of
                    // our `DataType`. We need to read the next one.
                    if inner_capacity < num_bits {
                        let shifted = left_shift_32(in_register, inner_capacity as i32);
                        let masked = op_and(shifted, mask);
                        out_register = op_or(out_register, masked);
                    }
                }

                output.process(out_register);
            }

            num_bytes_per_block
        }

        pub unsafe fn unpack_32<Output: Sink>(compressed: &[u8], mut output: Output) -> usize {
            let num_bytes_per_block = compute_num_bytes_per_block(32);
            assert!(
                compressed.len() >= num_bytes_per_block,
                "Compressed array seems too small. ({} < {}) ",
                compressed.len(),
                num_bytes_per_block
            );
            let input_ptr = compressed.as_ptr() as *const DataType;
            for i in 0..32 {
                let input_offset_ptr = input_ptr.offset(i as isize);
                let in_register: DataType = load_unaligned(input_offset_ptr);
                output.process(in_register);
            }
            num_bytes_per_block
        }

        pub trait Transformer {
            unsafe fn transform(&mut self, data: DataType) -> DataType;
        }

        struct NoDelta;

        impl Transformer for NoDelta {
            unsafe fn transform(&mut self, current: DataType) -> DataType {
                current
            }
        }

        struct DeltaComputer {
            pub previous: DataType,
        }

        impl Transformer for DeltaComputer {
            unsafe fn transform(&mut self, current: DataType) -> DataType {
                let result = compute_delta(current, self.previous);
                self.previous = current;
                result
            }
        }

        struct StrictDeltaComputer {
            pub previous: DataType,
        }

        impl Transformer for StrictDeltaComputer {
            #[inline]
            unsafe fn transform(&mut self, current: DataType) -> DataType {
                let result = compute_delta(current, self.previous);
                self.previous = current;
                sub(result, set1(1))
            }
        }

        pub trait Sink {
            unsafe fn process(&mut self, data_type: DataType);
        }

        struct Store {
            output_ptr: *mut DataType,
        }

        impl Store {
            fn new(output_ptr: *mut DataType) -> Store {
                Store { output_ptr }
            }
        }

        struct DeltaIntegrate {
            current: DataType,
            output_ptr: *mut DataType,
        }

        impl DeltaIntegrate {
            unsafe fn new(initial: u32, output_ptr: *mut DataType) -> DeltaIntegrate {
                DeltaIntegrate {
                    current: set1(initial as i32),
                    output_ptr,
                }
            }
        }

        impl Sink for DeltaIntegrate {
            #[inline]
            unsafe fn process(&mut self, delta: DataType) {
                self.current = integrate_delta(self.current, delta);
                store_unaligned(self.output_ptr, self.current);
                self.output_ptr = self.output_ptr.add(1);
            }
        }

        struct StrictDeltaIntegrate {
            current: DataType,
            output_ptr: *mut DataType,
        }

        impl StrictDeltaIntegrate {
            unsafe fn new(initial: u32, output_ptr: *mut DataType) -> StrictDeltaIntegrate {
                StrictDeltaIntegrate {
                    current: set1(initial as i32),
                    output_ptr,
                }
            }
        }

        impl Sink for StrictDeltaIntegrate {
            #[inline]
            unsafe fn process(&mut self, delta: DataType) {
                self.current = integrate_delta(self.current, add(delta, set1(1)));
                store_unaligned(self.output_ptr, self.current);
                self.output_ptr = self.output_ptr.add(1);
            }
        }

        impl Sink for Store {
            #[inline]
            unsafe fn process(&mut self, out_register: DataType) {
                store_unaligned(self.output_ptr, out_register);
                self.output_ptr = self.output_ptr.add(1);
            }
        }

        pub struct UnsafeBitPackerImpl;

        impl UnsafeBitPacker for UnsafeBitPackerImpl {
            const BLOCK_LEN: usize = BLOCK_LEN;

            unsafe fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
                if num_bits == 0u8 {
                    return 0;
                }
                if num_bits == 32u8 {
                    return pack_32(decompressed, compressed, NoDelta);
                }
                pack(decompressed, compressed, num_bits as usize, NoDelta)
            }

            unsafe fn compress_sorted(
                initial: u32,
                decompressed: &[u32],
                compressed: &mut [u8],
                num_bits: u8,
            ) -> usize {
                if num_bits == 0u8 {
                    return 0;
                }
                let delta_computer = DeltaComputer {
                    previous: set1(initial as i32),
                };
                if num_bits == 32u8 {
                    return pack_32(decompressed, compressed, delta_computer);
                }
                pack(decompressed, compressed, num_bits as usize, delta_computer)
            }

            unsafe fn compress_strictly_sorted(
                initial: Option<u32>,
                decompressed: &[u32],
                compressed: &mut [u8],
                num_bits: u8,
            ) -> usize {
                let initial = initial.unwrap_or(u32::MAX);
                if num_bits == 0u8 {
                    return 0;
                }
                let delta_computer = StrictDeltaComputer {
                    previous: set1(initial as i32),
                };
                if num_bits == 32u8 {
                    return pack_32(decompressed, compressed, delta_computer);
                }
                pack(decompressed, compressed, num_bits as usize, delta_computer)
            }

            unsafe fn decompress(
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                assert!(
                    decompressed.len() >= BLOCK_LEN,
                    "The output array is not large enough : ({} >= {})",
                    decompressed.len(),
                    BLOCK_LEN
                );
                let output_ptr = decompressed.as_mut_ptr() as *mut DataType;
                let mut output = Store::new(output_ptr);
                if num_bits == 0u8 {
                    let zero = set1(0i32);
                    for _ in 0..32 {
                        output.process(zero);
                    }
                    return 0;
                }
                if num_bits == 32u8 {
                    return unpack_32(compressed, output);
                }
                unpack(compressed, output, num_bits as usize)
            }

            unsafe fn decompress_sorted(
                initial: u32,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                assert!(
                    decompressed.len() >= BLOCK_LEN,
                    "The output array is not large enough : ({} >= {})",
                    decompressed.len(),
                    BLOCK_LEN
                );
                let output_ptr = decompressed.as_mut_ptr() as *mut DataType;
                let mut output = DeltaIntegrate::new(initial, output_ptr);
                if num_bits == 0u8 {
                    let zero = set1(0i32);
                    for _ in 0..32 {
                        output.process(zero);
                    }
                    return 0;
                }
                if num_bits == 32u8 {
                    return unpack_32(compressed, output);
                }
                unpack(compressed, output, num_bits as usize)
            }

            unsafe fn decompress_strictly_sorted(
                initial: Option<u32>,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                assert!(
                    decompressed.len() >= BLOCK_LEN,
                    "The output array is not large enough : ({} >= {})",
                    decompressed.len(),
                    BLOCK_LEN
                );
                let initial = initial.unwrap_or(u32::MAX);
                let output_ptr = decompressed.as_mut_ptr() as *mut DataType;
                let mut output = StrictDeltaIntegrate::new(initial, output_ptr);
                if num_bits == 0u8 {
                    let zero = set1(0i32);
                    for _ in 0..32 {
                        output.process(zero);
                    }
                    return 0;
                }
                if num_bits == 32u8 {
                    return unpack_32(compressed, output);
                }
                unpack(compressed, output, num_bits as usize)
            }

            unsafe fn num_bits(decompressed: &[u32]) -> u8 {
                assert_eq!(
                    decompressed.len(),
                    BLOCK_LEN,
                    "`decompressed`'s len is not `BLOCK_LEN={}`",
                    BLOCK_LEN
                );
                let data: *const DataType = decompressed.as_ptr() as *const DataType;
                let mut accumulator = load_unaligned(data);
                for i in 1..32 {
                    let newvec = load_unaligned(data.add(i));
                    accumulator = op_or(accumulator, newvec);
                }
                most_significant_bit(or_collapse_to_u32(accumulator))
            }

            unsafe fn num_bits_sorted(initial: u32, decompressed: &[u32]) -> u8 {
                let initial_vec = set1(initial as i32);
                let data: *const DataType = decompressed.as_ptr() as *const DataType;
                let first = load_unaligned(data);
                let mut accumulator = compute_delta(load_unaligned(data), initial_vec);
                let mut previous = first;

                for i in 1..31 {
                    let current = load_unaligned(data.add(i));
                    let delta = compute_delta(current, previous);
                    accumulator = op_or(accumulator, delta);
                    previous = current;
                }

                let current = load_unaligned(data.add(31));
                let delta = compute_delta(current, previous);
                accumulator = op_or(accumulator, delta);
                most_significant_bit(or_collapse_to_u32(accumulator))
            }

            unsafe fn num_bits_strictly_sorted(initial: Option<u32>, decompressed: &[u32]) -> u8 {
                let initial = initial.unwrap_or(u32::MAX);
                let initial_vec = set1(initial as i32);
                let data: *const DataType = decompressed.as_ptr() as *const DataType;
                let first = load_unaligned(data);
                let one = set1(1);
                let mut accumulator = sub(compute_delta(load_unaligned(data), initial_vec), one);
                let mut previous = first;

                for i in 1..31 {
                    let current = load_unaligned(data.add(i));
                    let delta = sub(compute_delta(current, previous), one);
                    accumulator = op_or(accumulator, delta);
                    previous = current;
                }

                let current = load_unaligned(data.add(31));
                let delta = sub(compute_delta(current, previous), one);
                accumulator = op_or(accumulator, delta);
                most_significant_bit(or_collapse_to_u32(accumulator))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::UnsafeBitPackerImpl;
            use crate::tests::{test_suite_compress_decompress, DeltaKind};
            use crate::UnsafeBitPacker;

            #[test]
            fn test_num_bits() {
                for num_bits in 0..32 {
                    for pos in 0..32 {
                        let mut vals = [0u32; UnsafeBitPackerImpl::BLOCK_LEN];
                        if num_bits > 0 {
                            vals[pos] = 1 << (num_bits - 1);
                        }
                        assert_eq!(
                            unsafe { UnsafeBitPackerImpl::num_bits(&vals[..]) },
                            num_bits
                        );
                    }
                }
            }

            #[test]
            fn test_bitpacker_nodelta() {
                test_suite_compress_decompress::<UnsafeBitPackerImpl>(DeltaKind::NoDelta);
            }

            #[test]
            fn test_bitpacker_delta() {
                test_suite_compress_decompress::<UnsafeBitPackerImpl>(DeltaKind::Delta);
            }

            #[test]
            fn test_bitpacker_strict_delta() {
                test_suite_compress_decompress::<UnsafeBitPackerImpl>(DeltaKind::StrictDelta);
            }
        }
    };
}
