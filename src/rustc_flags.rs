/// Tells Cargo to pass `$flags` to the compiler.
///
/// As of this writing, only `-l` and `-L` flags are supported.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-flags=$flags");
/// ```
///
/// # Examples
///
/// The `$flags`  get concatenated into a single formatting
/// string. Formatting runtime values can be done by passing subsequent values.
///
/// ```
/// let git_rev_hash = //
/// # "0000111122223333444455556666777788889999";
/// cargo_emit::rustc_env!("MY_HASH", "{}", git_rev_hash);
/// ```
#[macro_export]
macro_rules! rustc_flags {
    (to: $stream:expr, $($flags:literal),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-flags", $flags);)+
    };
    (to: $stream:expr, $($flags:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-flags", "{}", $flags);)+
    };
    ($($flags:literal),+ $(,)?) => {
        $crate::rustc_flags!(to: std::io::stdout(), $flags);
    };
    ($($flags:expr),+ $(,)?) => {
        $crate::rustc_flags!(to: std::io::stdout(), $flags);
    };
}
