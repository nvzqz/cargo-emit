/// Tells Cargo to pass the -C link-arg=FLAG option to the compiler,
/// but only when building supported targets (benchmarks, binaries, cdylib crates, examples, and tests).
/// Its usage is highly platform specific.
/// It is useful to set the shared library version or linker script.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-link-arg=$flag");
/// ```
///
/// # Examples
///
/// ```
/// let flag1 = // ...
/// # "";
/// let flag2 = // ...
/// # "";
/// cargo_emit::rustc_link_arg!(
///     flag1, flag2
/// );
/// ```
///
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let flag1 = // ...
/// # "";
/// let flag2 = // ...
/// # "";
/// let mut stdout = std::io::stdout();
/// cargo_emit::rustc_link_arg!(
///     to: stdout,
///     flag1, flag2
/// );
/// ```
#[macro_export]
macro_rules! rustc_link_arg {
    (to: $stream:expr, $($flag:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-link-arg", "{}", $flag);)+
    };
    ($($flag:expr),+ $(,)?) => {
        $crate::rustc_link_arg!(to: std::io::stdout(), $($flag),+);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_arg!(
                    to: output,
                    "ARG"
                );
            }),
            @"cargo:rustc-link-arg=ARG
        "
        );
    }

    #[test]
    fn multiple() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_arg!(
                    to: output,
                    "ARG1",
                    "ARG2"
                );
            }),
            @r###"
        cargo:rustc-link-arg=ARG1
        cargo:rustc-link-arg=ARG2
        "###
        );
    }
}
