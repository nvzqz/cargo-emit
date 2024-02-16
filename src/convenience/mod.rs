#[cfg(feature = "compare_and_set_contents_hash")]
/// Check if the contents of a file have changed using its hash instead of its modification time.
pub mod compare_and_set_contents_hash;

#[cfg(feature = "compare_and_set_contents_hash")]
pub use compare_and_set_contents_hash::compare_and_set_contents_hash;
