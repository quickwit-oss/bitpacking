# Fast Bitpacking algorithms


[![Linux build status](https://travis-ci.org/tantivy-search/bitpacking.svg?branch=master)](https://travis-ci.org/tantivy-search/bitpacking)
[![](http://meritbadge.herokuapp.com/bitpacking)](https://crates.io/crates/bitpacking)

This crate is a **Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp)**.

It makes it possible to compress/decompress :
- sequence of small integers
- sequences of increasing integers

:star: It is fast. Expect > 4 billions integers per seconds.


# Documentation

[Reference documentation](https://docs.rs/bitpacking/0.2.0/bitpacking/)


# What is bitpacking ?

Compressing small integers is a very common problem in search engines, databases, and analytics.

Traditional compression scheme like LZ4 is really suited to address this problem efficiently.
Instead, there are different families of solutions to this problem.

One of the most straightforward and efficient one is `bitpacking` :
- Integers are first grouped into blocks of constant size (e.g. `128` when using the SSE2 implementation).
- If not available implicitely, compute the minimum number of bits `b` that makes it possible to represent all of the integers.
In other words, the smallest `b` such that all integers in the block are stricly smaller than 2<sup>n</sup>.
- The bitpacked representation is then some variation of the concatenation of the integers restricted to their least significant `b`-bits.

For instance, assuming a block of `4`, when encoding `4, 9, 3, 2`. Assuming that the highest value in the block is 9, `b = 4`. All values will then be encoded over 4 bits as follows.

| original number | binary representation |
|:----------------|:----------------------|
| 4               | 0100                  |
| 9               | 1001                  |
| 3               | 0011                  |
| 2               | 0010                  |
| ...             | ...                   |


As a result, each integer of this block will only require 4 bits.


# Choosing between BitPacker1x, BitPacker4x and BitPacker8x.


:warning: `BitPacker1x`, `BitPacker4x`, and `BitPacker8x` produce different formats,
and are incompatible one with another.

`BitPacker4x` and `BitPacker8x` are designed specifically to leverage `SSE3` and `AVX2`
instructions respectively.

It will fallback to a scalar implementation of these format if these instruction sets are not available.
For instance :
- because your compilation target architecture is not `x86_64`
- because the CPU you use is from an older generation

:ok_hand: I recommend using `BitPacker4x` if you are in doubt.

## `BitPacker1x`

BitPacker1x` is what you would expect from a bitpacker.
The integer representation over `b` bits are simply concatenated one
after the other. One block must contain `32 integers`.

## `BitPacker4x`

BitPacker4x bits ordering works in layers of 4 integers. This gives an opportunity
to leverage `SSE3` instructions to encode and decode the stream.
One block must contain `128 integers`.

### BitPacker8x

BitPacker8x bits ordering works in layers of 8 integers. This gives an opportunity
to leverage `AVX2` instructions to encode and decode the stream.
One block must contain `256 integers`.



# Compressing small integers

```rust
extern crate bitpacking;

use bitpacking::{BitPacker4x, BitPacker};


// We compress one block at a time.
assert_eq!(my_data.len(), BitPacker4x::BLOCK_LEN);
let num_bits: u8 = BitPacker4x::num_bits(&my_data);

// A block will be take at most 4 bytes per-integers.
let mut compressed = vec![0u8; 4 * BitPacker4x::BLOCK_LEN];

let compressed_len = BitPacker4x::compress(&my_data, &mut compressed[..], num_bits);
```
# Decompressing small integers

```rust
extern crate bitpacking;
use bitpacking::{BitPacker4x, BitPacker};

let mut decompressed = vec![0u32; BitPacker4x::BLOCK_LEN];
BitPacker4x::decompress(&compressed[..compressed_len], &mut decompressed[..], num_bits);
```

# Benchmark

The following benchmarks have been run on one thread on my laptop's CPU:
Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz.

You can get accurate figures on your hardware by running the following command.

    cargo bench


## BitPacker1x

| operation        | throughput           |
|:-----------------|:---------------------|
| compress         | 1.4 billions int/s   |
| compress_delta   | 0.9 billions int/s   |
| decompress       | 1.8 billions int/s   |
| decompress_delta | 1.4 billions int/s   |

# BitPacker4x

| operation        | throughput           |
|:-----------------|:---------------------|
| compress         | 4.2 billions int/s   |
| compress_delta   | 2.3 billions int/s   |
| decompress       | 5.1 billions int/s   |
| decompress_delta | 4.6 billions int/s   |

# BitPacker8x

| operation        | throughput           |
|:-----------------|:---------------------|
| compress         | 6.15 billions int/s  |
| compress_delta   | 500 millions int/s   |
| decompress       | 5.25 billions int/s  |
| decompress_delta | 4.8 billions int/s   |


# Reference

- [SIMD Compression and the Intersection of Sorted Integers](https://arxiv.org/abs/1401.6399)

# Other crates you might want to check out

- [stream vbyte](https://crates.io/crates/stream-vbyte) A Stream-VByte implementation
- [mayda](https://github.com/fralalonde/mayda) Another crate implementation the same algorithms.
