/// Configuration options for code generation.
/// Used by emitter and parser to control output behavior.
///
/// TODO (2nd pass):
/// - Add support for additional derives (e.g. PartialEq, Serialize, Deserialize)
/// - Add visibility control (e.g. pub struct vs private)
/// - Add namespace/module support
/// - Add output format toggle (e.g. Rust, JSON, Godot GDScript)
/// - Add field-level config (e.g. default values, optional fields)
#[derive(Default)]
pub struct Config {
    /// Emit `#[derive(Debug)]`
    pub derive_debug: bool,

    /// Emit `#[derive(Clone)]`
    pub derive_clone: bool,

    /// Emit `impl Display` block
    pub emit_display_impl: bool,

    /// Include `#[derive(Serialize, Deserialize)]`
    pub use_serde: bool,
}
