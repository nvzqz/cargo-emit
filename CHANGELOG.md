# Changelog [![Crates.io][crate-badge]][crate]
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Fixed

- Fixed recursion in `rustc_link_search!(…)`.
- Removed redundant `$x:literal` macro patterns.

### Added

- 40 snapshot tests (using [insta](https://crates.io/crates/insta))
- Optional `to:` parameter for writing into a `std::fmt::Write` or `io::fmt::Write`.

# Removed

- `#![no_std]`, since the project never worked on no_std to begin with.

## [0.1.1] - 2019-12-15

### Fixed
- Links to docs for specific macros.

## 0.1.0 - 2019-12-15

### Added
- [`pair!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.pair.html)
- [`rerun_if_changed!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rerun_if_changed.html)
- [`rerun_if_env_changed!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rerun_if_env_changed.html)
- [`rustc_cdylib_link_arg!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_cdylib_link_arg.html)
- [`rustc_cfg!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_cfg.html)
- [`rustc_env!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_env.html)
- [`rustc_flags!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_flags.html)
- [`rustc_link_lib!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_link_lib.html)
- [`rustc_link_search!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_link_search.html)
- [`warning!`](https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.warning.html)

[crate]:       https://crates.io/crates/cargo-emit
[crate-badge]: https://img.shields.io/crates/v/cargo-emit.svg

[Keep a Changelog]:    http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

[Unreleased]: https://github.com/nvzqz/static-assertions-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/nvzqz/static-assertions-rs/compare/v0.1.0...v0.1.1
