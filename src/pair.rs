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
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let mut stdout = std::io::stdout();
/// cargo_emit::pair!(
///     to: stdout,
///     "root", "/path/to/root"
/// );
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
        #[allow(unused_imports)]
        use std::{fmt::Write as _, io::Write as _};

        #[allow(clippy::explicit_write)]
        writeln!($stream, concat!("cargo:", $key, "=", $value) $(, $($args)*)?).unwrap()
    }};
    ($key:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::pair!(to: std::io::stdout(), $key, $value $(, $($args)*)?);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_literal() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "KEY", "VALUE"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_formatted_by_index() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "{}", "VALUE", "KEY"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_formatted_by_name() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "{key}", "VALUE", key = "KEY"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_value_formatted_by_index() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "KEY", "{}", "VALUE"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_value_formatted_by_name() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "KEY", "{value}", value = "VALUE"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_and_value_formatted_by_index() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "{}", "{}", "KEY", "VALUE"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_and_value_formatted_by_name() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::pair!(
                    to: output,
                    "{key}", "{value}", key = "KEY", value = "VALUE"
                );
            }),
            @"cargo:KEY=VALUE\n"
        );
    }
}
