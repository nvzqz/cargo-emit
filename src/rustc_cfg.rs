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

#[cfg(test)]
mod tests {
    #[test]
    fn literal() {
        insta::assert_debug_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_cfg!(
                    to: output,
                    "CFG"
                );
            }),
            @r###""cargo:rustc-cfg=CFG\n""###
        );
    }

    #[test]
    fn formatted() {
        insta::assert_debug_snapshot!(
            crate::capture_output(|output| {
                crate::rustc_cfg!(
                    to: output,
                    "{}", "CFG"
                );
            }),
            @r###""cargo:rustc-cfg=CFG\n""###
        );
    }
}
