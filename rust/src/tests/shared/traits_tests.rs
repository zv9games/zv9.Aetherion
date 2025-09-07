use godot::prelude::*;
use crate::godot4::api::signals::{AetherionSignals, SignalReceiver};

#[test]
fn test_receiver_behavior() {
    let base_node = Node::new_alloc();
    let base = Base::from(base_node);

    let mut receiver = SignalReceiver::init(base);
    receiver.receive("ping");
    assert!(receiver.last_received() == "ping", "Receiver should store last signal");
    println!("✅ test_receiver_behavior");
}

#[test]
fn test_signals_behavior() {
    let base_node = Node::new_alloc();
    let base = Base::from(base_node);

    let mut signals = AetherionSignals::init(base);
    signals.emit_custom_signal("hello", vec![]);
    assert!(signals.signal_emitted("hello"), "Signal should be emitted");
    println!("✅ test_signals_behavior");
}
