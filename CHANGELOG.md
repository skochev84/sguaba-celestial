# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-28

### Added

- Initial release of standalone celestial reference frames crate
- Celestial coordinate systems: ICRS, GCRF, EME2000, Ecliptic, MCI
- High-precision Earth rotation transformations (IAU 2006/2000A)
- Time scale conversions: UTC, TAI, TT, TDB, UT1
- Keplerian orbital elements support
- TLE (Two-Line Element) parsing and propagation
- Right Ascension/Declination coordinate conversions
- Extension traits for coordinate operations
- Builder patterns for type-safe celestial coordinate construction
- Comprehensive test suite (26 unit tests, 9 doctests)
- Full documentation with examples

### Technical

- Implemented workarounds for Rust orphan rules via extension traits
- Rotation construction via public Euler angle API (see EXTRACTION_WORKAROUNDS.md)
- Dependencies: sguaba 0.9.11, nalgebra 0.34.1, chrono 0.4, uom 0.37.0

[0.1.0]: https://github.com/skochev84/sguaba-celestial/releases/tag/v0.1.0
