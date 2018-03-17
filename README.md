# Fast Bitpacking algorithms

This crate is a **Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp)**.
It contains different implementation of integers compression via bitpacking.
Each implementation requires on a different CPU SIMD instruction set,
for state of the art performance.

This crate is used by [`tantivy`](https://github.com/tantivy-search/tantivy).

For instance, with `SSE3`, you can typically expect more than 4 billions ints per seconds.
Check the [Benchmark](#benchmark) for more accurate details.

# What is BitPacking ?

Compressing small integers or increasing integers is a very common problem in search engines, databases, analytics. The latter can be reduced to the earlier via delta-encoding (i.e. Encoding the difference between consecutive difference rather than the integers themselves).
Traditional compression scheme like LZ4 is really not very efficient to address this problem.

There are different families of solutions to this problem. One of the most straightforward and efficient one is `bitpacking` :

- Integers are first grouped into blocks of constant size (e.g. `128` when using the SSE2 implementation).
- Compute the minimum number of bits `b` that makes it possible to represent all of the integers. In other words, the smallest `b` such that all integers in the block are stricly smaller than 2<sup>n</sup>.
- The bitpacked representation is then some variation of the concatenation of the integers restricted to their least significant `b`-bits.

For instance, assuming a block of `4`, when encoding `4, 9, 3, 2`. Assuming that the highest value in the block is 9, `b = 4`. All values will then be encoded over 4 bits as follows.

| original number | binary representation |
|:----------------|:----------------------|
| 4               | 0100                  |
| 9               | 1001                  |
| 3               | 0011                  |
| 2               | 0010                  |

As a result



# Usage

This crate contains different implementation, depending on the instruction set available with your processor.
**The resulting format are not compatible one with each other.**

Currently the following are available :
- scalar: implementation not using any SIMD instruction. This implementation is still very performant
- SSE2:
- AVX


```rust
// Compressing
let num_bits: u8 = SSE3BitPacker::num_bits(&fake_data);
let mut compressed = vec![0u8; (num_bits as usize) * SSE3BitPacker::BLOCK_LEN / 8];
SSE3BitPacker::compress(&fake_data, &mut compressed[..], num_bits);

// Decompressing
let mut decompressed = vec![0u32; SSE3BitPacker::BLOCK_LEN];
SSE3BitPacker::decompress(&compressed, &mut decompressed[..], num_bits);

assert_eq!(&fake_data, &decompressed);
```

# Benchmark


# What kind of software could benefit from this crate?


Make sure to compile with

	RUSTFLAGS="-C target-cpu=native"

# Reference

- [SIMD Compression and the Intersection of Sorted Integers](https://arxiv.org/abs/1401.6399)

# Other crates you might want to check out

- [stream vbyte](https://crates.io/crates/stream-vbyte) A Stream-VByte implementation
- [mayda](https://github.com/fralalonde/mayda) Another crate implementation the same algorithms.
