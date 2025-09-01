AetherionEngine is a mythic core—a modular, dimension-agnostic 
procedural generation engine coded in Rust as a GDExtension 
for Godot 4+(5, 6 ->).

Directory/File Structure:

lexicon: ├── └── │



c:/zv9/zv9.aetherion/rust/src/
├── aetherion/                             # 🧠 Core procedural engine
│   ├── core/                              # Engine lifecycle and runtime control
│   │   ├── dimension.rs
│   │   ├── lifecycle.rs
│   │   ├── runtime.rs
│   │   └── mod.rs
│   │
│   ├── generator/                         # Procedural generation logic
│   │   ├── noise.rs                       # Perlin, Simplex, etc.
│   │   ├── patterns.rs                    # Pattern-based placement
│   │   ├── noise_config.rs               # Configurable noise settings
│   │   └── mod.rs
│   │
│   ├── pipeline/                          # Map building and data flow
│   │   ├── builder/
│   │   │   ├── builder.rs                # Generic Builder trait
│   │   │   ├── threaded.rs              # Threaded build execution
│   │   │   └── mod.rs
│   │   ├── data/
│   │   │   ├── chunk.rs                 # MapDataChunk
│   │   │   ├── tile.rs                  # TileInfo
│   │   │   ├── options.rs               # MapBuildOptions
│   │   │   └── mod.rs
│   │   └── mod.rs
│   │
│   └── mod.rs                            # Re-exports core, generator, pipeline
│
├── godot4/                                # 🎮 Godot integration layer
│   ├── api/                               # Public Godot-facing classes
│   │   ├── engine.rs                     # AetherionEngine
│   │   ├── signals.rs                    # AetherionSignals
│   │   ├── generator.rs                  # AetherionGenerator (exposes noise/patterns)
│   │   ├── config.rs                     # AetherionConfig (tile size, seed, etc.)
│   │   ├── map.rs                        # AetherionMap (runtime tile/voxel state)
│   │   └── mod.rs
│   │
│   ├── bindings/                          # Rust ↔ Godot type conversions
│   │   ├── godot_types.rs
│   │   └── mod.rs
│   │
│   ├── messaging/                         # Async communication
│   │   ├── messages.rs                   # EngineMessage enum
│   │   ├── sync.rs                       # GodotSync queue
│   │   └── mod.rs
│   │
│   ├── signals/                           # Signal definitions and dispatch
│   │   ├── definitions.rs
│   │   ├── dispatch.rs
│   │   └── mod.rs
│   │
│   └── mod.rs
│
├── util/                                  # 🛠 Utility library
│   ├── config.rs                         # Internal config structs
│   ├── logging.rs                        # Logging macros/utilities
│   ├── timing.rs                         # Tick/budget management
│   └── mod.rs
│
├── shared/                                # Common types and traits
│   ├── math.rs
│   ├── types.rs
│   ├── traits.rs                         # Builder, Generator, etc.
│   └── mod.rs
│
├── plugin/                                # 🧩 Godot plugin registration
│   ├── gdextension.rs                    # GDExtension entry point
│   ├── registration.rs                  # add_class::<T>() calls
│   └── mod.rs
│
├── tests/
│	├── generation_tests.rs         # Noise, patterns, tile placement
│	├── pipeline_tests.rs           # Builder, chunk streaming, threading
│	├── data_tests.rs               # MapDataChunk, TileInfo, serialization
│	├── signal_tests.rs             # EngineMessage, GodotSync
│	├── trait_tests.rs              # Custom trait impls (e.g., PacmanMazeBuilder)
│	└── common.rs                   # Shared test utilities
│
│
├── lib.rs                                 # Entry point
└── prelude.rs                             # Common imports

╔══════════════════════════════════════════════════════════════════════════╗
║ 🧪 AETHERION TESTER — Ritual Interface Blueprint                         ║
║                                                                          ║
║ A graphical showcase for AetherionEngine’s procedural capabilities.      ║
║ Driven by GDScript, powered by Rust, and designed to demonstrate         ║
║ real-time tile placement, signal flow, and engine flexibility.           ║
║                                                                          ║
╠══════════════════════════════════════════════════════════════════════════╣
║ 🎛️ CONTROL PANEL                                                        ║
║                                                                          ║
║ • Grid Size: SpinBox (e.g. 100×100 to 2000×2000)                         ║
║ • Seed Input: LineEdit or SpinBox                                       ║
║ • Tile Types: OptionButton (Black, Blue, Custom)                        ║
║ • Placement Mode: OptionButton (Random, Checkerboard, Clustered)       ║
║ • Generate Button: Triggers Rust build method                           ║
║                                                                          ║
╠══════════════════════════════════════════════════════════════════════════╣
║ 🧱 TILEMAP PREVIEW                                                      ║
║                                                                          ║
║ • TileMap node with Camera2D                                            ║
║ • Zoom and pan controls                                                 ║
║ • Optional animation: fade-in, pulse, wave                              ║
║ • Debug overlay layer for placement visualization                       ║
║                                                                          ║
╠══════════════════════════════════════════════════════════════════════════╣
║ 📊 PERFORMANCE METRICS                                                  ║
║                                                                          ║
║ • Tile count                                                            ║
║ • Placement time (from Rust)                                            ║
║ • FPS and memory usage                                                  ║
║ • Live updates via signals or polling                                   ║
║                                                                          ║
╠══════════════════════════════════════════════════════════════════════════╣
║ 🔔 SIGNAL ECHO CONSOLE                                                  ║
║                                                                          ║
║ • RichTextLabel or TextEdit                                             ║
║ • Streams: build_map_start, map_building_status, generation_complete   ║
║ • Color-coded messages                                                  ║
║ • Optional timestamping                                                 ║
║                                                                          ║
╠══════════════════════════════════════════════════════════════════════════╣
║ 🖱️ INTERACTIVE TILE PLACEMENT                                          ║
║                                                                          ║
║ • Click to place tile                                                   ║
║ • Drag to paint, right-click to erase                                   ║
║ • Sends coordinates to Rust for mutation                                ║
║ • Optional undo/redo buffer                                             ║
║                                                                          ║
╠══════════════════════════════════════════════════════════════════════════╣
║ 🧠 PRESET LOADER + EXPORTER                                             ║
║                                                                          ║
║ • Save/load generation presets                                          ║
║ • Export tile data to JSON or custom formats                            ║
║ • Future-ready for Godot Asset Library plugin                           ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝

