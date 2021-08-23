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
    (to: $stream:expr, $feature:expr $(, $($args:tt)*)?) => {
        $crate::pair!(to: $stream, "rustc-cfg", $feature $(, $($args)+)?);
    };
    ($feature:expr $(, $($args:tt)*)?) => {
        $crate::rustc_cfg!(to: std::io::stdout(), $feature $(, $($args)+)?);
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
    (to: $stream:expr, $var:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::pair!(to: $stream, "rustc-env", concat!($var, "=", $value) $(, $($args)+)?);
    };
    ($var:expr, $value:expr $(, $($args:tt)*)?) => {
        $crate::rustc_env!(to: std::io::stdout(), $var, $value $(, $($args)+)?);
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
    (to: $stream:expr, $($flags:literal),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-flags", $flags);)+
    };
    (to: $stream:expr, $($flags:expr),+ $(,)?) => {
        $($crate::pair!(to: $stream, "rustc-flags", "{}", $flags);)+
    };
    ($($flags:literal),+ $(,)?) => {
        $crate::rustc_flags!(to: std::io::stdout(), $flags);
    };
    ($($flags:expr),+ $(,)?) => {
        $crate::rustc_flags!(to: std::io::stdout(), $flags);
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
