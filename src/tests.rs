extern crate rand;
extern crate test;

use self::rand::{Rng, SeedableRng, XorShiftRng};
use self::test::Bencher;
use super::BitPacker;
use super::most_significant_bit;
use super::UnsafeBitPacker;

pub fn generate_array(n: usize, max_num_bits: u8) -> Vec<u32> {
    assert!(max_num_bits <= 32u8);
    let seed: &[u32; 4] = &[1, 2, 3, 4];
    let max_val: u64 = 1u64 << max_num_bits;
    let mut rng: XorShiftRng = XorShiftRng::from_seed(*seed);
    (0..n).map(|_| rng.gen_range(0, max_val) as u32).collect()
}

pub(crate) fn test_util_compatible<TLeft: UnsafeBitPacker, TRight: UnsafeBitPacker>(left: &TLeft, right: &TRight, block_len: usize) {
    for num_bits in 0..33 {
        let original = generate_array(block_len, num_bits as u8);
        let mut output_left = vec![0u8; block_len * num_bits / 8];
        let mut output_right = vec![0u8; block_len * num_bits / 8];
        unsafe {
            let num_bits_left = left.num_bits(&original);
            let num_bits_right = right.num_bits(&original);
            assert_eq!(num_bits_left, num_bits_right);
            assert_eq!(num_bits_left, num_bits as u8);
            let left_len = left.compress(&original, &mut output_left[..], num_bits_left);
            let right_len = right.compress(&original, &mut output_right[..], num_bits_right);
            assert_eq!(left_len, right_len);
            assert_eq!(&output_left[..left_len], &output_right[..right_len]);
        }
    }
}

fn test_util_compress_decompress<TBitPacker: BitPacker>(data: &[u32], expected_num_bits: u8) {
    assert_eq!(data.len(), TBitPacker::BLOCK_LEN);

    let mut original = vec![0u32; data.len()];

    original.copy_from_slice(data);

    let mut compressed = vec![0u8; (TBitPacker::BLOCK_LEN as usize) * 4];
    let mut result = vec![0u32; TBitPacker::BLOCK_LEN as usize];

    let bitpacker = TBitPacker::new();
    let numbits = bitpacker.num_bits(&original[..]);
    assert_eq!(numbits, expected_num_bits);

    bitpacker.compress(&original[..], &mut compressed[..], numbits);

    let compressed_len = (numbits as usize) * TBitPacker::BLOCK_LEN / 8;
    for &el in &compressed[compressed_len..] {
        assert_eq!(el, 0u8);
    }

    bitpacker.decompress(&compressed[..compressed_len], &mut result[..], numbits);

    for i in 0..TBitPacker::BLOCK_LEN {
        assert_eq!(
            original[i],
            result[i],
            "Failed at index {}, for expect_num_bits {}, \nORIGINAL {:?} \nRESULT {:?}",
            i,
            expected_num_bits,
            &original[..i + 5],
            &result[..i + 5]
        );
    }
}

fn test_util_compress_decompress_delta<TBitPacker: BitPacker>(data: &[u32], expected_num_bits: u8) {
    assert_eq!(data.len(), TBitPacker::BLOCK_LEN);

    for initial in 0u32..2u32 {
        let mut original = data.to_owned();
        integrate_data(initial, &mut original);
        let mut compressed = vec![0u8; (TBitPacker::BLOCK_LEN as usize) * 4];
        let mut result = vec![0u32; TBitPacker::BLOCK_LEN as usize];

        let bitpacker = TBitPacker::new();

        let numbits = bitpacker.num_bits_sorted(initial, &original[..]);
        assert_eq!(
            numbits,
            expected_num_bits,
            "Failed identifying max bits. Initial {}. Shifted data {:?}",
            initial,
            &original[..5]
        );
        let bitpacker = TBitPacker::new();
        bitpacker.compress_sorted(initial, &original[..], &mut compressed[..], numbits);

        let compressed_len = (numbits as usize) * TBitPacker::BLOCK_LEN / 8;
        for &el in &compressed[compressed_len..] {
            assert_eq!(el, 0u8);
        }

        bitpacker.decompress_sorted(
            initial,
            &compressed[..compressed_len],
            &mut result[..],
            numbits,
        );

        for i in 0..TBitPacker::BLOCK_LEN {
            assert_eq!(
                original[i],
                result[i],
                "Failed at index {}, for expect_num_bits {}, \nORIGINAL {:?} \nRESULT {:?}",
                i,
                expected_num_bits,
                &original[..i + 5],
                &result[..i + 5]
            );
        }
    }
}

enum DataType {
    Delta(u32),
    NoDelta,
}

fn integrate_data(initial: u32, data: &mut [u32]) {
    let mut cumul = initial;
    let len = data.len();
    for i in 0..len {
        cumul = cumul.wrapping_add(data[i]);
        data[i] = cumul;
    }
}

fn create_array(block_len: usize, num_bits_arr: &[u8], data_type: DataType) -> Vec<u32> {
    let mut original_values = vec![];
    for &num_bits in num_bits_arr {
        let val: u32 = (1u64 << (num_bits as u64) - 1u64) as u32;
        for _ in 0..block_len {
            original_values.push(val);
        }
    }
    if let DataType::Delta(initial) = data_type {
        integrate_data(initial, &mut original_values);
    }
    original_values
}

pub unsafe fn bench_compress_util<TBitPacker: BitPacker>(bench: &mut Bencher, num_bits_arr: &[u8]) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, num_bits_arr, DataType::NoDelta);
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let bitpacker = TBitPacker::new();

    bench.iter(|| {
        let mut offset = 0;
        for i in 0..num_blocks {
            let start = i * TBitPacker::BLOCK_LEN;
            let stop = start + TBitPacker::BLOCK_LEN;
            let block = &original_values[start..stop];
            let num_bits = bitpacker.num_bits(block);
            let stride = TBitPacker::BLOCK_LEN * (num_bits as usize) / 8;
            num_bits_vec.push(num_bits);
            bitpacker.compress(block, &mut compress[offset..], num_bits);
            offset += stride;
        }
    });
}

pub unsafe fn bench_compress_delta_util<TBitPacker: BitPacker>(
    bench: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, num_bits_arr, DataType::Delta(3u32));
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let bitpacker = TBitPacker::new();
    bench.iter(|| {
        let mut offset = 0;
        for i in 0..num_blocks {
            let start = i * TBitPacker::BLOCK_LEN;
            let stop = start + TBitPacker::BLOCK_LEN;
            let block = &original_values[start..stop];
            let num_bits = bitpacker.num_bits(block);
            let stride = TBitPacker::BLOCK_LEN * (num_bits as usize) / 8;
            num_bits_vec.push(num_bits);
            bitpacker.compress_sorted(3u32, block, &mut compress[offset..], num_bits);
            offset += stride;
        }
    });
}

pub unsafe fn bench_decompress_util<TBitPacker: BitPacker>(
    bench: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, &num_bits_arr, DataType::NoDelta);
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let bitpacker = TBitPacker::new();
    let mut offset = 0;
    for i in 0..num_blocks {
        let start = i * TBitPacker::BLOCK_LEN;
        let stop = start + TBitPacker::BLOCK_LEN;
        let block = &original_values[start..stop];
        let num_bits = bitpacker.num_bits(block);
        num_bits_vec.push(num_bits);
        bitpacker.compress(block, &mut compressed[offset..], num_bits);
        offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
    }
    let mut result: Vec<u32> = vec![0u32; original_values.len()];
    bench.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            bitpacker.decompress(&compressed[offset..], dest_block, num_bits);
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

pub unsafe fn bench_decompress_delta_util<TBitPacker: BitPacker>(
    bench: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, &num_bits_arr, DataType::Delta(3u32));
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let mut offset = 0;
    let bitpacker = TBitPacker::new();
    for i in 0..num_blocks {
        let start = i * TBitPacker::BLOCK_LEN;
        let stop = start + TBitPacker::BLOCK_LEN;
        let block = &original_values[start..stop];
        let num_bits = bitpacker.num_bits(block);
        num_bits_vec.push(num_bits);
        bitpacker.compress(block, &mut compressed[offset..], num_bits);
        offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
    }
    let mut result: Vec<u32> = vec![0u32; original_values.len()];
    bench.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            bitpacker.decompress(&compressed[offset..], dest_block, num_bits);
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

macro_rules! bench_one {
    ($name:ident, $n:expr, $implementation:ident) => {
        mod $name {

            extern crate test;
            use self::test::Bencher;

            use tests::bench_compress_delta_util;
            use tests::bench_compress_util;
            use tests::bench_decompress_delta_util;
            use tests::bench_decompress_util;
            use $implementation as BenchedBitPacker;

            const NUM_INTS: usize = 1_000;

            #[bench]
            fn bench_decompress(bench: &mut Bencher) {
                unsafe {
                    let num_bits = [$n; NUM_INTS];
                    bench_decompress_util::<BenchedBitPacker>(bench, &num_bits[..]);
                }
            }

            #[bench]
            fn bench_compress(bench: &mut Bencher) {
                unsafe {
                    let num_bits = [$n; NUM_INTS];
                    bench_compress_util::<BenchedBitPacker>(bench, &num_bits[..]);
                }
            }

            #[bench]
            fn bench_compress_delta(bench: &mut Bencher) {
                unsafe {
                    let num_bits = [$n; NUM_INTS];
                    bench_compress_delta_util::<BenchedBitPacker>(bench, &num_bits[..]);
                }
            }

            #[bench]
            fn bench_decompress_delta(bench: &mut Bencher) {
                unsafe {
                    let num_bits = [$n; NUM_INTS];
                    bench_decompress_delta_util::<BenchedBitPacker>(bench, &num_bits[..]);
                }
            }
        }
    };
}

pub fn test_suite_compress_decompress<TBitPacker: BitPacker>(delta: bool) {
    let num_blocks = (1 << 15) / TBitPacker::BLOCK_LEN;
    let n = num_blocks * TBitPacker::BLOCK_LEN;
    for num_bits in 0u8..32u8 {
        let original = generate_array(n, num_bits);
        for i in 0..num_blocks {
            let block = &original[i * TBitPacker::BLOCK_LEN..(i + 1) * TBitPacker::BLOCK_LEN];
            let computed_num_bits = block
                .iter()
                .cloned()
                .map(most_significant_bit)
                .max()
                .unwrap_or(0u8);
            assert!(computed_num_bits <= num_bits);
            if delta {
                test_util_compress_decompress_delta::<TBitPacker>(block, computed_num_bits);
            } else {
                test_util_compress_decompress::<TBitPacker>(block, computed_num_bits);
            }
        }
    }
}

#[macro_export]
macro_rules! bench_suite {
    ($implementation:ident) => {
        bench_one!(bench_num_bits_01, 1, $implementation);
        bench_one!(bench_num_bits_02, 2, $implementation);
        bench_one!(bench_num_bits_03, 3, $implementation);
        bench_one!(bench_num_bits_04, 4, $implementation);
        bench_one!(bench_num_bits_05, 5, $implementation);
        bench_one!(bench_num_bits_06, 6, $implementation);
        bench_one!(bench_num_bits_07, 7, $implementation);
        bench_one!(bench_num_bits_08, 8, $implementation);
        bench_one!(bench_num_bits_09, 9, $implementation);
        bench_one!(bench_num_bits_10, 10, $implementation);
        bench_one!(bench_num_bits_11, 11, $implementation);
        bench_one!(bench_num_bits_12, 12, $implementation);
        bench_one!(bench_num_bits_13, 13, $implementation);
        bench_one!(bench_num_bits_14, 14, $implementation);
        bench_one!(bench_num_bits_15, 15, $implementation);
        bench_one!(bench_num_bits_16, 16, $implementation);
        bench_one!(bench_num_bits_17, 17, $implementation);
        bench_one!(bench_num_bits_18, 18, $implementation);
        bench_one!(bench_num_bits_19, 19, $implementation);
        bench_one!(bench_num_bits_20, 20, $implementation);
        bench_one!(bench_num_bits_21, 21, $implementation);
        bench_one!(bench_num_bits_22, 22, $implementation);
        bench_one!(bench_num_bits_23, 23, $implementation);
        bench_one!(bench_num_bits_24, 24, $implementation);
        bench_one!(bench_num_bits_25, 25, $implementation);
        bench_one!(bench_num_bits_26, 26, $implementation);
        bench_one!(bench_num_bits_27, 27, $implementation);
        bench_one!(bench_num_bits_28, 28, $implementation);
        bench_one!(bench_num_bits_29, 29, $implementation);
        bench_one!(bench_num_bits_30, 30, $implementation);
        bench_one!(bench_num_bits_31, 31, $implementation);
        bench_one!(bench_num_bits_32, 32, $implementation);
    };
}
