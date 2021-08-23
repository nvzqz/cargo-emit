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
    (to: $stream:expr, $($flag:literal),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-cdylib-link-arg", $flag);)+
    };
    (to: $stream:expr, $($flag:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-cdylib-link-arg", "{}", $flag);)+
    };
    ($($flag:literal),+ $(,)?) => {
        $crate::rustc_cdylib_link_arg!(to: std::io::stdout(), $($flag),+);
    };
    ($($flag:expr),+ $(,)?) => {
        $crate::rustc_cdylib_link_arg!(to: std::io::stdout(), $($flag),+);
    };
}
