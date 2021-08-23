/// Tells Cargo to assign `$var` for the environment variable for `$key`.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-env=$var=$value");
/// ```
///
/// # Examples
///
/// Useful for injecting environment variables during the build.
///
/// The `$var` and `$value` parameters get concatenated into a single formatting
/// string. Formatting runtime values can be done by passing subsequent values.
///
/// ```
/// let git_rev_hash = //
/// # "0000111122223333444455556666777788889999";
/// cargo_emit::rustc_env!("MY_HASH", "{}", git_rev_hash);
/// ```
#[macro_export]
macro_rules! rustc_env {
    (to: $stream:expr, $var:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::pair!(to: $stream, "rustc-env", concat!($var, "=", $value) $(, $($args)+)?);
    };
    ($var:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::rustc_env!(to: std::io::stdout(), $var, $value $(, $($args)+)?);
    };
}
