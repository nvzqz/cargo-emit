//! <div align="center">
//!     <a href="https://github.com/nvzqz/cargo-emit">
//!         <img src="https://raw.githubusercontent.com/nvzqz/cargo-emit/assets/logo.svg?sanitize=true"
//!              alt="Cargo Emit Logo"
//!              width="300"
//!              height="300">
//!     </a>
//!     <br><br>
//!     <a href="https://crates.io/crates/cargo-emit">
//!         <img src="https://img.shields.io/crates/d/cargo-emit.svg"
//!              alt="Downloads">
//!     </a>
//!     <a href="https://travis-ci.com/nvzqz/cargo-emit">
//!         <img src="https://travis-ci.com/nvzqz/cargo-emit.svg?branch=master"
//!              alt="Build Status">
//!     </a>
//!     <img src="https://img.shields.io/badge/rustc-^1.37.0-blue.svg"
//!          alt="rustc ^1.37.0">
//!     <br><br>
//! </div>
//!
//! Talk to Cargo easily at build time, brought to you by [Nikolai Vazquez].
//!
//! This library provides:
//!
//! - Convenience macros for communicating with Cargo during the [`build.rs`]
//!   phrase. Cargo listens to certain [build script outputs] that dictate how
//!   it should behave.
//!
//! - An accessible location for seeing what script build outputs are available
//!   to emit.
//!
//! - Protection against typos that can be made when printing these formatted
//!   outputs directly yourself. Mistyping macro names will result in a compile
//!   failure.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [build-dependencies]
//! cargo-emit = "0.1"
//! ```
//!
//! and something like this to your [`build.rs`]:
//!
//! ```
//! # let should_warn = true;
//! if should_warn {
//!     cargo_emit::warning!("(C-3PO voice) We're doomed");
//! }
//! ```
//!
//! **Note:** This library is meant to be used with [Rust 2018 edition][2018],
//! so that `cargo_emit::` can be used to prefix macro calls.
//!
//! # Compatibility
//!
//! This crate is compatible with Rust 1.31+ in order to use the
//! `$crate::macro!` feature introduced in [Rust 2018][2018].
//!
//! # Examples
//!
//! Very thorough examples are provided in the docs for
//! [each individual macro](#macros).
//!
//! # Donate
//!
//! This project is made freely available (as in free beer), but unfortunately
//! not all beer is free! So, if you would like to buy me a beer (or coffee or
//! *more*), then consider supporting my work that's benefited your project
//! and thousands of others.
//!
//! <a href="https://www.patreon.com/nvzqz">
//!     <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
//! </a>
//! <a href="https://www.paypal.me/nvzqz">
//!     <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
//! </a>
//!
//! [Nikolai Vazquez]: https://twitter.com/NikolaiVazquez
//! [build script outputs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
//! [2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018
//! [crate]: https://crates.io/crates/cargo-emit
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [`build.rs`]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

#![doc(html_root_url = "https://docs.rs/cargo-emit/0.1.1")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/nvzqz/cargo-emit/assets/logo.svg?sanitize=true")]

#![deny(missing_docs)]

mod pair;
mod rerun;
mod rustc;
mod warning;
