# Fast Bitpacking algorithms

Rust port of [Daniel Lemire's simdcomp C library](https://github.com/lemire/simdcomp).
It contains different flavors bitpacking, leveraging different instructions

# What is BitPacking ?

Compressing small integers is a very common problem in database, and search engines.
Traditional compression scheme like LZ4 are not really suited for compressing efficiently small integers.
There are different families of solutions to this problem ; one of the most straightforward and efficient one is `bitpacking` :

- Integers are first grouped into block of constant size (e.g. `128` when using the SSE2 implementation).
- Then, The maximum bit width `b` required to represent all of these integers is identified.
- The bitpacked representation, is then the concatenation of the integers restricted to their least significant `b`-bits.

# Usage

This crate contains different implementation, depending on the instruction set available with your processor.
**The resulting format are not compatible one with each other.**

Currently the following are available :
- scalar: implementation not using any SIMD instruction. This implementation is still very performant
- SSE2:
- AVX:
- AVX


# Benchmark


# What kind of software could benefit from this crate?

[Tantivy](https://github.com/tantivy-search/tantivy) (a rust search engine) uses it to reprensents its postings list.
bit-packing is one of your options if you work with d

Make sure to compile with

	RUSTFLAGS="-C target-cpu=native" 



# Usage