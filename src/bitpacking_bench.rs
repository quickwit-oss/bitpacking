#[macro_use]
extern crate criterion;
extern crate bitpacking;

use self::criterion::{Bencher, Criterion};

use bitpacking::{BitPacker, BitPacker1x, BitPacker4x, BitPacker8x};
use criterion::Benchmark;
use criterion::Throughput;

const NUM_BLOCKS: usize = 10;

fn integrate_data(initial: u32, data: &mut [u32]) {
    let mut cumul = initial;
    let len = data.len();
    for i in 0..len {
        cumul = cumul.wrapping_add(data[i]);
        data[i] = cumul;
    }
}

fn strict_integrate_data(initial: Option<u32>, data: &mut [u32]) {
    let mut cumul = initial.unwrap_or(u32::MAX);
    let len = data.len();
    for i in 0..len {
        cumul = cumul.wrapping_add(data[i]).wrapping_add(1);
        data[i] = cumul;
    }
}

enum DataType {
    NoDelta,
    Delta(u32),
    StrictDelta(Option<u32>),
}

fn create_array(block_len: usize, num_bits_arr: &[u8], data_type: DataType) -> Vec<u32> {
    let mut original_values = vec![];
    for &num_bits in num_bits_arr {
        let val: u32 = (1u64 << (num_bits as u64) - 1u64) as u32;
        for _ in 0..block_len {
            original_values.push(val);
        }
    }
    match data_type {
        DataType::NoDelta => (),
        DataType::Delta(initial) => integrate_data(initial, &mut original_values),
        DataType::StrictDelta(initial) => strict_integrate_data(initial, &mut original_values),
    }
    original_values
}

fn bench_decompress_util<TBitPacker: BitPacker + 'static>(
    bitpacker: TBitPacker,
    bencher: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    let original_values = create_array(TBitPacker::BLOCK_LEN, &num_bits_arr, DataType::NoDelta);
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
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
    bencher.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            bitpacker.decompress(&compressed[offset..], dest_block, num_bits);
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

fn bench_compress_util<TBitPacker: BitPacker + 'static>(
    bitpacker: TBitPacker,
    bencher: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    let original_values = create_array(TBitPacker::BLOCK_LEN, num_bits_arr, DataType::NoDelta);
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    bencher.iter(|| {
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

fn bench_decompress_delta_util<TBitPacker: BitPacker + 'static>(
    bitpacker: TBitPacker,
    bencher: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    let initial_value = 3u32;
    let original_values = create_array(
        TBitPacker::BLOCK_LEN,
        &num_bits_arr,
        DataType::Delta(initial_value),
    );
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let mut offset = 0;
    for i in 0..num_blocks {
        let start = i * TBitPacker::BLOCK_LEN;
        let stop = start + TBitPacker::BLOCK_LEN;
        let block = &original_values[start..stop];
        let num_bits = bitpacker.num_bits_sorted(initial_value, block);
        num_bits_vec.push(num_bits);
        bitpacker.compress_sorted(initial_value, block, &mut compressed[offset..], num_bits);
        offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
    }
    let mut result: Vec<u32> = vec![0u32; original_values.len()];
    bencher.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            bitpacker.decompress_sorted(initial_value, &compressed[offset..], dest_block, num_bits);
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

fn bench_compress_delta_util<TBitPacker: BitPacker + 'static>(
    bitpacker: TBitPacker,
    bencher: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    let original_values = create_array(TBitPacker::BLOCK_LEN, num_bits_arr, DataType::Delta(3u32));
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    bencher.iter(|| {
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

fn bench_decompress_strict_delta_util<TBitPacker: BitPacker + 'static>(
    bitpacker: TBitPacker,
    bencher: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    let initial_value = Some(3u32);
    let original_values = create_array(
        TBitPacker::BLOCK_LEN,
        &num_bits_arr,
        DataType::StrictDelta(initial_value),
    );
    let mut compressed = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    let mut offset = 0;
    for i in 0..num_blocks {
        let start = i * TBitPacker::BLOCK_LEN;
        let stop = start + TBitPacker::BLOCK_LEN;
        let block = &original_values[start..stop];
        let num_bits = bitpacker.num_bits_strictly_sorted(initial_value, block);
        num_bits_vec.push(num_bits);
        bitpacker.compress_strictly_sorted(
            initial_value,
            block,
            &mut compressed[offset..],
            num_bits,
        );
        offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
    }
    let mut result: Vec<u32> = vec![0u32; original_values.len()];
    bencher.iter(|| {
        let mut offset = 0;
        for (i, num_bits) in num_bits_vec.iter().cloned().enumerate() {
            let dest_block = &mut result[i * TBitPacker::BLOCK_LEN..][..TBitPacker::BLOCK_LEN];
            bitpacker.decompress_strictly_sorted(
                initial_value,
                &compressed[offset..],
                dest_block,
                num_bits,
            );
            offset += (num_bits as usize) * TBitPacker::BLOCK_LEN / 8;
        }
    });
}

fn bench_compress_strict_delta_util<TBitPacker: BitPacker + 'static>(
    bitpacker: TBitPacker,
    bencher: &mut Bencher,
    num_bits_arr: &[u8],
) {
    let num_blocks = num_bits_arr.len();
    let initial = Some(3u32);
    let original_values = create_array(
        TBitPacker::BLOCK_LEN,
        num_bits_arr,
        DataType::StrictDelta(initial),
    );
    let mut compress = vec![0u8; original_values.len() * 4];
    let mut num_bits_vec = Vec::with_capacity(num_bits_arr.len());
    bencher.iter(|| {
        let mut offset = 0;
        for i in 0..num_blocks {
            let start = i * TBitPacker::BLOCK_LEN;
            let stop = start + TBitPacker::BLOCK_LEN;
            let block = &original_values[start..stop];
            let num_bits = bitpacker.num_bits(block);
            let stride = TBitPacker::BLOCK_LEN * (num_bits as usize) / 8;
            num_bits_vec.push(num_bits);
            bitpacker.compress_strictly_sorted(initial, block, &mut compress[offset..], num_bits);
            offset += stride;
        }
    });
}

fn criterion_benchmark_bitpacker<TBitPacker: BitPacker + 'static>(
    name: &str,
    bitpacker: TBitPacker,
    criterion: &mut Criterion,
) {
    for &num_bit in [1u8, 2u8, 24u8, 31u8].iter() {
        let num_bits = [num_bit; NUM_BLOCKS];
        criterion.bench(
            name,
            Benchmark::new(format!("decompress-{}", num_bit).as_str(), move |b| {
                bench_decompress_util::<TBitPacker>(bitpacker, b, &num_bits[..]);
            })
            .throughput(Throughput::Elements(
                (NUM_BLOCKS * TBitPacker::BLOCK_LEN) as u64,
            )),
        );
        criterion.bench(
            name,
            Benchmark::new(format!("decompress-delta-{}", num_bit).as_str(), move |b| {
                bench_decompress_delta_util::<TBitPacker>(bitpacker, b, &num_bits[..]);
            })
            .throughput(Throughput::Elements(
                (NUM_BLOCKS * TBitPacker::BLOCK_LEN) as u64,
            )),
        );
        criterion.bench(
            name,
            Benchmark::new(
                format!("decompress-strict-delta-{}", num_bit).as_str(),
                move |b| {
                    bench_decompress_strict_delta_util::<TBitPacker>(bitpacker, b, &num_bits[..]);
                },
            )
            .throughput(Throughput::Elements(
                (NUM_BLOCKS * TBitPacker::BLOCK_LEN) as u64,
            )),
        );
        criterion.bench(
            name,
            Benchmark::new(format!("compress-{}", num_bit).as_str(), move |b| {
                bench_compress_util::<TBitPacker>(bitpacker, b, &num_bits[..]);
            })
            .throughput(Throughput::Elements(
                (NUM_BLOCKS * TBitPacker::BLOCK_LEN) as u64,
            )),
        );
        criterion.bench(
            name,
            Benchmark::new(format!("compress-delta-{}", num_bit).as_str(), move |b| {
                bench_compress_delta_util::<TBitPacker>(bitpacker, b, &num_bits[..]);
            })
            .throughput(Throughput::Elements(
                (NUM_BLOCKS * TBitPacker::BLOCK_LEN) as u64,
            )),
        );
        criterion.bench(
            name,
            Benchmark::new(
                format!("compress-strict-delta-{}", num_bit).as_str(),
                move |b| {
                    bench_compress_strict_delta_util::<TBitPacker>(bitpacker, b, &num_bits[..]);
                },
            )
            .throughput(Throughput::Elements(
                (NUM_BLOCKS * TBitPacker::BLOCK_LEN) as u64,
            )),
        );
    }
}

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion_benchmark_bitpacker("BitPacker1x", BitPacker1x::new(), criterion);
    criterion_benchmark_bitpacker("BitPacker4x", BitPacker4x::new(), criterion);
    criterion_benchmark_bitpacker("BitPacker8x", BitPacker8x::new(), criterion);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
