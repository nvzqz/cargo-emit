/// Tells Cargo to pass the `-C link-arg=$flag` option to the compiler,
/// but only when building a binary target.
/// Its usage is highly platform specific.
/// It is useful to set a linker script or other linker options.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-link-arg-bins=$flag");
/// ```
///
/// # Examples
///
/// ```
/// let flag1 = // ...
/// # "";
/// let flag2 = // ...
/// # "";
/// cargo_emit::rustc_link_arg_bins!(
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
/// cargo_emit::rustc_link_arg_bins!(
///     to: stdout,
///     flag1, flag2
/// );
/// ```
#[macro_export]
macro_rules! rustc_link_arg_bins {
    (to: $stream:expr, $($flag:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-link-arg-bins", "{}", $flag);)+
    };
    ($($flag:expr),+ $(,)?) => {
        $crate::rustc_link_arg_bins!(to: std::io::stdout(), $($flag),+);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_arg_bins!(
                    to: output,
                    "ARG"
                );
            }),
            @"cargo:rustc-link-arg-bins=ARG
        "
        );
    }

    #[test]
    fn multiple() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_arg_bins!(
                    to: output,
                    "ARG1",
                    "ARG2"
                );
            }),
            @r###"
        cargo:rustc-link-arg-bins=ARG1
        cargo:rustc-link-arg-bins=ARG2
        "###
        );
    }
}
