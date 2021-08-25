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
///
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let mut stdout = std::io::stdout();
/// let git_rev_hash = // ...
/// # "0000111122223333444455556666777788889999";
/// cargo_emit::rustc_env!(
///     to: stdout,
///    "MY_HASH", "{}", git_rev_hash
/// );
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

#[cfg(test)]
mod tests {
    #[test]
    fn single_literal() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "KEY", "VALUE"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_formatted_by_index() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "{}", "VALUE", "KEY"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_formatted_by_name() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "{key}", "VALUE", key = "KEY"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_value_formatted_by_index() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "KEY", "{}", "VALUE"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_value_formatted_by_name() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "KEY", "{value}", value = "VALUE"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_and_value_formatted_by_index() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "{}", "{}", "KEY", "VALUE"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }

    #[test]
    fn single_with_key_and_value_formatted_by_name() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_env!(
                    to: output,
                    "{key}", "{value}", key = "KEY", value = "VALUE"
                );
            }),
            @"cargo:rustc-env=KEY=VALUE\n"
        );
    }
}
