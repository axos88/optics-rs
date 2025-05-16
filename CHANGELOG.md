# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking changes
  - refactored entire type hierarchy:
    - removed Optic base trait,separated into five base traits:
    - introduced PartialGetter, Getter, Setter, PartialReversible, Reversible
    - all optics implement a combination of these base traits
    - optics are now implemented via an Impl (public) and several concrete (opaque) structs, currently only MappedXXX, but it is now easier to add more, even outside the crate
    - removed CombineWithXXX traits, as different optics require different signatures. Combination is now part of the Impl structs.
    - optics can now be combined even if errors do not implment Into<> for eachother, but a mapper function is provided
    - lens are now contstructed via exported helper functions, not using the Struct::new, as the structs are opaque.
    - prisms now return a result with any error rather than an option
### Changed
  - refactored type constraints as per u/OliveTreeFounder's suggestion - thanks!
  - switched From bounds to Into bounds
### Added
### Fixed


## [0.1.2] - 2025-05-15

### Breaking changes
### Added
  - a proper example
  - release script
### Fixed
  - Fixed missing export of `ComposedFallibleIso`

---

## [0.1.0] - 2025-05-14

- Initial public release of `optics` crate.
- Core implementation of optics abstractions: optics, lenses, prisms, isos, and falllible isos.
- Crate documentation and CI setup.
- MSRV enforced at Rust `1.86.0`.

---

## Notes

- Minimum Supported Rust Version (MSRV): `1.86.0`
- This crate follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
- Version numbers comply with [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

