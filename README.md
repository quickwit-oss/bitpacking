# Fast Bitpacking algorithms

This crate is a **Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp)**.
It contains different implementation of integers compression via bitpacking.
Each implementation requires on a different CPU SIMD instruction set,
for state of the art performance.

This crate is used by [`tantivy`](https://github.com/tantivy-search/tantivy).

# What is BitPacking ?

Compressing small integers is a very common
problem in search engines, databases, and analytics.

Compressing increasing integers can also be reduced to compressing
small integer via delta-encoding : instead of encoding the values
 themselves, one encodes the gaps between consecutive values.

Traditional compression schemes like LZ4 aren't really suited to address this problem efficiently.
Instead, there are different families of solutions to this problem.

One of the most straightforward and efficient ones is `bitpacking` :
- Integers are first grouped into blocks of constant size (e.g. `128` when using the SSE2 implementation).
- If not available implicitely, compute the minimum number of bits `b` that makes it possible to represent all of the integers.
In other words, the smallest `b` such that all integers in the block are stricly smaller than 2<sup>b</sup>.

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



# Usage

This crate contains different implementations for bitpacking depending on the instruction set available with your processor.
**The resulting formats are not compatible one with each other.**

Currently the following are available :
- scalar: implementation not using any SIMD instruction. A block has a size of 32.
This implementation is still more performant than naive solutions.
- SSE3: bitpacking 4 integer at once. (block size of 128). *Requires the sse3 feature to be enabled. This feature is enabled by default.*
- AVX: butpacking 8 integers at once. (block size of 256). *Delta integration is comparatively expensive*. Requires to
enable the avx feature.

I recommend using the SSE3 implementation if you are not sure what you are doing and you are targetting x86_64 CPUs that have been produced after 2006.

Make sure to compile with

	RUSTFLAGS="-C target-cpu=native"


# Compressing small integers

```rust
extern crate bitpacking;
use bitpacking::{SSE3BitPacker, BitPacker};

// ...

// We compress one block at a time.
assert_eq!(my_data.len(), SSE3BitPacker::BLOCK_LEN);
// Each `BitPacker` has a method to efficiently compute the minimum number of bits required
// to represent all of the integers in the block.
let num_bits: u8 = SSE3BitPacker::num_bits(&my_data);

// The compressed block len is straightforward : BLOCK_LEN (here 128) * num_bits / 8.
let compressed_len = SSE3BitPacker::compressed_block_size(num_bits);
let mut compressed = vec![0u8; compressed_len];

// compressing
SSE3BitPacker::compress(&fake_data, &mut compressed[..], num_bits);
```

# Decompressing small integers

```rust
extern crate bitpacking;
use bitpacking::{SSE3BitPacker, BitPacker};

// ...
let mut decompressed = vec![0u32; SSE3BitPacker::BLOCK_LEN];
SSE3BitPacker::decompress(&compressed, &mut decompressed[..], num_bits);

assert_eq!(&fake_data, &decompressed);
```

# Compressing an increasing sequence of integers


# Decompressing an increasing sequence of integers



# Benchmark

The following benchmarks have been run on one thread on my laptop's CPU:
Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz.

Throughput figures are expressed in term of decompressed 32-bits integers.
To get figures expressed in integers/s, divide by 4.

You can reproduce it on your hardware by running the following command.

    RUSTFLAGS="-C target-cpu=native" cargo bench


## Scalar


| num bits | operation        | throughput |
|:---------|:-----------------|:-----------|
| 01       | compress         | 7158 MB/s  |
| 01       | compress_delta   | 4674 MB/s  |
| 01       | decompress       | 12891 MB/s |
| 01       | decompress_delta | 7634 MB/s  |
| 02       | compress         | 7039 MB/s  |
| 02       | compress_delta   | 4933 MB/s  |
| 02       | decompress       | 9902 MB/s  |
| 02       | decompress_delta | 8195 MB/s  |
| 03       | compress         | 7026 MB/s  |
| 03       | compress_delta   | 4721 MB/s  |
| 03       | decompress       | 11316 MB/s |
| 03       | decompress_delta | 7659 MB/s  |
| 04       | compress         | 7889 MB/s  |
| 04       | compress_delta   | 4560 MB/s  |
| 04       | decompress       | 9303 MB/s  |
| 04       | decompress_delta | 7516 MB/s  |
| 05       | compress         | 7130 MB/s  |
| 05       | compress_delta   | 4432 MB/s  |
| 05       | decompress       | 10232 MB/s |
| 05       | decompress_delta | 7309 MB/s  |
| 06       | compress         | 6973 MB/s  |
| 06       | compress_delta   | 4424 MB/s  |
| 06       | decompress       | 10725 MB/s |
| 06       | decompress_delta | 7343 MB/s  |
| 07       | compress         | 6535 MB/s  |
| 07       | compress_delta   | 4274 MB/s  |
| 07       | decompress       | 9541 MB/s  |
| 07       | decompress_delta | 7101 MB/s  |
| 08       | compress         | 7659 MB/s  |
| 08       | compress_delta   | 4250 MB/s  |
| 08       | decompress       | 9170 MB/s  |
| 08       | decompress_delta | 7030 MB/s  |
| 09       | compress         | 6711 MB/s  |
| 09       | compress_delta   | 4155 MB/s  |
| 09       | decompress       | 8030 MB/s  |
| 09       | decompress_delta | 6857 MB/s  |
| 10       | compress         | 6539 MB/s  |
| 10       | compress_delta   | 4216 MB/s  |
| 10       | decompress       | 8449 MB/s  |
| 10       | decompress_delta | 7173 MB/s  |
| 11       | compress         | 6452 MB/s  |
| 11       | compress_delta   | 4112 MB/s  |
| 11       | decompress       | 7858 MB/s  |
| 11       | decompress_delta | 7480 MB/s  |
| 12       | compress         | 6760 MB/s  |
| 12       | compress_delta   | 4002 MB/s  |
| 12       | decompress       | 8541 MB/s  |
| 12       | decompress_delta | 8665 MB/s  |
| 13       | compress         | 6101 MB/s  |
| 13       | compress_delta   | 3937 MB/s  |
| 13       | decompress       | 7602 MB/s  |
| 13       | decompress_delta | 8686 MB/s  |
| 14       | compress         | 6107 MB/s  |
| 14       | compress_delta   | 3911 MB/s  |
| 14       | decompress       | 7856 MB/s  |
| 14       | decompress_delta | 8729 MB/s  |
| 15       | compress         | 5713 MB/s  |
| 15       | compress_delta   | 3834 MB/s  |
| 15       | decompress       | 7404 MB/s  |
| 15       | decompress_delta | 9561 MB/s  |
| 16       | compress         | 7788 MB/s  |
| 16       | compress_delta   | 3779 MB/s  |
| 16       | decompress       | 8901 MB/s  |
| 16       | decompress_delta | 10075 MB/s |
| 17       | compress         | 5738 MB/s  |
| 17       | compress_delta   | 3723 MB/s  |
| 17       | decompress       | 7225 MB/s  |
| 17       | decompress_delta | 10050 MB/s |
| 18       | compress         | 5755 MB/s  |
| 18       | compress_delta   | 3830 MB/s  |
| 18       | decompress       | 7421 MB/s  |
| 18       | decompress_delta | 11767 MB/s |
| 19       | compress         | 5491 MB/s  |
| 19       | compress_delta   | 3803 MB/s  |
| 19       | decompress       | 7110 MB/s  |
| 19       | decompress_delta | 11627 MB/s |
| 20       | compress         | 5601 MB/s  |
| 20       | compress_delta   | 3758 MB/s  |
| 20       | decompress       | 7423 MB/s  |
| 20       | decompress_delta | 11571 MB/s |
| 21       | compress         | 5078 MB/s  |
| 21       | compress_delta   | 3667 MB/s  |
| 21       | decompress       | 6911 MB/s  |
| 21       | decompress_delta | 11573 MB/s |
| 22       | compress         | 5127 MB/s  |
| 22       | compress_delta   | 3645 MB/s  |
| 22       | decompress       | 6996 MB/s  |
| 22       | decompress_delta | 11565 MB/s |
| 23       | compress         | 4888 MB/s  |
| 23       | compress_delta   | 3633 MB/s  |
| 23       | decompress       | 6730 MB/s  |
| 23       | decompress_delta | 11555 MB/s |
| 24       | compress         | 5450 MB/s  |
| 24       | compress_delta   | 3627 MB/s  |
| 24       | decompress       | 7566 MB/s  |
| 24       | decompress_delta | 12292 MB/s |
| 25       | compress         | 4588 MB/s  |
| 25       | compress_delta   | 3610 MB/s  |
| 25       | decompress       | 7869 MB/s  |
| 25       | decompress_delta | 12811 MB/s |
| 26       | compress         | 5322 MB/s  |
| 26       | compress_delta   | 3843 MB/s  |
| 26       | decompress       | 10381 MB/s |
| 26       | decompress_delta | 13311 MB/s |
| 27       | compress         | 5104 MB/s  |
| 27       | compress_delta   | 3801 MB/s  |
| 27       | decompress       | 8799 MB/s  |
| 27       | decompress_delta | 13320 MB/s |
| 28       | compress         | 5582 MB/s  |
| 28       | compress_delta   | 3787 MB/s  |
| 28       | decompress       | 8754 MB/s  |
| 28       | decompress_delta | 13611 MB/s |
| 29       | compress         | 5021 MB/s  |
| 29       | compress_delta   | 3797 MB/s  |
| 29       | decompress       | 10984 MB/s |
| 29       | decompress_delta | 14279 MB/s |
| 30       | compress         | 5988 MB/s  |
| 30       | compress_delta   | 3793 MB/s  |
| 30       | decompress       | 10706 MB/s |
| 30       | decompress_delta | 14044 MB/s |
| 31       | compress         | 6851 MB/s  |
| 31       | compress_delta   | 3800 MB/s  |
| 31       | decompress       | 10099 MB/s |
| 31       | decompress_delta | 13228 MB/s |
| 32       | compress         | 9590 MB/s  |
| 32       | compress_delta   | 3790 MB/s  |
| 32       | decompress       | 14084 MB/s |
| 32       | decompress_delta | 14160 MB/s |

## SSE3


| num bits | operation        | throughput |
|:---------|:-----------------|:-----------|
| 01       | compress         | 27429 MB/s |
| 01       | compress_delta   | 15311 MB/s |
| 01       | decompress       | 25522 MB/s |
| 01       | decompress_delta | 22767 MB/s |
| 02       | compress         | 27447 MB/s |
| 02       | compress_delta   | 14747 MB/s |
| 02       | decompress       | 25496 MB/s |
| 02       | decompress_delta | 22633 MB/s |
| 03       | compress         | 26741 MB/s |
| 03       | compress_delta   | 14575 MB/s |
| 03       | decompress       | 25041 MB/s |
| 03       | decompress_delta | 22465 MB/s |
| 04       | compress         | 27874 MB/s |
| 04       | compress_delta   | 14296 MB/s |
| 04       | decompress       | 24867 MB/s |
| 04       | decompress_delta | 22385 MB/s |
| 05       | compress         | 26279 MB/s |
| 05       | compress_delta   | 13955 MB/s |
| 05       | decompress       | 24736 MB/s |
| 05       | decompress_delta | 22047 MB/s |
| 06       | compress         | 25931 MB/s |
| 06       | compress_delta   | 13599 MB/s |
| 06       | decompress       | 24549 MB/s |
| 06       | decompress_delta | 22050 MB/s |
| 07       | compress         | 25768 MB/s |
| 07       | compress_delta   | 13298 MB/s |
| 07       | decompress       | 24335 MB/s |
| 07       | decompress_delta | 21691 MB/s |
| 08       | compress         | 27379 MB/s |
| 08       | compress_delta   | 13034 MB/s |
| 08       | decompress       | 24332 MB/s |
| 08       | decompress_delta | 21695 MB/s |
| 09       | compress         | 25371 MB/s |
| 09       | compress_delta   | 12695 MB/s |
| 09       | decompress       | 23869 MB/s |
| 09       | decompress_delta | 21226 MB/s |
| 10       | compress         | 25125 MB/s |
| 10       | compress_delta   | 12371 MB/s |
| 10       | decompress       | 24042 MB/s |
| 10       | decompress_delta | 21315 MB/s |
| 11       | compress         | 24514 MB/s |
| 11       | compress_delta   | 12261 MB/s |
| 11       | decompress       | 23711 MB/s |
| 11       | decompress_delta | 21033 MB/s |
| 12       | compress         | 24945 MB/s |
| 12       | compress_delta   | 11972 MB/s |
| 12       | decompress       | 23250 MB/s |
| 12       | decompress_delta | 21076 MB/s |
| 13       | compress         | 23880 MB/s |
| 13       | compress_delta   | 11831 MB/s |
| 13       | decompress       | 23194 MB/s |
| 13       | decompress_delta | 20918 MB/s |
| 14       | compress         | 22706 MB/s |
| 14       | compress_delta   | 11497 MB/s |
| 14       | decompress       | 22895 MB/s |
| 14       | decompress_delta | 20818 MB/s |
| 15       | compress         | 23132 MB/s |
| 15       | compress_delta   | 11276 MB/s |
| 15       | decompress       | 22831 MB/s |
| 15       | decompress_delta | 20601 MB/s |
| 16       | compress         | 24871 MB/s |
| 16       | compress_delta   | 11052 MB/s |
| 16       | decompress       | 22897 MB/s |
| 16       | decompress_delta | 20039 MB/s |
| 17       | compress         | 21828 MB/s |
| 17       | compress_delta   | 11011 MB/s |
| 17       | decompress       | 22462 MB/s |
| 17       | decompress_delta | 20627 MB/s |
| 18       | compress         | 22332 MB/s |
| 18       | compress_delta   | 10691 MB/s |
| 18       | decompress       | 22411 MB/s |
| 18       | decompress_delta | 20233 MB/s |
| 19       | compress         | 22098 MB/s |
| 19       | compress_delta   | 10934 MB/s |
| 19       | decompress       | 22303 MB/s |
| 19       | decompress_delta | 20316 MB/s |
| 20       | compress         | 22037 MB/s |
| 20       | compress_delta   | 10939 MB/s |
| 20       | decompress       | 22056 MB/s |
| 20       | decompress_delta | 20417 MB/s |
| 21       | compress         | 21479 MB/s |
| 21       | compress_delta   | 10871 MB/s |
| 21       | decompress       | 21884 MB/s |
| 21       | decompress_delta | 20543 MB/s |
| 22       | compress         | 21380 MB/s |
| 22       | compress_delta   | 10843 MB/s |
| 22       | decompress       | 21717 MB/s |
| 22       | decompress_delta | 20185 MB/s |
| 23       | compress         | 20885 MB/s |
| 23       | compress_delta   | 10979 MB/s |
| 23       | decompress       | 21474 MB/s |
| 23       | decompress_delta | 20227 MB/s |
| 24       | compress         | 21154 MB/s |
| 24       | compress_delta   | 10675 MB/s |
| 24       | decompress       | 21414 MB/s |
| 24       | decompress_delta | 20435 MB/s |
| 25       | compress         | 20021 MB/s |
| 25       | compress_delta   | 10754 MB/s |
| 25       | decompress       | 21175 MB/s |
| 25       | decompress_delta | 19418 MB/s |
| 26       | compress         | 19996 MB/s |
| 26       | compress_delta   | 10881 MB/s |
| 26       | decompress       | 21165 MB/s |
| 26       | decompress_delta | 20735 MB/s |
| 27       | compress         | 19522 MB/s |
| 27       | compress_delta   | 10780 MB/s |
| 27       | decompress       | 21109 MB/s |
| 27       | decompress_delta | 20440 MB/s |
| 28       | compress         | 19268 MB/s |
| 28       | compress_delta   | 10871 MB/s |
| 28       | decompress       | 20985 MB/s |
| 28       | decompress_delta | 19330 MB/s |
| 29       | compress         | 18665 MB/s |
| 29       | compress_delta   | 10769 MB/s |
| 29       | decompress       | 20757 MB/s |
| 29       | decompress_delta | 20545 MB/s |
| 30       | compress         | 18808 MB/s |
| 30       | compress_delta   | 10528 MB/s |
| 30       | decompress       | 20605 MB/s |
| 30       | decompress_delta | 19338 MB/s |
| 31       | compress         | 18356 MB/s |
| 31       | compress_delta   | 10750 MB/s |
| 31       | decompress       | 20368 MB/s |
| 31       | decompress_delta | 19186 MB/s |
| 32       | compress         | 18698 MB/s |
| 32       | compress_delta   | 10600 MB/s |
| 32       | decompress       | 20803 MB/s |
| 32       | decompress_delta | 20611 MB/s |



## AVX2

Requires to compile with the `avx2` feature.


| num bits | operation        | throughput |
|:---------|:-----------------|:-----------|
| 01       | compress         | 28138 MB/s |
| 01       | compress_delta   | 9466 MB/s  |
| 01       | decompress       | 23299 MB/s |
| 01       | decompress_delta | 18810 MB/s |
| 02       | compress         | 27984 MB/s |
| 02       | compress_delta   | 9223 MB/s  |
| 02       | decompress       | 22833 MB/s |
| 02       | decompress_delta | 18594 MB/s |
| 03       | compress         | 26452 MB/s |
| 03       | compress_delta   | 9710 MB/s  |
| 03       | decompress       | 22550 MB/s |
| 03       | decompress_delta | 18202 MB/s |
| 04       | compress         | 26241 MB/s |
| 04       | compress_delta   | 9304 MB/s  |
| 04       | decompress       | 22350 MB/s |
| 04       | decompress_delta | 18174 MB/s |
| 05       | compress         | 26244 MB/s |
| 05       | compress_delta   | 9192 MB/s  |
| 05       | decompress       | 22049 MB/s |
| 05       | decompress_delta | 17964 MB/s |
| 06       | compress         | 26103 MB/s |
| 06       | compress_delta   | 8817 MB/s  |
| 06       | decompress       | 21621 MB/s |
| 06       | decompress_delta | 17705 MB/s |
| 07       | compress         | 24917 MB/s |
| 07       | compress_delta   | 9598 MB/s  |
| 07       | decompress       | 21288 MB/s |
| 07       | decompress_delta | 17634 MB/s |
| 08       | compress         | 25899 MB/s |
| 08       | compress_delta   | 9189 MB/s  |
| 08       | decompress       | 20944 MB/s |
| 08       | decompress_delta | 17459 MB/s |
| 09       | compress         | 25044 MB/s |
| 09       | compress_delta   | 9223 MB/s  |
| 09       | decompress       | 20849 MB/s |
| 09       | decompress_delta | 17283 MB/s |
| 10       | compress         | 23441 MB/s |
| 10       | compress_delta   | 8784 MB/s  |
| 10       | decompress       | 20551 MB/s |
| 10       | decompress_delta | 17104 MB/s |
| 11       | compress         | 23543 MB/s |
| 11       | compress_delta   | 9703 MB/s  |
| 11       | decompress       | 20255 MB/s |
| 11       | decompress_delta | 16557 MB/s |
| 12       | compress         | 24130 MB/s |
| 12       | compress_delta   | 9019 MB/s  |
| 12       | decompress       | 19993 MB/s |
| 12       | decompress_delta | 16815 MB/s |
| 13       | compress         | 23002 MB/s |
| 13       | compress_delta   | 9300 MB/s  |
| 13       | decompress       | 19736 MB/s |
| 13       | decompress_delta | 16541 MB/s |
| 14       | compress         | 22253 MB/s |
| 14       | compress_delta   | 9064 MB/s  |
| 14       | decompress       | 19407 MB/s |
| 14       | decompress_delta | 16421 MB/s |
| 15       | compress         | 22598 MB/s |
| 15       | compress_delta   | 9709 MB/s  |
| 15       | decompress       | 19180 MB/s |
| 15       | decompress_delta | 16256 MB/s |
| 16       | compress         | 22947 MB/s |
| 16       | compress_delta   | 9645 MB/s  |
| 16       | decompress       | 19071 MB/s |
| 16       | decompress_delta | 16245 MB/s |
| 17       | compress         | 21449 MB/s |
| 17       | compress_delta   | 9904 MB/s  |
| 17       | decompress       | 18418 MB/s |
| 17       | decompress_delta | 16111 MB/s |
| 18       | compress         | 20659 MB/s |
| 18       | compress_delta   | 9617 MB/s  |
| 18       | decompress       | 18375 MB/s |
| 18       | decompress_delta | 16317 MB/s |
| 19       | compress         | 20426 MB/s |
| 19       | compress_delta   | 9831 MB/s  |
| 19       | decompress       | 18272 MB/s |
| 19       | decompress_delta | 16169 MB/s |
| 20       | compress         | 19645 MB/s |
| 20       | compress_delta   | 9975 MB/s  |
| 20       | decompress       | 18038 MB/s |
| 20       | decompress_delta | 16248 MB/s |
| 21       | compress         | 19745 MB/s |
| 21       | compress_delta   | 9749 MB/s  |
| 21       | decompress       | 17785 MB/s |
| 21       | decompress_delta | 16221 MB/s |
| 22       | compress         | 19493 MB/s |
| 22       | compress_delta   | 10282 MB/s |
| 22       | decompress       | 17702 MB/s |
| 22       | decompress_delta | 16200 MB/s |
| 23       | compress         | 18843 MB/s |
| 23       | compress_delta   | 9994 MB/s  |
| 23       | decompress       | 17524 MB/s |
| 23       | decompress_delta | 15987 MB/s |
| 24       | compress         | 18553 MB/s |
| 24       | compress_delta   | 10976 MB/s |
| 24       | decompress       | 17420 MB/s |
| 24       | decompress_delta | 15688 MB/s |
| 25       | compress         | 17683 MB/s |
| 25       | compress_delta   | 11073 MB/s |
| 25       | decompress       | 17244 MB/s |
| 25       | decompress_delta | 16333 MB/s |
| 26       | compress         | 17399 MB/s |
| 26       | compress_delta   | 10946 MB/s |
| 26       | decompress       | 17070 MB/s |
| 26       | decompress_delta | 16033 MB/s |
| 27       | compress         | 17283 MB/s |
| 27       | compress_delta   | 11372 MB/s |
| 27       | decompress       | 16837 MB/s |
| 27       | decompress_delta | 15853 MB/s |
| 28       | compress         | 17099 MB/s |
| 28       | compress_delta   | 11061 MB/s |
| 28       | decompress       | 16727 MB/s |
| 28       | decompress_delta | 15881 MB/s |
| 29       | compress         | 16420 MB/s |
| 29       | compress_delta   | 11029 MB/s |
| 29       | decompress       | 16506 MB/s |
| 29       | decompress_delta | 15669 MB/s |
| 30       | compress         | 16334 MB/s |
| 30       | compress_delta   | 10821 MB/s |
| 30       | decompress       | 16360 MB/s |
| 30       | decompress_delta | 16330 MB/s |
| 31       | compress         | 15869 MB/s |
| 31       | compress_delta   | 11153 MB/s |
| 31       | decompress       | 16334 MB/s |
| 31       | decompress_delta | 16385 MB/s |
| 32       | compress         | 16059 MB/s |
| 32       | compress_delta   | 11042 MB/s |
| 32       | decompress       | 16174 MB/s |
| 32       | decompress_delta | 16046 MB/s |


# Reference

- [SIMD Compression and the Intersection of Sorted Integers](https://arxiv.org/abs/1401.6399)

# Other crates you might want to check out

- [stream vbyte](https://crates.io/crates/stream-vbyte) A Stream-VByte implementation
- [mayda](https://github.com/fralalonde/mayda) Another crate implementation the same algorithms.
