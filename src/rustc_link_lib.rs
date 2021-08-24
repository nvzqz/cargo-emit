/// Tells Cargo to pass `$lib` to the compiler as a `-l` flag.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-link-lib=[$kind=]$name");
/// ```
///
/// # Examples
///
/// Useful for telling the linker what libraries should be linked.
///
/// ```
/// cargo_emit::rustc_link_lib!(
///     "ssl", // same as `=> "dylib"`
///     "ruby" => "static",
///     "CoreFoundation" => "framework",
/// );
/// ```
///
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let mut stdout = std::io::stdout();
/// // ...
/// cargo_emit::rustc_link_lib!(
///     to: stdout,
///     "ssl", // same as `=> "dylib"`
///     "ruby" => "static",
///     "CoreFoundation" => "framework",
/// );
/// ```
#[macro_export]
macro_rules! rustc_link_lib {
    (to: $stream:expr, $name:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-lib", "{}", $name);
    };
    (to: $stream:expr, $name:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-lib", "{}={}", $kind, $name);
    };
    (to: $stream:expr, $($name:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_lib!(to: $stream, $name $(=> $kind)?);)+
    } };
    ($name:expr $(,)?) => {
        $crate::rustc_link_lib!(to: std::io::stdout(), $name);
    };
    ($name:expr => $kind:expr $(,)?) => {
        $crate::rustc_link_lib!(to: std::io::stdout(), $name => $kind);
    };
    ($($name:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $crate::rustc_link_lib!(to: std::io::stdout(), $($name $(=> $kind)?),+);
    } };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_name_literal() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_lib!(
                    to: output,
                    "NAME"
                );
            }),
            @"cargo:rustc-link-lib=NAME\n"
        );
    }

    #[test]
    fn single_name_expression() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                let name = "NAME";
                crate::rustc_link_lib!(
                    to: output,
                    name
                );
            }),
            @"cargo:rustc-link-lib=NAME\n"
        );
    }

    #[test]
    fn single_name_literal_with_kind() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_lib!(
                    to: output,
                    "NAME" => "KIND"
                );
            }),
            @"cargo:rustc-link-lib=KIND=NAME\n"
        );
    }

    #[test]
    fn single_name_expression_with_kind() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                let name = "NAME";
                let kind = "KIND";
                crate::rustc_link_lib!(
                    to: output,
                    name => kind
                );
            }),
            @"cargo:rustc-link-lib=KIND=NAME\n"
        );
    }

    #[test]
    fn multiple_name_expression_with_kind() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                let name2 = "NAME2";
                let kind2 = "KIND2";
                crate::rustc_link_lib!(
                    to: output,
                    "NAME1" => "KIND1",
                    name2 => kind2,
                );
            }),
            @"cargo:rustc-link-lib=KIND1=NAME1\n\
              cargo:rustc-link-lib=KIND2=NAME2\n"
        );
    }
}
