/// Tells Cargo to run again if the file or directory at `$key` changes.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rerun-if-env-changed=$key");
/// ```
///
/// `$key` is the name of an environment variable which indicates that if the
/// environment variable's value changes the build script should be rerun. This
/// basically behaves the same as [`rerun_if_changed!`] except that it works
/// with environment variables instead. Note that the environment variables here
/// are intended for global environment variables like `CC` and such, it's not
/// necessary to use this for env vars like `TARGET` that Cargo sets. Also note
/// that if [`rerun_if_env_changed!`] is used then Cargo will only rerun the
/// build script if those environment variables change or if files printed out
/// by [`rerun_if_changed!`] change.
///
/// # Examples
///
/// This is useful for tracking build-dependent files that Cargo does not
/// already know.
///
/// ```
/// cargo_emit::rerun_if_env_changed!("MY_DEPENDENCY", "PATH");
/// ```
///
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let mut stdout = std::io::stdout();
/// cargo_emit::rerun_if_env_changed!(
///     to: stdout,
///     "MY_DEPENDENCY", "PATH"
/// );
/// ```
///
/// [`rerun_if_changed!`]: macro.rerun_if_changed.html
/// [`rerun_if_env_changed!`]: macro.rerun_if_env_changed.html
#[macro_export]
macro_rules! rerun_if_env_changed {
    (to: $stream:expr, $($key:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rerun-if-env-changed", "{}", $key);)+
    };
    ($($key:expr),+ $(,)?) => {
        $crate::rerun_if_env_changed!(to: std::io::stdout(), "{}", $($key),+);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_env_changed!(
                    to: output,
                    "KEY"
                );
            }),
            @"cargo:rerun-if-env-changed=KEY\n"
        );
    }

    #[test]
    fn multiple() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_env_changed!(
                    to: output,
                    "KEY1",
                    "KEY2",
                    "KEY3",
                );
            }),
            @"cargo:rerun-if-env-changed=KEY1\n\
              cargo:rerun-if-env-changed=KEY2\n\
              cargo:rerun-if-env-changed=KEY3\n"
        );
    }
}
