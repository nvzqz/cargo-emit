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
/// cargo_emit::rustc_link_lib! {
///     "ssl", // same as `=> "dylib"`
///     "ruby" => "static",
///     "CoreFoundation" => "framework",
/// }
/// ```
#[macro_export]
macro_rules! rustc_link_lib {
    (to: $stream:expr, $name:literal $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-lib", $name);
    };
    (to: $stream:expr, $name:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-lib", "{}", $name);
    };
    (to: $stream:expr, $name:literal => $kind:literal $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-lib", concat!($kind, "=", $name));
    };
    (to: $stream:expr, $name:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-lib", "{}={}", $kind, $name);
    };
    (to: $stream:expr, $($name:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_lib!(to: $stream, $name $(=> $kind)?);)+
    } };

    ($name:literal $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $name);
    };
    ($name:expr $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $name);
    };
    ($name:literal => $kind:literal $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $name => $kind);
    };
    ($name:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $name => $kind);
    };
    ($($name:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_lib!(to: std::io::stdout(), $name $(=> $kind)?);)+
    } };
}
