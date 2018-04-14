extern crate rand;
extern crate test;

use self::rand::{Rng, SeedableRng, XorShiftRng};
use self::test::Bencher;

use super::UnsafeBitPacker;
use super::most_significant_bit;

pub fn generate_array(n: usize, max_num_bits: u8) -> Vec<u32> {
    assert!(max_num_bits <= 32u8);
    let seed: &[u32; 4] = &[1, 2, 3, 4];
    let max_val: u64 = 1u64 << max_num_bits;
    let mut rng: XorShiftRng = XorShiftRng::from_seed(*seed);
    (0..n).map(|_| rng.gen_range(0, max_val) as u32).collect()
}

#[cfg(feature="simd")]
pub(crate) fn test_util_compatible<TLeft: UnsafeBitPacker, TRight: UnsafeBitPacker>(block_len: usize) {
    for num_bits in 0..33 {
        let original = generate_array(block_len, num_bits as u8);
        let mut output_left = vec![0u8; block_len * num_bits / 8];
        let mut output_right = vec![0u8; block_len * num_bits / 8];
        unsafe {
            let num_bits_left = TLeft::num_bits(&original);
            let num_bits_right = TRight::num_bits(&original);
            assert_eq!(num_bits_left, num_bits_right);
            assert_eq!(num_bits_left, num_bits as u8);
            let left_len = TLeft::compress(&original, &mut output_left[..], num_bits_left);
            let right_len = TRight::compress(&original, &mut output_right[..], num_bits_right);
            assert_eq!(left_len, right_len);
            assert_eq!(&output_left[..left_len], &output_right[..right_len]);
        }
    }
}

fn test_util_compress_decompress<TBitPacker: UnsafeBitPacker>(data: &[u32], expected_num_bits: u8) {
    assert_eq!(data.len(), TBitPacker::BLOCK_LEN);

    unsafe {
        let mut original = vec![0u32; data.len()];

        original.copy_from_slice(data);

        let mut compressed = vec![0u8; (TBitPacker::BLOCK_LEN as usize) * 4];
        let mut result = vec![0u32; TBitPacker::BLOCK_LEN as usize];

        let numbits = TBitPacker::num_bits(&original[..]);
        assert_eq!(numbits, expected_num_bits);

        TBitPacker::compress(&original[..], &mut compressed[..], numbits);

        let compressed_len = (numbits as usize) * TBitPacker::BLOCK_LEN / 8;
        for &el in &compressed[compressed_len..] {
            assert_eq!(el, 0u8);
        }

        TBitPacker::decompress(&compressed[..compressed_len], &mut result[..], numbits);

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

fn test_util_compress_decompress_delta<TBitPacker: UnsafeBitPacker>(data: &[u32], expected_num_bits: u8) {
    assert_eq!(data.len(), TBitPacker::BLOCK_LEN);

    for initial in 0u32..2u32 {
        let mut original = data.to_owned();
        integrate_data(initial, &mut original);
        let mut compressed = vec![0u8; (TBitPacker::BLOCK_LEN as usize) * 4];
        let mut result = vec![0u32; TBitPacker::BLOCK_LEN as usize];

        unsafe {
            let numbits = TBitPacker::num_bits_sorted(initial, &original[..]);
            assert_eq!(
                numbits,
                expected_num_bits,
                "Failed identifying max bits. Initial {}. Shifted data {:?}",
                initial,
                &original[..5]
            );

            TBitPacker::compress_sorted(initial, &original[..], &mut compressed[..], numbits);

            let compressed_len = (numbits as usize) * TBitPacker::BLOCK_LEN / 8;
            for &el in &compressed[compressed_len..] {
                assert_eq!(el, 0u8);
            }

            TBitPacker::decompress_sorted(
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

pub(crate) unsafe fn bench_compress_util<TBitPacker: UnsafeBitPacker>(bench: &mut Bencher, num_bits_arr: &[u8]) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, num_bits_arr, DataType::NoDelta);
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    bench.iter(|| {
        let mut offset = 0;
        for i in 0..num_blocks {
            let start = i * TBitPacker::BLOCK_LEN;
            let stop = start + TBitPacker::BLOCK_LEN;
            let block = &original_values[start..stop];
            let num_bits = TBitPacker::num_bits(block);
            let stride = TBitPacker::BLOCK_LEN * (num_bits as usize) / 8;
            num_bits_vec.push(num_bits);
            TBitPacker::compress(block, &mut compress[offset..], num_bits);
            offset += stride;
        }
    });
}

pub(crate) unsafe fn bench_compress_delta_util<TBitPacker: UnsafeBitPacker>(
    bench: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, num_bits_arr, DataType::Delta(3u32));
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    bench.iter(|| {
        let mut offset = 0;
        for i in 0..num_blocks {
            let start = i * TBitPacker::BLOCK_LEN;
            let stop = start + TBitPacker::BLOCK_LEN;
            let block = &original_values[start..stop];
            let num_bits = TBitPacker::num_bits(block);
            let stride = TBitPacker::BLOCK_LEN * (num_bits as usize) / 8;
            num_bits_vec.push(num_bits);
            TBitPacker::compress_sorted(3u32, block, &mut compress[offset..], num_bits);
            offset += stride;
        }
    });
}

pub(crate) unsafe fn bench_decompress_util<TBitPacker>(bench: &mut Bencher, num_bits_arr: &[u8])
    where TBitPacker: UnsafeBitPacker
{
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, &num_bits_arr, DataType::NoDelta);
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let mut offset = 0;
    for i in 0..num_blocks {
        let start = i * TBitPacker::BLOCK_LEN;
        let stop = start + TBitPacker::BLOCK_LEN;
        let block = &original_values[start..stop];
        let num_bits = TBitPacker::num_bits(block);
        num_bits_vec.push(num_bits);
        TBitPacker::compress(block, &mut compressed[offset..], num_bits);
        offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
    }
    let mut result: Vec<u32> = vec![0u32; original_values.len()];
    bench.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            TBitPacker::decompress(&compressed[offset..], dest_block, num_bits);
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

pub(crate) unsafe fn bench_decompress_delta_util<TBitPacker: UnsafeBitPacker>(
    bench: &mut Bencher, num_bits_arr: &[u8]) {
    let num_blocks = num_bits_arr.len();
    bench.bytes = (num_blocks * TBitPacker::BLOCK_LEN * 4) as u64;
    let original_values = create_array(TBitPacker::BLOCK_LEN, &num_bits_arr, DataType::Delta(3u32));
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let mut offset = 0;
    for i in 0..num_blocks {
        let start = i * TBitPacker::BLOCK_LEN;
        let stop = start + TBitPacker::BLOCK_LEN;
        let block = &original_values[start..stop];
        let num_bits = TBitPacker::num_bits(block);
        num_bits_vec.push(num_bits);
        TBitPacker::compress(block, &mut compressed[offset..], num_bits);
        offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
    }
    let mut result: Vec<u32> = vec![0u32; original_values.len()];
    bench.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            TBitPacker::decompress(&compressed[offset..], dest_block, num_bits);
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

pub(crate) fn test_suite_compress_decompress<TBitPacker: UnsafeBitPacker>(delta: bool) {
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

