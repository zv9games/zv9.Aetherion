use godot::prelude::*;
use crate::godot4::api::signals::AetherionSignals;

#[test]
fn test_signal_initialization() {
    let base_node = Node::new_alloc(); // Proper Godot node instantiation
    let base = Base::from(base_node);

    let mut scene = AetherionSignals::init(base);
    assert!(scene.is_ready(), "Scene should be initialized and ready");
    println!("✅ test_signal_initialization");
}

#[test]
fn test_signal_dispatch() {
    let base_node = Node::new_alloc();
    let base = Base::from(base_node);

    let mut scene = AetherionSignals::init(base);
    scene.emit_custom_signal("test_signal", vec![]);
    assert!(scene.signal_emitted("test_signal"), "Signal should be emitted");
    println!("✅ test_signal_dispatch");
}
