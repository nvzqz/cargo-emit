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
/// Useful for telling the linker where a library can be found.
///
/// ```
/// cargo_emit::rustc_link_search!(
///     "path/to/ssl/lib/", // same as `=> "all"`
///     "path/to/ruby/lib/" => "native",
/// );
/// ```
#[macro_export]
macro_rules! rustc_link_search {
    (to: $stream:expr, $path:literal $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-search", $path);
    };
    (to: $stream:expr, $path:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-search", "{}", $path);
    };
    (to: $stream:expr, $path:literal => $kind:literal $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-search", concat!($kind, "=", $path));
    };
    (to: $stream:expr, $path:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: $stream, "rustc-link-search", "{}={}", $kind, $path);
    };
    (to: $stream:expr, $($path:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_lib!(to: $stream, $path $(=> $kind)?);)+
    } };

    ($path:literal $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $path);
    };
    ($path:expr $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $path);
    };
    ($path:literal => $kind:literal $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $path => $kind);
    };
    ($path:expr => $kind:expr $(,)?) => {
        $crate::pair!(to: std::io::stdout(), $path => $kind);
    };
    ($($path:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_search!(to: std::io::stdout(), $path $(=> $kind)?);)+
    } };
}
