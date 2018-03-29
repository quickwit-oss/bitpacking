# Fast Bitpacking algorithms

This crate is a **Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp)**.
It contains different implementation of integers compression via bitpacking.
Each implementation requires on a different CPU SIMD instruction set,
for state of the art performance.

This crate is used by [`tantivy`](https://github.com/tantivy-search/tantivy).

For instance, with `SSE3`, you can typically expect more than 4 billions ints per seconds.
Check the [Benchmark](#benchmark) for more accurate details.

# What is BitPacking ?

Compressing small integers is a very common
problem in search engines, databases, and analytics.

Compressing increasing integers can also be reduced to compressing
small integer via delta-encoding : instead of encoding the values
 themselves, one encodes the gaps between consecutive values.

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



# Usage

This crate contains different implementation for bitpacking depending on the instruction set available with your processor.
**The resulting format are not compatible one with each other.**

Currently the following are available :
- scalar: implementation not using any SIMD instruction. A block has a size of 32.
This implementation is still more performant naive solutions.
- SSE3: bitpacking 4 integer at once. (block size of 128). *Requires the sse3 feature to be enabled. This feature is enabled by default.*
- AVX: butpacking 8 integers at once. (block size of 256). *Delta integration is comparatively expensive*. Requires to
enable the avx feature.

I recommend using the SSE3 implementation if you are not sure what you are doing and you are targetting x86_64 CPUs that have been produced after 2006.


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

| num bits | operation         | throughput                                          |
|:-- ------|:------------------|:----------------------------------------------------|
|  01      | compress          |      17,880 ns/iter (+/- 361) = 7158 MB/s           |
|  01      | compress_delta    |      27,382 ns/iter (+/- 407) = 4674 MB/s           |
|  01      | decompress        |       9,929 ns/iter (+/- 62) = 12891 MB/s           |
|  01      | decompress_delta  |      16,766 ns/iter (+/- 53) = 7634 MB/s            |
|  02      | compress          |      18,182 ns/iter (+/- 315) = 7039 MB/s           |
|  02      | compress_delta    |      25,947 ns/iter (+/- 401) = 4933 MB/s           |
|  02      | decompress        |      12,926 ns/iter (+/- 184) = 9902 MB/s           |
|  02      | decompress_delta  |      15,619 ns/iter (+/- 34) = 8195 MB/s            |
|  03      | compress          |      18,218 ns/iter (+/- 282) = 7026 MB/s           |
|  03      | compress_delta    |      27,111 ns/iter (+/- 433) = 4721 MB/s           |
|  03      | decompress        |      11,311 ns/iter (+/- 17) = 11316 MB/s           |
|  03      | decompress_delta  |      16,711 ns/iter (+/- 39) = 7659 MB/s            |
|  04      | compress          |      16,225 ns/iter (+/- 262) = 7889 MB/s           |
|  04      | compress_delta    |      28,068 ns/iter (+/- 573) = 4560 MB/s           |
|  04      | decompress        |      13,759 ns/iter (+/- 525) = 9303 MB/s           |
|  04      | decompress_delta  |      17,030 ns/iter (+/- 105) = 7516 MB/s           |
|  05      | compress          |      17,950 ns/iter (+/- 324) = 7130 MB/s           |
|  05      | compress_delta    |      28,878 ns/iter (+/- 278) = 4432 MB/s           |
|  05      | decompress        |      12,509 ns/iter (+/- 75) = 10232 MB/s           |
|  05      | decompress_delta  |      17,512 ns/iter (+/- 18) = 7309 MB/s            |
|  06      | compress          |      18,355 ns/iter (+/- 286) = 6973 MB/s           |
|  06      | compress_delta    |      28,930 ns/iter (+/- 419) = 4424 MB/s           |
|  06      | decompress        |      11,934 ns/iter (+/- 13) = 10725 MB/s           |
|  06      | decompress_delta  |      17,431 ns/iter (+/- 54) = 7343 MB/s            |
|  07      | compress          |      19,585 ns/iter (+/- 313) = 6535 MB/s           |
|  07      | compress_delta    |      29,945 ns/iter (+/- 800) = 4274 MB/s           |
|  07      | decompress        |      13,415 ns/iter (+/- 454) = 9541 MB/s           |
|  07      | decompress_delta  |      18,025 ns/iter (+/- 64) = 7101 MB/s            |
|  08      | compress          |      16,711 ns/iter (+/- 277) = 7659 MB/s           |
|  08      | compress_delta    |      30,116 ns/iter (+/- 544) = 4250 MB/s           |
|  08      | decompress        |      13,958 ns/iter (+/- 106) = 9170 MB/s           |
|  08      | decompress_delta  |      18,206 ns/iter (+/- 143) = 7030 MB/s           |
|  09      | compress          |      19,072 ns/iter (+/- 343) = 6711 MB/s           |
|  09      | compress_delta    |      30,802 ns/iter (+/- 443) = 4155 MB/s           |
|  09      | decompress        |      15,939 ns/iter (+/- 496) = 8030 MB/s           |
|  09      | decompress_delta  |      18,666 ns/iter (+/- 42) = 6857 MB/s            |
|  10      | compress          |      19,572 ns/iter (+/- 325) = 6539 MB/s           |
|  10      | compress_delta    |      30,357 ns/iter (+/- 453) = 4216 MB/s           |
|  10      | decompress        |      15,149 ns/iter (+/- 518) = 8449 MB/s           |
|  10      | decompress_delta  |      17,844 ns/iter (+/- 39) = 7173 MB/s            |
|  11      | compress          |      19,836 ns/iter (+/- 425) = 6452 MB/s           |
|  11      | compress_delta    |      31,123 ns/iter (+/- 475) = 4112 MB/s           |
|  11      | decompress        |      16,289 ns/iter (+/- 513) = 7858 MB/s           |
|  11      | decompress_delta  |      17,111 ns/iter (+/- 62) = 7480 MB/s            |
|  12      | compress          |      18,934 ns/iter (+/- 342) = 6760 MB/s           |
|  12      | compress_delta    |      31,984 ns/iter (+/- 836) = 4002 MB/s           |
|  12      | decompress        |      14,986 ns/iter (+/- 678) = 8541 MB/s           |
|  12      | decompress_delta  |      14,772 ns/iter (+/- 97) = 8665 MB/s            |
|  13      | compress          |      20,979 ns/iter (+/- 322) = 6101 MB/s           |
|  13      | compress_delta    |      32,507 ns/iter (+/- 467) = 3937 MB/s           |
|  13      | decompress        |      16,836 ns/iter (+/- 566) = 7602 MB/s           |
|  13      | decompress_delta  |      14,736 ns/iter (+/- 26) = 8686 MB/s            |
|  14      | compress          |      20,957 ns/iter (+/- 802) = 6107 MB/s           |
|  14      | compress_delta    |      32,723 ns/iter (+/- 489) = 3911 MB/s           |
|  14      | decompress        |      16,292 ns/iter (+/- 1,195) = 7856 MB/s         |
|  14      | decompress_delta  |      14,663 ns/iter (+/- 127) = 8729 MB/s           |
|  15      | compress          |      22,403 ns/iter (+/- 774) = 5713 MB/s           |
|  15      | compress_delta    |      33,378 ns/iter (+/- 492) = 3834 MB/s           |
|  15      | decompress        |      17,286 ns/iter (+/- 1,076) = 7404 MB/s         |
|  15      | decompress_delta  |      13,387 ns/iter (+/- 86) = 9561 MB/s            |
|  16      | compress          |      16,434 ns/iter (+/- 293) = 7788 MB/s           |
|  16      | compress_delta    |      33,865 ns/iter (+/- 518) = 3779 MB/s           |
|  16      | decompress        |      14,379 ns/iter (+/- 961) = 8901 MB/s           |
|  16      | decompress_delta  |      12,704 ns/iter (+/- 28) = 10075 MB/s           |
|  17      | compress          |      22,306 ns/iter (+/- 565) = 5738 MB/s           |
|  17      | compress_delta    |      34,378 ns/iter (+/- 451) = 3723 MB/s           |
|  17      | decompress        |      17,714 ns/iter (+/- 1,379) = 7225 MB/s         |
|  17      | decompress_delta  |      12,736 ns/iter (+/- 35) = 10050 MB/s           |
|  18      | compress          |      22,238 ns/iter (+/- 335) = 5755 MB/s           |
|  18      | compress_delta    |      33,419 ns/iter (+/- 488) = 3830 MB/s           |
|  18      | decompress        |      17,247 ns/iter (+/- 1,033) = 7421 MB/s         |
|  18      | decompress_delta  |      10,877 ns/iter (+/- 13) = 11767 MB/s           |
|  19      | compress          |      23,310 ns/iter (+/- 351) = 5491 MB/s           |
|  19      | compress_delta    |      33,654 ns/iter (+/- 600) = 3803 MB/s           |
|  19      | decompress        |      18,002 ns/iter (+/- 1,349) = 7110 MB/s         |
|  19      | decompress_delta  |      11,008 ns/iter (+/- 79) = 11627 MB/s           |
|  20      | compress          |      22,851 ns/iter (+/- 820) = 5601 MB/s           |
|  20      | compress_delta    |      34,058 ns/iter (+/- 562) = 3758 MB/s           |
|  20      | decompress        |      17,243 ns/iter (+/- 1,043) = 7423 MB/s         |
|  20      | decompress_delta  |      11,062 ns/iter (+/- 42) = 11571 MB/s           |
|  21      | compress          |      25,202 ns/iter (+/- 374) = 5078 MB/s           |
|  21      | compress_delta    |      34,903 ns/iter (+/- 515) = 3667 MB/s           |
|  21      | decompress        |      18,521 ns/iter (+/- 856) = 6911 MB/s           |
|  21      | decompress_delta  |      11,060 ns/iter (+/- 47) = 11573 MB/s           |
|  22      | compress          |      24,963 ns/iter (+/- 389) = 5127 MB/s           |
|  22      | compress_delta    |      35,112 ns/iter (+/- 731) = 3645 MB/s           |
|  22      | decompress        |      18,296 ns/iter (+/- 941) = 6996 MB/s           |
|  22      | decompress_delta  |      11,067 ns/iter (+/- 83) = 11565 MB/s           |
|  23      | compress          |      26,182 ns/iter (+/- 405) = 4888 MB/s           |
|  23      | compress_delta    |      35,223 ns/iter (+/- 498) = 3633 MB/s           |
|  23      | decompress        |      19,019 ns/iter (+/- 1,259) = 6730 MB/s         |
|  23      | decompress_delta  |      11,077 ns/iter (+/- 92) = 11555 MB/s           |
|  24      | compress          |      23,486 ns/iter (+/- 393) = 5450 MB/s           |
|  24      | compress_delta    |      35,284 ns/iter (+/- 517) = 3627 MB/s           |
|  24      | decompress        |      16,917 ns/iter (+/- 1,575) = 7566 MB/s         |
|  24      | decompress_delta  |      10,413 ns/iter (+/- 82) = 12292 MB/s           |
|  25      | compress          |      27,895 ns/iter (+/- 216) = 4588 MB/s           |
|  25      | compress_delta    |      35,456 ns/iter (+/- 504) = 3610 MB/s           |
|  25      | decompress        |      16,265 ns/iter (+/- 915) = 7869 MB/s           |
|  25      | decompress_delta  |       9,991 ns/iter (+/- 12) = 12811 MB/s           |
|  26      | compress          |      24,047 ns/iter (+/- 504) = 5322 MB/s           |
|  26      | compress_delta    |      33,299 ns/iter (+/- 974) = 3843 MB/s           |
|  26      | decompress        |      12,330 ns/iter (+/- 48) = 10381 MB/s           |
|  26      | decompress_delta  |       9,616 ns/iter (+/- 21) = 13311 MB/s           |
|  27      | compress          |      25,074 ns/iter (+/- 914) = 5104 MB/s           |
|  27      | compress_delta    |      33,667 ns/iter (+/- 438) = 3801 MB/s           |
|  27      | decompress        |      14,546 ns/iter (+/- 1,424) = 8799 MB/s         |
|  27      | decompress_delta  |       9,609 ns/iter (+/- 31) = 13320 MB/s           |
|  28      | compress          |      22,929 ns/iter (+/- 331) = 5582 MB/s           |
|  28      | compress_delta    |      33,796 ns/iter (+/- 472) = 3787 MB/s           |
|  28      | decompress        |      14,621 ns/iter (+/- 1,590) = 8754 MB/s         |
|  28      | decompress_delta  |       9,404 ns/iter (+/- 27) = 13611 MB/s           |
|  29      | compress          |      25,489 ns/iter (+/- 393) = 5021 MB/s           |
|  29      | compress_delta    |      33,705 ns/iter (+/- 487) = 3797 MB/s           |
|  29      | decompress        |      11,653 ns/iter (+/- 37) = 10984 MB/s           |
|  29      | decompress_delta  |       8,964 ns/iter (+/- 81) = 14279 MB/s           |
|  30      | compress          |      21,375 ns/iter (+/- 330) = 5988 MB/s           |
|  30      | compress_delta    |      33,739 ns/iter (+/- 431) = 3793 MB/s           |
|  30      | decompress        |      11,955 ns/iter (+/- 12) = 10706 MB/s           |
|  30      | decompress_delta  |       9,114 ns/iter (+/- 89) = 14044 MB/s           |
|  31      | compress          |      18,683 ns/iter (+/- 319) = 6851 MB/s           |
|  31      | compress_delta    |      33,679 ns/iter (+/- 474) = 3800 MB/s           |
|  31      | decompress        |      12,674 ns/iter (+/- 582) = 10099 MB/s          |
|  31      | decompress_delta  |       9,676 ns/iter (+/- 13) = 13228 MB/s           |
|  32      | compress          |      13,347 ns/iter (+/- 242) = 9590 MB/s           |
|  32      | compress_delta    |      33,772 ns/iter (+/- 478) = 3790 MB/s           |
|  32      | decompress        |       9,088 ns/iter (+/- 55) = 14084 MB/s           |
|  32      | decompress_delta  |       9,039 ns/iter (+/- 14) = 14160 MB/s           |

## SSE3


| num bits | operation         | throughput. divide by 4 to get a number of integers |
|:-- ------|:------------------|:----------------------------------------------------|
|  01      | compress          |  18,666 ns/iter (+/- 424) = 27429 MB/s              |
|  01      | compress_delta    |  33,440 ns/iter (+/- 541) = 15311 MB/s              |
|  01      | decompress        |  20,061 ns/iter (+/- 60) = 25522 MB/s               |
|  01      | decompress_delta  |  22,488 ns/iter (+/- 115) = 22767 MB/s              |
|  02      | compress          |  18,654 ns/iter (+/- 460) = 27447 MB/s              |
|  02      | compress_delta    |  34,717 ns/iter (+/- 532) = 14747 MB/s              |
|  02      | decompress        |  20,081 ns/iter (+/- 23) = 25496 MB/s               |
|  02      | decompress_delta  |  22,621 ns/iter (+/- 116) = 22633 MB/s              |
|  03      | compress          |  19,146 ns/iter (+/- 555) = 26741 MB/s              |
|  03      | compress_delta    |  35,127 ns/iter (+/- 498) = 14575 MB/s              |
|  03      | decompress        |  20,446 ns/iter (+/- 35) = 25041 MB/s               |
|  03      | decompress_delta  |  22,790 ns/iter (+/- 102) = 22465 MB/s              |
|  04      | compress          |  18,368 ns/iter (+/- 562) = 27874 MB/s              |
|  04      | compress_delta    |  35,812 ns/iter (+/- 963) = 14296 MB/s              |
|  04      | decompress        |  20,589 ns/iter (+/- 264) = 24867 MB/s              |
|  04      | decompress_delta  |  22,872 ns/iter (+/- 95) = 22385 MB/s               |
|  05      | compress          |  19,483 ns/iter (+/- 622) = 26279 MB/s              |
|  05      | compress_delta    |  36,688 ns/iter (+/- 724) = 13955 MB/s              |
|  05      | decompress        |  20,698 ns/iter (+/- 320) = 24736 MB/s              |
|  05      | decompress_delta  |  23,223 ns/iter (+/- 185) = 22047 MB/s              |
|  06      | compress          |  19,744 ns/iter (+/- 632) = 25931 MB/s              |
|  06      | compress_delta    |  37,648 ns/iter (+/- 674) = 13599 MB/s              |
|  06      | decompress        |  20,856 ns/iter (+/- 3,136) = 24549 MB/s            |
|  06      | decompress_delta  |  23,219 ns/iter (+/- 124) = 22050 MB/s              |
|  07      | compress          |  19,869 ns/iter (+/- 703) = 25768 MB/s              |
|  07      | compress_delta    |  38,502 ns/iter (+/- 586) = 13298 MB/s              |
|  07      | decompress        |  21,039 ns/iter (+/- 2,616) = 24335 MB/s            |
|  07      | decompress_delta  |  23,604 ns/iter (+/- 135) = 21691 MB/s              |
|  08      | compress          |  18,700 ns/iter (+/- 812) = 27379 MB/s              |
|  08      | compress_delta    |  39,279 ns/iter (+/- 748) = 13034 MB/s              |
|  08      | decompress        |  21,042 ns/iter (+/- 138) = 24332 MB/s              |
|  08      | decompress_delta  |  23,599 ns/iter (+/- 144) = 21695 MB/s              |
|  09      | compress          |  20,180 ns/iter (+/- 841) = 25371 MB/s              |
|  09      | compress_delta    |  40,329 ns/iter (+/- 750) = 12695 MB/s              |
|  09      | decompress        |  21,450 ns/iter (+/- 47) = 23869 MB/s               |
|  09      | decompress_delta  |  24,121 ns/iter (+/- 142) = 21226 MB/s              |
|  10      | compress          |  20,378 ns/iter (+/- 869) = 25125 MB/s              |
|  10      | compress_delta    |  41,385 ns/iter (+/- 1,041) = 12371 MB/s            |
|  10      | decompress        |  21,296 ns/iter (+/- 56) = 24042 MB/s               |
|  10      | decompress_delta  |  24,020 ns/iter (+/- 91) = 21315 MB/s               |
|  11      | compress          |  20,886 ns/iter (+/- 831) = 24514 MB/s              |
|  11      | compress_delta    |  41,756 ns/iter (+/- 655) = 12261 MB/s              |
|  11      | decompress        |  21,593 ns/iter (+/- 50) = 23711 MB/s               |
|  11      | decompress_delta  |  24,342 ns/iter (+/- 117) = 21033 MB/s              |
|  12      | compress          |  20,525 ns/iter (+/- 485) = 24945 MB/s              |
|  12      | compress_delta    |  42,766 ns/iter (+/- 621) = 11972 MB/s              |
|  12      | decompress        |  22,021 ns/iter (+/- 49) = 23250 MB/s               |
|  12      | decompress_delta  |  24,293 ns/iter (+/- 98) = 21076 MB/s               |
|  13      | compress          |  21,440 ns/iter (+/- 814) = 23880 MB/s              |
|  13      | compress_delta    |  43,275 ns/iter (+/- 732) = 11831 MB/s              |
|  13      | decompress        |  22,074 ns/iter (+/- 149) = 23194 MB/s              |
|  13      | decompress_delta  |  24,476 ns/iter (+/- 162) = 20918 MB/s              |
|  14      | compress          |  22,549 ns/iter (+/- 884) = 22706 MB/s              |
|  14      | compress_delta    |  44,530 ns/iter (+/- 835) = 11497 MB/s              |
|  14      | decompress        |  22,362 ns/iter (+/- 241) = 22895 MB/s              |
|  14      | decompress_delta  |  24,593 ns/iter (+/- 93) = 20818 MB/s               |
|  15      | compress          |  22,133 ns/iter (+/- 1,012) = 23132 MB/s            |
|  15      | compress_delta    |  45,404 ns/iter (+/- 608) = 11276 MB/s              |
|  15      | decompress        |  22,425 ns/iter (+/- 120) = 22831 MB/s              |
|  15      | decompress_delta  |  24,853 ns/iter (+/- 128) = 20601 MB/s              |
|  16      | compress          |  20,586 ns/iter (+/- 1,044) = 24871 MB/s            |
|  16      | compress_delta    |  46,324 ns/iter (+/- 877) = 11052 MB/s              |
|  16      | decompress        |  22,361 ns/iter (+/- 66) = 22897 MB/s               |
|  16      | decompress_delta  |  25,550 ns/iter (+/- 84) = 20039 MB/s               |
|  17      | compress          |  23,456 ns/iter (+/- 1,002) = 21828 MB/s            |
|  17      | compress_delta    |  46,498 ns/iter (+/- 729) = 11011 MB/s              |
|  17      | decompress        |  22,794 ns/iter (+/- 104) = 22462 MB/s              |
|  17      | decompress_delta  |  24,821 ns/iter (+/- 90) = 20627 MB/s               |
|  18      | compress          |  22,926 ns/iter (+/- 676) = 22332 MB/s              |
|  18      | compress_delta    |  47,890 ns/iter (+/- 860) = 10691 MB/s              |
|  18      | decompress        |  22,845 ns/iter (+/- 112) = 22411 MB/s              |
|  18      | decompress_delta  |  25,305 ns/iter (+/- 243) = 20233 MB/s              |
|  19      | compress          |  23,169 ns/iter (+/- 434) = 22098 MB/s              |
|  19      | compress_delta    |  46,823 ns/iter (+/- 725) = 10934 MB/s              |
|  19      | decompress        |  22,956 ns/iter (+/- 63) = 22303 MB/s               |
|  19      | decompress_delta  |  25,201 ns/iter (+/- 211) = 20316 MB/s              |
|  20      | compress          |  23,233 ns/iter (+/- 1,193) = 22037 MB/s            |
|  20      | compress_delta    |  46,803 ns/iter (+/- 861) = 10939 MB/s              |
|  20      | decompress        |  23,213 ns/iter (+/- 91) = 22056 MB/s               |
|  20      | decompress_delta  |  25,077 ns/iter (+/- 105) = 20417 MB/s              |
|  21      | compress          |  23,837 ns/iter (+/- 1,511) = 21479 MB/s            |
|  21      | compress_delta    |  47,095 ns/iter (+/- 823) = 10871 MB/s              |
|  21      | decompress        |  23,396 ns/iter (+/- 102) = 21884 MB/s              |
|  21      | decompress_delta  |  24,923 ns/iter (+/- 126) = 20543 MB/s              |
|  22      | compress          |  23,947 ns/iter (+/- 851) = 21380 MB/s              |
|  22      | compress_delta    |  47,218 ns/iter (+/- 762) = 10843 MB/s              |
|  22      | decompress        |  23,576 ns/iter (+/- 122) = 21717 MB/s              |
|  22      | decompress_delta  |  25,365 ns/iter (+/- 157) = 20185 MB/s              |
|  23      | compress          |  24,515 ns/iter (+/- 788) = 20885 MB/s              |
|  23      | compress_delta    |  46,631 ns/iter (+/- 788) = 10979 MB/s              |
|  23      | decompress        |  23,842 ns/iter (+/- 112) = 21474 MB/s              |
|  23      | decompress_delta  |  25,312 ns/iter (+/- 228) = 20227 MB/s              |
|  24      | compress          |  24,203 ns/iter (+/- 780) = 21154 MB/s              |
|  24      | compress_delta    |  47,960 ns/iter (+/- 633) = 10675 MB/s              |
|  24      | decompress        |  23,909 ns/iter (+/- 55) = 21414 MB/s               |
|  24      | decompress_delta  |  25,054 ns/iter (+/- 66) = 20435 MB/s               |
|  25      | compress          |  25,572 ns/iter (+/- 158) = 20021 MB/s              |
|  25      | compress_delta    |  47,610 ns/iter (+/- 813) = 10754 MB/s              |
|  25      | decompress        |  24,179 ns/iter (+/- 198) = 21175 MB/s              |
|  25      | decompress_delta  |  26,366 ns/iter (+/- 95) = 19418 MB/s               |
|  26      | compress          |  25,604 ns/iter (+/- 737) = 19996 MB/s              |
|  26      | compress_delta    |  47,051 ns/iter (+/- 644) = 10881 MB/s              |
|  26      | decompress        |  24,190 ns/iter (+/- 809) = 21165 MB/s              |
|  26      | decompress_delta  |  24,692 ns/iter (+/- 126) = 20735 MB/s              |
|  27      | compress          |  26,226 ns/iter (+/- 640) = 19522 MB/s              |
|  27      | compress_delta    |  47,491 ns/iter (+/- 658) = 10780 MB/s              |
|  27      | decompress        |  24,255 ns/iter (+/- 232) = 21109 MB/s              |
|  27      | decompress_delta  |  25,048 ns/iter (+/- 211) = 20440 MB/s              |
|  28      | compress          |  26,572 ns/iter (+/- 367) = 19268 MB/s              |
|  28      | compress_delta    |  47,094 ns/iter (+/- 501) = 10871 MB/s              |
|  28      | decompress        |  24,398 ns/iter (+/- 138) = 20985 MB/s              |
|  28      | decompress_delta  |  26,486 ns/iter (+/- 110) = 19330 MB/s              |
|  29      | compress          |  27,430 ns/iter (+/- 321) = 18665 MB/s              |
|  29      | compress_delta    |  47,540 ns/iter (+/- 578) = 10769 MB/s              |
|  29      | decompress        |  24,666 ns/iter (+/- 140) = 20757 MB/s              |
|  29      | decompress_delta  |  24,920 ns/iter (+/- 112) = 20545 MB/s              |
|  30      | compress          |  27,222 ns/iter (+/- 445) = 18808 MB/s              |
|  30      | compress_delta    |  48,632 ns/iter (+/- 742) = 10528 MB/s              |
|  30      | decompress        |  24,848 ns/iter (+/- 120) = 20605 MB/s              |
|  30      | decompress_delta  |  26,476 ns/iter (+/- 114) = 19338 MB/s              |
|  31      | compress          |  27,892 ns/iter (+/- 939) = 18356 MB/s              |
|  31      | compress_delta    |  47,625 ns/iter (+/- 812) = 10750 MB/s              |
|  31      | decompress        |  25,137 ns/iter (+/- 101) = 20368 MB/s              |
|  31      | decompress_delta  |  26,686 ns/iter (+/- 67) = 19186 MB/s               |
|  32      | compress          |  27,382 ns/iter (+/- 757) = 18698 MB/s              |
|  32      | compress_delta    |  48,300 ns/iter (+/- 592) = 10600 MB/s              |
|  32      | decompress        |  24,611 ns/iter (+/- 112) = 20803 MB/s              |
|  32      | decompress_delta  |  24,841 ns/iter (+/- 177) = 20611 MB/s              |



## AVX2

Requires to compile with the `avx2` feature.

| num bits | operation         | throughput. divide by 4 to get a number of integers |
|:-- ------|:------------------|:----------------------------------------------------|
| 01       | compress          |  36,392 ns/iter (+/- 1,063) = 28138 MB/s            |
| 01       | compress_delta    |  108,166 ns/iter (+/- 2,358) = 9466 MB/s            |
| 01       | decompress        |  43,950 ns/iter (+/- 224) = 23299 MB/s              |
| 01       | decompress_delta  |  54,439 ns/iter (+/- 557) = 18810 MB/s              |
| 02       | compress          |  36,592 ns/iter (+/- 456) = 27984 MB/s              |
| 02       | compress_delta    |  111,022 ns/iter (+/- 12,232) = 9223 MB/s           |
| 02       | decompress        |  44,847 ns/iter (+/- 288) = 22833 MB/s              |
| 02       | decompress_delta  |  55,070 ns/iter (+/- 502) = 18594 MB/s              |
| 03       | compress          |  38,711 ns/iter (+/- 287) = 26452 MB/s              |
| 03       | compress_delta    |  105,453 ns/iter (+/- 2,889) = 9710 MB/s            |
| 03       | decompress        |  45,409 ns/iter (+/- 310) = 22550 MB/s              |
| 03       | decompress_delta  |  56,256 ns/iter (+/- 145) = 18202 MB/s              |
| 04       | compress          |  39,022 ns/iter (+/- 438) = 26241 MB/s              |
| 04       | compress_delta    |  110,057 ns/iter (+/- 15,065) = 9304 MB/s           |
| 04       | decompress        |  45,815 ns/iter (+/- 303) = 22350 MB/s              |
| 04       | decompress_delta  |  56,343 ns/iter (+/- 622) = 18174 MB/s              |
| 05       | compress          |  39,017 ns/iter (+/- 346) = 26244 MB/s              |
| 05       | compress_delta    |  111,393 ns/iter (+/- 10,697) = 9192 MB/s           |
| 05       | decompress        |  46,440 ns/iter (+/- 133) = 22049 MB/s              |
| 05       | decompress_delta  |  57,000 ns/iter (+/- 606) = 17964 MB/s              |
| 06       | compress          |  39,229 ns/iter (+/- 393) = 26103 MB/s              |
| 06       | compress_delta    |  116,133 ns/iter (+/- 585) = 8817 MB/s              |
| 06       | decompress        |  47,361 ns/iter (+/- 292) = 21621 MB/s              |
| 06       | decompress_delta  |  57,835 ns/iter (+/- 268) = 17705 MB/s              |
| 07       | compress          |  41,095 ns/iter (+/- 3,312) = 24917 MB/s            |
| 07       | compress_delta    |  106,684 ns/iter (+/- 444) = 9598 MB/s              |
| 07       | decompress        |  48,102 ns/iter (+/- 278) = 21288 MB/s              |
| 07       | decompress_delta  |  58,067 ns/iter (+/- 597) = 17634 MB/s              |
| 08       | compress          |  39,537 ns/iter (+/- 234) = 25899 MB/s              |
| 08       | compress_delta    |  111,434 ns/iter (+/- 2,006) = 9189 MB/s            |
| 08       | decompress        |  48,890 ns/iter (+/- 266) = 20944 MB/s              |
| 08       | decompress_delta  |  58,651 ns/iter (+/- 418) = 17459 MB/s              |
| 09       | compress          |  40,887 ns/iter (+/- 395) = 25044 MB/s              |
| 09       | compress_delta    |  111,025 ns/iter (+/- 26,735) = 9223 MB/s           |
| 09       | decompress        |  49,114 ns/iter (+/- 643) = 20849 MB/s              |
| 09       | decompress_delta  |  59,248 ns/iter (+/- 320) = 17283 MB/s              |
| 10       | compress          |  43,684 ns/iter (+/- 453) = 23441 MB/s              |
| 10       | compress_delta    |  116,563 ns/iter (+/- 1,550) = 8784 MB/s            |
| 10       | decompress        |  49,826 ns/iter (+/- 216) = 20551 MB/s              |
| 10       | decompress_delta  |  59,868 ns/iter (+/- 376) = 17104 MB/s              |
| 11       | compress          |  43,494 ns/iter (+/- 659) = 23543 MB/s              |
| 11       | compress_delta    |  105,528 ns/iter (+/- 676) = 9703 MB/s              |
| 11       | decompress        |  50,555 ns/iter (+/- 141) = 20255 MB/s              |
| 11       | decompress_delta  |  61,846 ns/iter (+/- 9,814) = 16557 MB/s            |
| 12       | compress          |  42,436 ns/iter (+/- 5,820) = 24130 MB/s            |
| 12       | compress_delta    |  113,533 ns/iter (+/- 2,352) = 9019 MB/s            |
| 12       | decompress        |  51,217 ns/iter (+/- 865) = 19993 MB/s              |
| 12       | decompress_delta  |  60,895 ns/iter (+/- 765) = 16815 MB/s              |
| 13       | compress          |  44,516 ns/iter (+/- 699) = 23002 MB/s              |
| 13       | compress_delta    |  110,107 ns/iter (+/- 3,208) = 9300 MB/s            |
| 13       | decompress        |  51,883 ns/iter (+/- 539) = 19736 MB/s              |
| 13       | decompress_delta  |  61,906 ns/iter (+/- 1,034) = 16541 MB/s            |
| 14       | compress          |  46,015 ns/iter (+/- 1,049) = 22253 MB/s            |
| 14       | compress_delta    |  112,966 ns/iter (+/- 1,806) = 9064 MB/s            |
| 14       | decompress        |  52,763 ns/iter (+/- 521) = 19407 MB/s              |
| 14       | decompress_delta  |  62,356 ns/iter (+/- 918) = 16421 MB/s              |
| 15       | compress          |  45,313 ns/iter (+/- 778) = 22598 MB/s              |
| 15       | compress_delta    |  105,469 ns/iter (+/- 1,838) = 9709 MB/s            |
| 15       | decompress        |  53,387 ns/iter (+/- 801) = 19180 MB/s              |
| 15       | decompress_delta  |  62,990 ns/iter (+/- 913) = 16256 MB/s              |
| 16       | compress          |  44,624 ns/iter (+/- 1,084) = 22947 MB/s            |
| 16       | compress_delta    |  106,158 ns/iter (+/- 2,254) = 9645 MB/s            |
| 16       | decompress        |  53,694 ns/iter (+/- 334) = 19071 MB/s              |
| 16       | decompress_delta  |  63,033 ns/iter (+/- 709) = 16245 MB/s              |
| 17       | compress          |  47,739 ns/iter (+/- 13,320) = 21449 MB/s           |
| 17       | compress_delta    |  103,389 ns/iter (+/- 115,203) = 9904 MB/s          |
| 17       | decompress        |  55,597 ns/iter (+/- 10,976) = 18418 MB/s           |
| 17       | decompress_delta  |  63,559 ns/iter (+/- 32,018) = 16111 MB/s           |
| 18       | compress          |  49,566 ns/iter (+/- 13,625) = 20659 MB/s           |
| 18       | compress_delta    |  106,477 ns/iter (+/- 1,719) = 9617 MB/s            |
| 18       | decompress        |  55,725 ns/iter (+/- 729) = 18375 MB/s              |
| 18       | decompress_delta  |  62,753 ns/iter (+/- 2,087) = 16317 MB/s            |
| 19       | compress          |  50,131 ns/iter (+/- 2,159) = 20426 MB/s            |
| 19       | compress_delta    |  104,152 ns/iter (+/- 2,013) = 9831 MB/s            |
| 19       | decompress        |  56,041 ns/iter (+/- 413) = 18272 MB/s              |
| 19       | decompress_delta  |  63,331 ns/iter (+/- 3,002) = 16169 MB/s            |
| 20       | compress          |  52,123 ns/iter (+/- 2,475) = 19645 MB/s            |
| 20       | compress_delta    |  102,649 ns/iter (+/- 15,525) = 9975 MB/s           |
| 20       | decompress        |  56,766 ns/iter (+/- 890) = 18038 MB/s              |
| 20       | decompress_delta  |  63,020 ns/iter (+/- 1,088) = 16248 MB/s            |
| 21       | compress          |  51,861 ns/iter (+/- 1,236) = 19745 MB/s            |
| 21       | compress_delta    |  105,036 ns/iter (+/- 1,724) = 9749 MB/s            |
| 21       | decompress        |  57,574 ns/iter (+/- 372) = 17785 MB/s              |
| 21       | decompress_delta  |  63,127 ns/iter (+/- 534) = 16221 MB/s              |
| 22       | compress          |  52,529 ns/iter (+/- 573) = 19493 MB/s              |
| 22       | compress_delta    |  99,587 ns/iter (+/- 4,488) = 10282 MB/s            |
| 22       | decompress        |  57,844 ns/iter (+/- 842) = 17702 MB/s              |
| 22       | decompress_delta  |  63,206 ns/iter (+/- 2,165) = 16200 MB/s            |
| 23       | compress          |  54,342 ns/iter (+/- 2,061) = 18843 MB/s            |
| 23       | compress_delta    |  102,458 ns/iter (+/- 7,285) = 9994 MB/s            |
| 23       | decompress        |  58,432 ns/iter (+/- 1,194) = 17524 MB/s            |
| 23       | decompress_delta  |  64,051 ns/iter (+/- 3,629) = 15987 MB/s            |
| 24       | compress          |  55,193 ns/iter (+/- 6,001) = 18553 MB/s            |
| 24       | compress_delta    |  93,294 ns/iter (+/- 10,127) = 10976 MB/s           |
| 24       | decompress        |  58,782 ns/iter (+/- 830) = 17420 MB/s              |
| 24       | decompress_delta  |  65,271 ns/iter (+/- 2,452) = 15688 MB/s            |
| 25       | compress          |  57,907 ns/iter (+/- 3,361) = 17683 MB/s            |
| 25       | compress_delta    |  92,469 ns/iter (+/- 1,176) = 11073 MB/s            |
| 25       | decompress        |  59,381 ns/iter (+/- 1,178) = 17244 MB/s            |
| 25       | decompress_delta  |  62,692 ns/iter (+/- 433) = 16333 MB/s              |
| 26       | compress          |  58,853 ns/iter (+/- 3,033) = 17399 MB/s            |
| 26       | compress_delta    |  93,546 ns/iter (+/- 963) = 10946 MB/s              |
| 26       | decompress        |  59,986 ns/iter (+/- 309) = 17070 MB/s              |
| 26       | decompress_delta  |  63,865 ns/iter (+/- 755) = 16033 MB/s              |
| 27       | compress          |  59,247 ns/iter (+/- 408) = 17283 MB/s              |
| 27       | compress_delta    |  90,042 ns/iter (+/- 724) = 11372 MB/s              |
| 27       | decompress        |  60,817 ns/iter (+/- 732) = 16837 MB/s              |
| 27       | decompress_delta  |  64,593 ns/iter (+/- 401) = 15853 MB/s              |
| 28       | compress          |  59,886 ns/iter (+/- 411) = 17099 MB/s              |
| 28       | compress_delta    |  92,571 ns/iter (+/- 16,415) = 11061 MB/s           |
| 28       | decompress        |  61,217 ns/iter (+/- 2,677) = 16727 MB/s            |
| 28       | decompress_delta  |  64,477 ns/iter (+/- 1,474) = 15881 MB/s            |
| 29       | compress          |  62,362 ns/iter (+/- 613) = 16420 MB/s              |
| 29       | compress_delta    |  92,841 ns/iter (+/- 1,587) = 11029 MB/s            |
| 29       | decompress        |  62,035 ns/iter (+/- 4,271) = 16506 MB/s            |
| 29       | decompress_delta  |  65,349 ns/iter (+/- 1,779) = 15669 MB/s            |
| 30       | compress          |  62,690 ns/iter (+/- 663) = 16334 MB/s              |
| 30       | compress_delta    |  94,629 ns/iter (+/- 2,001) = 10821 MB/s            |
| 30       | decompress        |  62,591 ns/iter (+/- 851) = 16360 MB/s              |
| 30       | decompress_delta  |  62,706 ns/iter (+/- 2,699) = 16330 MB/s            |
| 31       | compress          |  64,525 ns/iter (+/- 1,087) = 15869 MB/s            |
| 31       | compress_delta    |  91,806 ns/iter (+/- 1,000) = 11153 MB/s            |
| 31       | decompress        |  62,690 ns/iter (+/- 570) = 16334 MB/s              |
| 31       | decompress_delta  |  62,493 ns/iter (+/- 480) = 16385 MB/s              |
| 32       | compress          |  63,763 ns/iter (+/- 878) = 16059 MB/s              |
| 32       | compress_delta    |  92,729 ns/iter (+/- 1,126) = 11042 MB/s            |
| 32       | decompress        |  63,311 ns/iter (+/- 455) = 16174 MB/s              |
| 32       | decompress_delta  |  63,814 ns/iter (+/- 2,171) = 16046 MB/s            |


# What kind of software could benefit from this crate?


Make sure to compile with

	RUSTFLAGS="-C target-cpu=native"

# Reference

- [SIMD Compression and the Intersection of Sorted Integers](https://arxiv.org/abs/1401.6399)

# Other crates you might want to check out

- [stream vbyte](https://crates.io/crates/stream-vbyte) A Stream-VByte implementation
- [mayda](https://github.com/fralalonde/mayda) Another crate implementation the same algorithms.
