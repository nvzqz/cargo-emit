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
/// cargo_emit::rustc_flags!("-l pthread");
/// ```
#[macro_export]
macro_rules! rustc_flags {
    (to: $stream:expr, $($flags:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-flags", "{}", $flags);)+
    };
    ($($flags:expr),+ $(,)?) => {
        $crate::rustc_flags!(to: std::io::stdout(), $($flags),+);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_flags!(
                    to: output,
                    "FLAG"
                );
            }),
            @"cargo:rustc-flags=FLAG\n"
        );
    }

    #[test]
    fn multiple() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_flags!(
                    to: output,
                    "FLAG1",
                    "FLAG2",
                );
            }),
            @"cargo:rustc-flags=FLAG1\ncargo:rustc-flags=FLAG2\n"
        );
    }
}
