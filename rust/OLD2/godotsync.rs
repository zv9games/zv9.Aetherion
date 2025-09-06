use godot::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GodotSync {
    #[base]
    base: Base<Node>,
    inner: Arc<Mutex<GodotSyncInner>>,
}

// GodotSyncInner can still derive Default as it doesn't contain a `Base` object
#[derive(Default)]
pub struct GodotSyncInner {
    tiles: Vec<TilePlacementMessage>,
    signals: Vec<EngineMessage>,
}

#[derive(Clone, Debug)]
pub struct TilePlacementMessage {
    pub position: Vector2i,
    pub info: TileInfo,
}

#[derive(Clone, Debug)]
pub struct TileInfo {
    pub source_id: i32,
    pub atlas_coords: Vector2i,
    pub alternate_id: i32,
}

#[derive(Clone, Debug)]
pub enum EngineMessage {
    Progress(i32),
    Status(String),
    Complete {
        width: i32,
        height: i32,
        mode: String,
        animate: bool,
        duration: f64,
    },
}

#[godot_api]
impl GodotSync {
    // This `init` is what the `#[class(init)]` macro uses to create your instance.
    // It's not a regular constructor.
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            // The `GodotSyncInner` part can be defaulted, as it doesn't contain a `Base` object.
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
        }
    }

    // This is the public method called by `AetherionEngine`.
    // It is not a Godot-specific initializer.
    pub fn new_instance() -> Self {
        // We can't use `init` directly. A better pattern is to have a regular `new` or `init`
        // function for your data-holding part.
        Self {
            base: Base::from_object(Gd::new_default()), // This part is a bit tricky, might need a different approach
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
        }
    }

    // Let's re-think `GodotSync`. It's a GodotClass itself, so it should be instantiated
    // via Godot's system. It shouldn't be created inside `AetherionEngine`.
    // A better approach is to have a single `GodotSync` node in the scene tree and let
    // `AetherionEngine` get a reference to it.
    // But since the original code wants to create it, let's go with the recommended pattern.
    pub fn init() -> Self {
        // This is where we will create a new instance of the GodotSync node.
        // We need to use `Gd::new_init` for this.
        let sync_instance = Gd::new_init(|base| Self::init(base));
        Self {
            base: sync_instance.base,
            inner: Arc::new(Mutex::new(GodotSyncInner::default())),
        }
    }


    #[func]
    fn _ready(&mut self) {
        godot_print!("🛠️ Initializing GodotSync");
        // Get the Godot object to use its methods
        let mut base_gd = self.base.to_gd();
        
        base_gd.set_process(true);
        if let Some(mut tree) = base_gd.get_tree() {
            // `create_timer` takes only one argument now
            let mut timer = tree.create_timer(0.1).expect("Failed to create timer");
            timer.connect("timeout", &Callable::from_object_method(&base_gd, "_process"));
            godot_print!("🛠️ Started timer for GodotSync _process");
        } else {
            godot_error!("⚠️ SceneTree not available in GodotSync _ready");
        }
    }

    #[func]
    fn _process(&mut self, _delta: f64) {
        // The previous code here was trying to call `AetherionEngine._process` from `GodotSync._process`.
        // This is a circular and incorrect design.
        // The correct approach is for `AetherionEngine` to poll `GodotSync` for messages, which it already does.
        // We can remove this function entirely.
    }

    pub fn add_tiles(&self, tiles: Vec<TilePlacementMessage>) {
        let mut inner = self.inner.lock().expect("Lock poisoned");
        godot_print!("🛠️ Adding {} tiles to GodotSync", tiles.len());
        inner.tiles.extend(tiles);
        godot_print!("🛠️ GodotSync now has {} tiles", inner.tiles.len());
    }

    pub fn drain_tiles(&self) -> Vec<TilePlacementMessage> {
        let mut inner = self.inner.lock().expect("Lock poisoned");
        godot_print!("🛠️ Draining {} tiles from GodotSync", inner.tiles.len());
        let tiles = inner.tiles.drain(..).collect();
        godot_print!("🛠️ After drain, GodotSync has {} tiles", inner.tiles.len());
        tiles
    }

    pub fn add_signal(&self, signal: EngineMessage) {
        let mut inner = self.inner.lock().expect("Lock poisoned");
        godot_print!("🛠️ Adding signal: {:?}", signal);
        inner.signals.push(signal);
        godot_print!("🛠️ GodotSync now has {} signals", inner.signals.len());
    }

    pub fn drain_signals(&self) -> Vec<EngineMessage> {
        let mut inner = self.inner.lock().expect("Lock poisoned");
        godot_print!("🛠️ Draining {} signals from GodotSync", inner.signals.len());
        let signals = inner.signals.drain(..).collect();
        godot_print!("🛠️ After drain, GodotSync has {} signals", inner.tiles.len());
        signals
    }
}