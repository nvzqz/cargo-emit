/// Tells Cargo to enable a `$feature`.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-cfg=$feature");
/// ```
///
/// # Examples
///
/// Useful for conditionally enabling certain code to run.
///
/// ```
/// # struct Cargo;
/// # impl Cargo {
/// #   fn can_bench(&self) -> bool { true }
/// # }
/// # let cargo = Cargo;
/// if cargo.can_bench() {
///     cargo_emit::rustc_cfg!("bench");
/// }
/// ```
///
/// Then outside of `build.rs`:
///
/// ```
/// #[cfg(bench)]
/// mod benches {
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! rustc_cfg {
    ($feature:expr $(, $($args:tt)*)?) => {
        $crate::pair!("rustc-cfg", $feature $(, $($args)+)?);
    };
}

/// Tells Cargo to assign `$var` for the environment variable for `$key`.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-env=$var=$value");
/// ```
///
/// # Examples
///
/// Useful for injecting environment variables during the build.
///
/// The `$var` and `$value` parameters get concatenated into a single formatting
/// string. Formatting runtime values can be done by passing subsequent values.
///
/// ```
/// let git_rev_hash = //
/// # "0000111122223333444455556666777788889999";
/// cargo_emit::rustc_env!("MY_HASH", "{}", git_rev_hash);
/// ```
#[macro_export]
macro_rules! rustc_env {
    ($var:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::pair!("rustc-env", concat!($var, "=", $value) $(, $($args)+)?);
    };
}

/// Tells Cargo to pass `$flags` to the compiler.
///
/// As of this writing, only `-l` and `-L` flags are supported.
///
/// This is equivalent to:
///
/// ```
/// println!("cargo:rustc-flags=$flags");
/// ```
///
/// # Examples
///
/// The `$flags`  get concatenated into a single formatting
/// string. Formatting runtime values can be done by passing subsequent values.
///
/// ```
/// let git_rev_hash = //
/// # "0000111122223333444455556666777788889999";
/// cargo_emit::rustc_env!("MY_HASH", "{}", git_rev_hash);
/// ```
#[macro_export]
macro_rules! rustc_flags {
    ($($flags:literal),+ $(,)?) => {
        $($crate::pair!("rustc-flags", $flags);)+
    };
    ($($flags:expr),+ $(,)?) => {
        $($crate::pair!("rustc-flags", "{}", $flags);)+
    };
}

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
    ($($flag:literal),+ $(,)?) => {
        $($crate::pair!("rustc-cdylib-link-arg", $flag);)+
    };
    ($($flag:expr),+ $(,)?) => {
        $($crate::pair!("rustc-cdylib-link-arg", "{}", $flag);)+
    };
}

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
    ($name:literal $(,)?) => {
        $crate::pair!("rustc-link-lib", $name);
    };
    ($name:expr $(,)?) => {
        $crate::pair!("rustc-link-lib", "{}", $name);
    };
    ($name:literal => $kind:literal $(,)?) => {
        $crate::pair!("rustc-link-lib", concat!($kind, "=", $name));
    };
    ($name:expr => $kind:expr $(,)?) => {
        $crate::pair!("rustc-link-lib", "{}={}", $kind, $name);
    };
    ($($name:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_lib!($name $(=> $kind)?);)+
    } };
}

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
    ($path:literal $(,)?) => {
        $crate::pair!("rustc-link-search", $path);
    };
    ($path:expr $(,)?) => {
        $crate::pair!("rustc-link-search", "{}", $path);
    };
    ($path:literal => $kind:literal $(,)?) => {
        $crate::pair!("rustc-link-search", concat!($kind, "=", $path));
    };
    ($path:expr => $kind:expr $(,)?) => {
        $crate::pair!("rustc-link-search", "{}={}", $kind, $path);
    };
    ($($path:expr $(=> $kind:expr)?),+ $(,)?) => { {
        $($crate::rustc_link_lib!($path $(=> $kind)?);)+
    } };
}
