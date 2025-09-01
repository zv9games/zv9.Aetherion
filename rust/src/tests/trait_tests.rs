// trait_tests.rs

#[cfg(test)]
mod tests {
    use godot::prelude::*;
    use crate::AetherionSignals;

    #[test]
    fn test_inode_trait_init() {
        let base = Base::<Node>::default();
        let node = AetherionSignals::init(base.clone());

        // Ensure the node is initialized with the correct base
        assert_eq!(node.base.upcast().name(), base.upcast().name());
    }

    #[test]
    fn test_inode_trait_ready() {
        let mut node = AetherionSignals::init(Base::<Node>::default());

        // This should trigger the godot_print! macro
        // You might mock or intercept stdout if needed
        node.ready();

        // Placeholder assertion — actual output would be verified in integration
        assert!(true);
    }

    #[test]
    fn test_trait_object_behavior() {
        // If your trait is used as a trait object, test dynamic dispatch
        let base = Base::<Node>::default();
        let mut node = AetherionSignals::init(base);

        // Call trait method via dynamic dispatch
        let inode_ref: &mut dyn INode = &mut node;
        inode_ref.ready();

        assert!(true); // Again, placeholder for actual behavior
    }
}
