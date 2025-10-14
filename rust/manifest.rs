AetherionEngine is a mythic core — a modular, dimension-agnostic  
procedural generation engine coded in Rust as a GDExtension  
for Godot 4+ (5, 6 →).

🪶 Manifest v1.0.seed

────────────────────────────────────────────────────────────
📁 Directory/File Structure

Legend:
├── Directory
│   Subdirectory
└── File

📎 Tagging Convention (for code, docs, and rituals)
@module:*       @signal:*       @ritual:*       @todo:*
@scene:*        @script:*       @binding:*      @asset:*
@config:*

────────────────────────────────────────────────────────────
📦 C:/ZV9/ — Root Workspace

├── zv9.aetherion/                # 📦 Unified Package: Rust Engine + Godot Tester
│   ├── .assets/                  # 🎨 Visual assets and overlays
│   ├── .git/                     # 🧬 Git history
│   │   
│   ├── aetherion_engine_tester/  # 🧪 Godot testbed and integration shell
│   │   ├── .godot/               # 🧱 Godot internal cache
│   │   ├── addons/               # 🔌 Plugin scripts and bindings
│   │   ├── root_scenes/          # 🎬 Main scene files
│   │   ├── root_scripts/         # 📜 Main GDScript logic
│   │   ├── aetherion.gdextension # 🔗 GDExtension config
│   │   └── project.godot         # 🧭 Godot project descriptor
│   │   
│   ├── cargo-aetherion/          # 🚚 CLI tools and cargo wrappers
│   │   ├── src/
│   │   └── target/
│   │   
│   ├── rust/                     # 🧠 Core engine source (Rust)
│   │   ├── Cargo.lock
│   │   ├── cargo.toml
│   │   ├── manifest.rs
│   │   │
│   │   ├── aetherion_binary/     # 🧃 Binary tools and sync macros
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── zv9_util_binary_func.rs
│   │   │       ├── zv9_util_binary_func2.rs
│   │   │       ├── zv9_util_binary_func3.rs
│   │   │       ├── zv9_util_binary_func_godot.rs
│   │   │       └── zv9_util_binary_menu.rs
│   │   │
│   │   ├── aetherion_core/       # 🧠 Core procedural logic and orchestration
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── [Flat module files listed individually below]
│   │   │       ├── lib.rs
│   │   │       ├── zv9_prelude.rs
│   │   │       ├── zv9_aetherion_codegen_config.rs
│   │   │       ├── zv9_aetherion_codegen_dsl.rs
│   │   │       ├── zv9_aetherion_codegen_emitter.rs
│   │   │       ├── zv9_aetherion_codegen_parser.rs
│   │   │       ├── zv9_aetherion_core_conductor.rs
│   │   │       ├── zv9_aetherion_core_dimension.rs
│   │   │       ├── zv9_aetherion_core_lifecycle.rs
│   │   │       ├── zv9_aetherion_core_runtime.rs
│   │   │       ├── zv9_aetherion_generator_noise.rs
│   │   │       ├── zv9_aetherion_generator_noise_config.rs
│   │   │       ├── zv9_aetherion_generator_patterns.rs
│   │   │       ├── zv9_aetherion_generator_pattern_type.rs
│   │   │       ├── zv9_aetherion_interaction_modifiers.rs
│   │   │       ├── zv9_aetherion_interaction_tools.rs
│   │   │       ├── zv9_aetherion_pipeline_builder_bitmask.rs
│   │   │       ├── zv9_aetherion_pipeline_builder_builder.rs
│   │   │       ├── zv9_aetherion_pipeline_builder_dummy_delivery.rs
│   │   │       ├── zv9_aetherion_pipeline_builder_streamer.rs
│   │   │       ├── zv9_aetherion_pipeline_builder_threaded.rs
│   │   │       ├── zv9_aetherion_pipeline_data_chunk.rs
│   │   │       ├── zv9_aetherion_pipeline_data_data.rs
│   │   │       ├── zv9_aetherion_pipeline_data_grid.rs
│   │   │       ├── zv9_aetherion_pipeline_data_tile.rs
│   │   │       ├── zv9_aetherion_structure_generation.rs
│   │   │       ├── zv9_aetherion_structure_placement.rs
│   │   │       ├── zv9_shared_grid2d.rs
│   │   │       ├── zv9_shared_grid_bounds.rs
│   │   │       ├── zv9_shared_math.rs
│   │   │       ├── zv9_shared_messages.rs
│   │   │       ├── zv9_shared_spatial.rs
│   │   │       ├── zv9_shared_traits.rs
│   │   │       ├── zv9_shared_types.rs
│   │   │
│   │   │       ├── zv9_util_config.rs
│   │   │       ├── zv9_util_direction.rs
│   │   │       ├── zv9_util_logging.rs
│   │   │       ├── zv9_util_position.rs
│   │   │       ├── zv9_util_profiling.rs
│   │   │       ├── zv9_util_random.rs
│   │   │       ├── zv9_util_time.rs
│   │   │       ├── zv9_util_timer.rs
│   │   │       ├── zv9_util_velocity.rs
│   │
│   │   ├── aetherion_engine/     # 🎮 Godot-facing bindings and API surface
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── zv9_aetherion_engine_queue.rs
│   │   │       ├── zv9_godot_interface_api_config.rs
│   │   │       ├── zv9_godot_interface_api_engine.rs
│   │   │       ├── zv9_godot_interface_api_engine_core.rs
│   │   │       ├── zv9_godot_interface_api_engine_signals.rs
│   │   │       ├── zv9_godot_interface_api_engine_util.rs
│   │   │       ├── zv9_godot_interface_api_generator.rs
│   │   │       ├── zv9_godot_interface_api_map.rs
│   │   │       ├── zv9_godot_interface_api_oracle.rs
│   │   │       ├── zv9_godot_interface_api_signals.rs
│   │   │       ├── zv9_godot_interface_bindings_godot_types.rs
│   │   │       ├── zv9_godot_interface_emulator.rs
│   │   │       ├── zv9_godot_interface_interface_controls.rs
│   │   │       ├── zv9_godot_interface_interface_diagnostics.rs
│   │   │       ├── zv9_godot_interface_map_ext.rs
│   │   │       ├── zv9_godot_interface_messaging_sync.rs
│   │   │       ├── zv9_godot_interface_signals_definitions.rs
│   │   │       ├── zv9_godot_interface_signals_dispatch.rs
│   │   │       ├── zv9_aetherion_pipeline_data_build_options.rs
│   │   │       ├── zv9_aetherion_pipeline_data_vector.rs
│   │   │       ├── zv9_aetherion_sync_bridge.rs
│   │   │       ├── zv9_api.rs
│   │   │       ├── zv9_lib.rs
│   │   │       ├── zv9_lib_interface.rs
│   │   │       ├── zv9_lib_trailkeeper.rs
│   │   │       └── zv9_prelude.rs
│   │   │
│   │   ├── aetherion_engine/     
│   │   │   ├── Cargo.toml
│   │   │   └── src/     
│   │   │		├── zv9_trailkeeper_collector.rs
│   │   │       ├── zv9_trailkeeper_config.rs
│   │   │       ├── zv9_trailkeeper_entry.rs
│   │   │       ├── zv9_trailkeeper_export.rs
│   │   │       ├── zv9_trailkeeper_macros.rs
│   │   │       ├── zv9_trailkeeper_registry.rs
│   │   │       ├── zv9_trailkeeper_scan.rs
│   │   │       ├── zv9_trailkeeper_watch.rs
│   │   ├── .gitignore
│   │   ├── bfg                    🧹 Git cleanup tool
│   │   ├── LICENSE.md
│   │   └── README.md
│
├── zv9.gdext/                     🔌 GDExtension bindings (compiled output)
├── zv9.godot/                     🏗️ Godot build artifacts and runtime cache

📘 GDScript-Callable Methods

🔹 Node: AetherionGenerator  📁 .\aetherion_engine\src\zv9_godot_interface_api_generator.rs
   └── fn generate_noise(&self, x: f32, y: f32, seed: i64) -> Dictionary {
   └── fn generate_pattern(&self, pattern_name: String, x: i32, y: i32) -> Dictionary {

🔹 Node: AetherionMap  📁 .\aetherion_engine\src\zv9_godot_interface_api_map.rs
   └── fn _ready(&self)
   └── fn set_tilemap(&mut self, tilemap: Gd<TileMap>)
   └── fn load_chunk(&mut self, tiles: Array<Variant>)
   └── fn get_tile(&self, index: i32) -> Dictionary {
   └── fn clear_chunk(&mut self)
   └── fn test_chunk_placement(&mut self)

🔹 Node: ControlPanel  📁 .\aetherion_engine\src\zv9_godot_interface_interface_controls.rs
   └── fn _ready(&self)
   └── fn generate_map(&self)
   └── fn set_pacing(&mut self, ms: i32)
   └── fn apply_preset(&mut self, name: GString)
   └── fn describe_settings(&self) -> String {
   └── fn to_config_dict(&self) -> Dictionary {

🔹 Node: AetherionConfig  📁 .\aetherion_engine\src\zv9_godot_interface_api_config.rs
   └── fn _ready(&self)
   └── fn get_chunk_area(&self) -> i32 {
   └── fn regenerate_seed(&mut self)

🔹 Node: AetherionEngine  📁 .\aetherion_engine\src\zv9_godot_interface_api_engine.rs
   └── fn enter_tree(&mut self)
   └── fn _ready(&mut self)
   └── fn process(&mut self, _delta: f64)
   └── fn emit_pending_signals(&mut self)
   └── fn tick(&mut self, tick: u64)
   └── fn build_map( &mut self, width: i32, height: i32, seed: i64, mode: String, animate: bool, black: Vector2i, blue: Vector2i, )
   └── fn set_tilemap(&mut self, tilemap: Gd<TileMap>)
   └── fn apply_chunks_to_tilemap(&mut self)
   └── fn debug_place_tile(&mut self, x: i32, y: i32)
   └── fn ping(&self)
   └── fn get_status(&self) -> String {
   └── fn set_signals_node(&mut self, node: Gd<AetherionSignals>)

🔹 Node: DiagnosticsOverlay  📁 .\aetherion_engine\src\zv9_godot_interface_interface_diagnostics.rs
   └── fn update_metrics(&mut self, tick: u64, avg_tick: f64, queue_len: i32)
   └── fn _ready(&self)

🔹 Node: AetherionSignals  📁 .\aetherion_engine\src\zv9_godot_interface_api_signals.rs
   └── fn _ready(&mut self)

🔹 Node: BuildOptions  📁 .\aetherion_engine\src\zv9_aetherion_pipeline_data_build_options.rs
   └── fn apply_preset(&mut self, name: GString)

🔹 Node: AetherionOracle  📁 .\aetherion_engine\src\zv9_godot_interface_api_oracle.rs
   └── fn _ready(&mut self)
   └── fn set_engine(&mut self, engine: Gd<AetherionEngine>)
   └── fn tick(&mut self)
   └── fn ping(&self)
   └── fn reset(&mut self)
   └── fn get_tick(&self) -> u64 {

📊 Summary:
   Files scanned: 1076
   Classes found: 9
   Total methods: 39
   Orphan #[func] methods: 9

✅ GDScript-callable methods printed.
