/// Tells Cargo to pass `$path` to the compiler as a `-L` flag.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-link-search=[$kind=]$path");
/// ```
///
/// # Examples
///
/// Useful for telling the linker where a path can be found.
///
/// ```
/// cargo_emit::rustc_link_search!(
///     "path/to/ssl/lib/", // same as `=> "all"`
///     "path/to/ruby/lib/" => "native",
/// );
/// ```
///
/// or, in case you want it to emit to a custom stream:
///
/// ```
/// let mut stdout = std::io::stdout();
/// // ...
/// cargo_emit::rustc_link_search!(
///     to: stdout,
///     "path/to/ssl/lib/", // same as `=> "all"`
///     "path/to/ruby/lib/" => "native",
/// );
/// ```
#[macro_export]
macro_rules! rustc_link_search {
    (to: $stream:expr, $path:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-search", "{}", $path);
    };
    (to: $stream:expr, $path:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-search", "{}={}", $kind, $path);
    };
    (to: $stream:expr, $($path:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_search!(to: $stream, $path $(=> $kind)?);)+
    } };
    ($path:expr $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $path);
    };
    ($path:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $path => $kind);
    };
    ($($path:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_search!(to: std::io::stdout(), $path $(=> $kind)?);)+
    } };
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_name_literal() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_search!(
                    to: output,
                    "PATH"
                );
            }),
            @"cargo:rustc-link-search=PATH\n"
        );
    }

    #[test]
    fn single_name_expression() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                let path = "PATH";
                crate::rustc_link_search!(
                    to: output,
                    path
                );
            }),
            @"cargo:rustc-link-search=PATH\n"
        );
    }

    #[test]
    fn single_name_literal_with_kind() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_link_search!(
                    to: output,
                    "PATH" => "KIND"
                );
            }),
            @"cargo:rustc-link-search=KIND=PATH\n"
        );
    }

    #[test]
    fn single_name_expression_with_kind() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                let path = "PATH";
                let kind = "KIND";
                crate::rustc_link_search!(
                    to: output,
                    path => kind
                );
            }),
            @"cargo:rustc-link-search=KIND=PATH\n"
        );
    }

    #[test]
    fn multiple_name_expression_with_kind() {
        insta::assert_display_snapshot!(
            crate::capture_output(|output| {
                let path2 = "PATH2";
                let kind2 = "KIND2";
                crate::rustc_link_search!(
                    to: output,
                    "PATH1" => "KIND1",
                    path2 => kind2,
                );
            }),
            @"cargo:rustc-link-search=KIND1=PATH1\n\
              cargo:rustc-link-search=KIND2=PATH2\n"
        );
    }
}
