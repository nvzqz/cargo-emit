/// Tells Cargo to pass `-C link-arg=$flag` to the compiler.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-cdylib-link-arg=$flag");
/// ```
///
/// # Examples
///
/// Runtime arguments can be passed directly.
///
/// ```
/// let flag1 = // ...
/// # "";
/// let flag2 = // ...
/// # "";
/// cargo_emit::rustc_cdylib_link_arg!(flag1, flag2);
/// ```
#[macro_export]
macro_rules! rustc_cdylib_link_arg {
    (to: $stream:expr, $($flag:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-cdylib-link-arg", "{}", $flag);)+
    };
    ($($flag:expr),+ $(,)?) => {
        $crate::rustc_cdylib_link_arg!(to: std::io::stdout(), $($flag),+);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_cdylib_link_arg!(
                    to: output,
                    "ARG"
                );
            }),
            @"cargo:rustc-cdylib-link-arg=ARG\n"
        );
    }

    #[test]
    fn multiple() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_cdylib_link_arg!(
                    to: output,
                    "ARG1",
                    "ARG2",
                );
            }),
            @"cargo:rustc-cdylib-link-arg=ARG1\ncargo:rustc-cdylib-link-arg=ARG2\n"
        );
    }
}
