#[derive(Default)]
pub struct Config {
    pub derive_debug: bool,
    pub derive_clone: bool,
    pub emit_display_impl: bool,
    pub use_serde: bool,
}
