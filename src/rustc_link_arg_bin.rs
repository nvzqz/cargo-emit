/// Tells Cargo to pass the `-C link-arg=$flag` option to the compiler,
/// but only when building the binary target with name `$bin`.
/// Its usage is highly platform specific.
/// It is useful to set a linker script or other linker options.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-link-arg-bin=$bin=$flag");
/// ```
///
/// # Examples
///
/// ```
/// cargo_emit::rustc_link_arg_bin!(
///     "hello_world" => "-Wall"
/// );
/// ```
///
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let mut stdout = std::io::stdout();
/// cargo_emit::rustc_link_arg_bin!(
///     to: stdout,
///     "hello_world" => "-Wall"
/// );
/// ```
#[macro_export]
macro_rules! rustc_link_arg_bin {
    (to: $stream:expr, $bin:expr => $flags:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-arg-bin", "{}={}", $bin, $flags);
    };
    (to: $stream:expr, $($bin:expr=> $flags:expr),+ $(,)?) => { {
        $($crate::rustc_link_arg_bin!(to: $stream, $bin => $flags);)+
    } };
    ($bin:expr => $flags:expr $(,)?) => {
        $crate::rustc_link_arg_bin!(to: std::io::stdout(), $bin => $flags);
    };
    ($($bin:expr=> $flags:expr),+ $(,)?) => { {
        $crate::rustc_link_arg_bin!(to: std::io::stdout(), $($bin=> $flags),+);
    } };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_arg_bin!(
                    to: output,
                    "BIN" => "FLAGS"
                );
            }),
            @"cargo:rustc-link-arg-bin=BIN=FLAGS
        "
        );
    }

    #[test]
    fn multiple() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_arg_bin!(
                    to: output,
                    "BIN1" => "FLAGS1",
                    "BIN2" => "FLAGS2",
                );
            }),
            @r###"
        cargo:rustc-link-arg-bin=BIN1=FLAGS1
        cargo:rustc-link-arg-bin=BIN2=FLAGS2
        "###
        );
    }
}
