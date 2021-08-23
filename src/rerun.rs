/// Tells Cargo to run again if the file or directory at `$path` changes.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rerun-if-changed=$path");
/// ```
///
/// `$path` is a path to a file or directory which indicates that the build
/// script should be re-run if it changes (detected by a more-recent
/// last-modified timestamp on the file). Normally build scripts are re-run if
/// any file inside the crate root changes, but this can be used to scope
/// changes to just a small set of files. (If this path points to a directory
/// the entire directory will not be traversed for changes -- only changes to
/// the timestamp of the directory itself (which corresponds to some types of
/// changes within the directory, depending on platform) will trigger a rebuild.
/// To request a re-run on any changes within an entire directory, print a line
/// for the directory and another line for everything inside it, recursively.)
/// Note that if the build script itself (or one of its dependencies) changes,
/// then it's rebuilt and rerun unconditionally, so
/// `rerun_if_changed!("build.rs")` is almost always redundant (unless you want
/// to ignore changes in all other files except for `build.rs`).
///
/// # Examples
///
/// This is useful for tracking build-dependent files that Cargo does not
/// already know.
///
/// ```
/// cargo_emit::rerun_if_changed!(
///     "/path/to/resource1",
///     "/path/to/resource2",
/// );
/// ```
#[macro_export]
macro_rules! rerun_if_changed {
    (to: $stream:expr, $($path:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rerun-if-changed", "{}", $path);)+
    };
    ($($path:expr),+ $(,)?) => {
        $crate::rerun_if_changed!(to: std::io::stdout(), $($path),+);
    };
}

/// Tells Cargo to run again if the file or directory at `$var` changes.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rerun-if-env-changed=$var");
/// ```
///
/// `$var` is the name of an environment variable which indicates that if the
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
/// [`rerun_if_changed!`]: macro.rerun_if_changed.html
/// [`rerun_if_env_changed!`]: macro.rerun_if_env_changed.html
#[macro_export]
macro_rules! rerun_if_env_changed {
    (to: $stream:expr, $($var:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rerun-if-env-changed", "{}", $var);)+
    };
    ($($var:expr),+ $(,)?) => {
        $crate::rerun_if_env_changed!(to: std::io::stdout(), "{}", $($var),+);
    };
}
