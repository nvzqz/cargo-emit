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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn single_literal() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_changed!(
                    to: output,
                    "/path/to/resource"
                );
            }),
            @"cargo:rerun-if-changed=/path/to/resource\n"
        );
    }

    #[test]
    fn single_expression() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_changed!(
                    to: output,
                    PathBuf::from("/path/to/resource").display()
                );
            }),
            @"cargo:rerun-if-changed=/path/to/resource\n"
        );
    }

    #[test]
    fn multiple_literals() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_changed!(
                    to: output,
                    "/path/to/resource1",
                    "/path/to/resource2",
                    "/path/to/resource3",
                    "/path/to/resource4",
                );
            }),
            @"cargo:rerun-if-changed=/path/to/resource1\n\
              cargo:rerun-if-changed=/path/to/resource2\n\
              cargo:rerun-if-changed=/path/to/resource3\n\
              cargo:rerun-if-changed=/path/to/resource4\n"
        );
    }

    #[test]
    fn multiple_expressions() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_changed!(
                    to: output,
                    PathBuf::from("/path/to/resource1").display(),
                    PathBuf::from("/path/to/resource2").display(),
                    PathBuf::from("/path/to/resource3").display(),
                    PathBuf::from("/path/to/resource4").display(),
                );
            }),
            @"cargo:rerun-if-changed=/path/to/resource1\n\
              cargo:rerun-if-changed=/path/to/resource2\n\
              cargo:rerun-if-changed=/path/to/resource3\n\
              cargo:rerun-if-changed=/path/to/resource4\n"
        );
    }

    #[test]
    fn multiple_mixed() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rerun_if_changed!(
                    to: output,
                    "/path/to/resource1",
                    PathBuf::from("/path/to/resource2").display(),
                    "/path/to/resource3",
                    PathBuf::from("/path/to/resource4").display(),
                );
            }),
            @"cargo:rerun-if-changed=/path/to/resource1\n\
              cargo:rerun-if-changed=/path/to/resource2\n\
              cargo:rerun-if-changed=/path/to/resource3\n\
              cargo:rerun-if-changed=/path/to/resource4\n"
        );
    }
}
