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
///     Ok(dir) => // ...
///     # {},
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
    ($($args:tt)+) => {
        $crate::pair!("warning", $($args)+)
    };
}
