<div align="center">
    <a href="https://github.com/nvzqz/cargo-emit">
        <img src="https://raw.githubusercontent.com/nvzqz/cargo-emit/assets/logo.svg?sanitize=true"
             width="300"
             height="300">
    </a>
    <h1>Cargo Emit</h1>
    <a href="https://crates.io/crates/cargo-emit">
        <img src="https://img.shields.io/crates/v/cargo-emit.svg" alt="Crates.io">
        <img src="https://img.shields.io/crates/d/cargo-emit.svg" alt="Downloads">
    </a>
    <a href="https://travis-ci.com/nvzqz/cargo-emit">
        <img src="https://travis-ci.com/nvzqz/cargo-emit.svg?branch=master" alt="Build Status">
    </a>
    <img src="https://img.shields.io/badge/rustc-^1.31.0-blue.svg" alt="rustc ^1.31.0">
    <br>
    <a href="https://www.patreon.com/nvzqz">
        <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
    </a>
    <a href="https://www.paypal.me/nvzqz">
        <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
    </a>
    <br><br>
</div>

Talk to Cargo easily at build time, brought to you by [Nikolai Vazquez].

This library provides:

- Convenience macros for communicating with Cargo during the [`build.rs`]
  phrase. Cargo listens to certain [build script outputs] that dictate how
  it should behave.

- An accessible location for seeing what script build outputs are available
  to emit.

- Protection against typos that can be made when printing these formatted
  outputs directly yourself. Mistyping macro names will result in a compile
  failure.

[Nikolai Vazquez]:      https://twitter.com/NikolaiVazquez
[`build.rs`]:           https://doc.rust-lang.org/cargo/reference/build-scripts.html
[build script outputs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script

## Usage

This crate exposes the following macros:
- [`pair!`]
- [`rerun_if_changed!`]
- [`rerun_if_env_changed!`]
- [`rustc_cdylib_link_arg!`]
- [`rustc_cfg!`]
- [`rustc_env!`]
- [`rustc_flags!`]
- [`rustc_link_lib!`]
- [`rustc_link_search!`]
- [`warning!`]

[`pair!`]:                  https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.pair.html
[`rerun_if_changed!`]:      https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rerun_if_changed.html
[`rerun_if_env_changed!`]:  https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rerun_if_env_changed.html
[`rustc_cdylib_link_arg!`]: https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_cdylib_link_arg.html
[`rustc_cfg!`]:             https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_cfg.html
[`rustc_env!`]:             https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_env.html
[`rustc_flags!`]:           https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_flags.html
[`rustc_link_lib!`]:        https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_link_lib.html
[`rustc_link_search!`]:     https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.rustc_link_search.html
[`warning!`]:               https://docs.rs/cargo-emit/0.1.0/cargo_emit/macro.warning.html

## License

This project is released under either:

- [MIT License](https://github.com/nvzqz/cargo-emit/blob/master/LICENSE-MIT)

- [Apache License (Version 2.0)](https://github.com/nvzqz/cargo-emit/blob/master/LICENSE-APACHE)
