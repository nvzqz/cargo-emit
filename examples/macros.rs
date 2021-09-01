//! Example used for testing for clippy warnings on CI.

fn main() {
    cargo_emit::pair!("KEY", "VALUE");
    cargo_emit::rerun_if_changed!("PATH");
    cargo_emit::rerun_if_env_changed!("KEY");
    cargo_emit::rustc_cdylib_link_arg!("FLAG");
    cargo_emit::rustc_cfg!("FEATURE");
    cargo_emit::rustc_env!("KEY", "VALUE");
    cargo_emit::rustc_flags!("FLAGS");
    cargo_emit::rustc_link_arg!("ARGUMENT");
    cargo_emit::rustc_link_arg_bin!("BINARY" => "ARGUMENT");
    cargo_emit::rustc_link_arg_bins!("ARGUMENT");
    cargo_emit::rustc_link_arg_bin!("BINARY" => "ARGUMENT");
    cargo_emit::rustc_link_lib!("NAME" => "KIND");
    cargo_emit::rustc_link_search!("PATH" => "KIND");
    cargo_emit::warning!("MESSAGE");
}
