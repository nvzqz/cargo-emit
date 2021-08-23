/// Emits a `$key`/`$value` pair to Cargo based on [build script outputs].
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:$key=$value");
/// ```
///
/// This is the base macro upon which the other macros are built.
///
/// # Examples
///
/// This can be used to emit arbitrary user-defined metadata.
///
/// ```
/// cargo_emit::pair!("root", "/path/to/root");
/// ```
///
/// The `$key` and `$value` parameters get concatenated into a single formatting
/// string. Formatting runtime values can be done by passing subsequent values.
///
/// ```
/// let name = "foo";
/// cargo_emit::pair!("{lib}dir", "/path/to/{lib}", lib = name);
/// ```
///
/// [build script outputs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
#[macro_export]
macro_rules! pair {
    (to: $stream:expr, $key:expr, $value:expr $(, $($args:tt)*)?) => {{
        use std::io::Write as _;
        use std::fmt::Write as _;
        writeln!($stream, concat!("cargo:", $key, "=", $value) $(, $($args)*)?).unwrap()
    }};
    ($key:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::pair!(to: std::io::stdout(), $key, $value $(, $($args)*)?);
    };
}
