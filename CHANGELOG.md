# bitpacking 0.9.3

- Performance improvements in BitPacker4x for ARM

# bitpacking 0.9.1

- Add strictly\_sorted variant of bitpackers which allow to compress sequence known to be strictly increasing (no equal element) in a slightly more compact format.

# bitpacking 0.8.2

- Much much faster compilation in debug mode. (#20)

# bitpacking 0.8.1

- Bugfix for arrays requiring 32-bits. (#17)
- Added proptest (thanks to @danburkert)
- Switched to edition 2018

# bitpacking 0.8.0

Enabling 1x, 4x, and 8x bitpacking based on feature flags.
For backward compatibility, by default, all 3 are enabled.
