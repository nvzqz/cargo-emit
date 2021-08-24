/// Tells Cargo to print the formatted `warning` message.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:warning=$args");
/// ```
///
/// # Examples
///
/// Useful for showing when something expected (but not critical) has failed.
///
/// ```
/// match std::env::current_dir() {
///     Ok(dir) => { /* ... */ }
///     Err(error) => cargo_emit::warning!(
///         "Something suspicious is happening: {}",
///         error,
///     ),
/// }
/// ```
///
/// Assuming you're building `my-crate`, you will see:
///
/// ```sh
/// $ cargo build
///    Compiling my-crate v0.1.0 (/path/to/my-crate)
/// warning: Something suspicious is happening: ...
/// ```
#[macro_export]
macro_rules! warning {
    (to: $stream:expr, $($args:tt)+) => {
        $crate::pair!(to: $stream, "warning", $($args)+)
    };
    ($($args:tt)+) => {
        $crate::warning!(to: std::io::stdout(), $($args)+)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_literal() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::warning!(
                    to: output,
                    "WARNING"
                );
            }),
            @"cargo:warning=WARNING\n"
        );
    }

    #[test]
    fn single_formatted_by_index() {
        // Formatted argument:
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::warning!(
                    to: output,
                    "{}", "WARNING"
                );
            }),
            @"cargo:warning=WARNING\n"
        );
    }

    #[test]
    fn single_formatted_by_key() {
        // Formatted argument:
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::warning!(
                    to: output,
                    "{warning}", warning = "WARNING"
                );
            }),
            @"cargo:warning=WARNING\n"
        );
    }
}
