AetherionEngine is a mythic core—a modular, dimension-agnostic 
procedural generation engine coded in Rust as a GDExtension 
for Godot 4+(5, 6 ->).

Directory/File Structure:

lexicon: ├── └── │



c:/zv9/zv9.aetherion/rust/src/
├── aetherion/                             # 🧠 Core engine modules for procedural generation and runtime orchestration
│   ├── core/                              # Lifecycle state, runtime ticks, and dimensional context
│   │   ├── dimension.rs                   # 2D/3D abstraction and switching logic
│   │   ├── lifecycle.rs                   # Engine state transitions and signal hooks
│   │   ├── runtime.rs                     # Tick progression and frame budget tracking
│   │   └── mod.rs                         # Core module exports
│   │
│   ├── generator/                         # Procedural content generation and configuration
│   │   ├── noise.rs                       # Noise algorithms (Basic, Cellular Automata, etc.)
│   │   ├── patterns.rs                    # Pattern overlays and spatial logic
│   │   ├── noise_config.rs               # Grid generation parameters and evolution rules
│   │   └── mod.rs                         # Generator module exports
│   │
│   ├── pipeline/                          # Map construction, chunk streaming, and builder orchestration
│   │   ├── builder/                       # Builder trait and threaded execution
│   │   │   ├── builder.rs                # Map builder logic and grid-to-chunk conversion
│   │   │   ├── threaded.rs              # Threaded builder dispatch with signal streaming
│   │   │   └── mod.rs
│   │   ├── data/                          # Map data structures and build options
│   │   │   ├── chunk.rs                 # MapDataChunk container
│   │   │   ├── tile.rs                  # TileInfo metadata
│   │   │   ├── options.rs               # MapBuildOptions and GodotNoiseType
│   │   │   └── mod.rs
│   │   └── mod.rs                         # Pipeline module exports
│   │
│   └── mod.rs                             # Re-exports core, generator, and pipeline modules
│
├── godot4/                                # 🎮 Godot engine integration and runtime bridge
│   ├── api/                               # Public-facing Godot classes and engine interface
│   │   ├── engine.rs                     # AetherionEngine lifecycle and control
│   │   ├── signals.rs                    # AetherionSignals for Godot dispatch
│   │   ├── generator.rs                  # AetherionGenerator exposing procedural logic
│   │   ├── config.rs                     # AetherionConfig for runtime parameters
│   │   ├── map.rs                        # AetherionMap for tile/voxel state
│   │   └── mod.rs
│   │
│   ├── bindings/                          # Type conversion between Rust and Godot
│   │   ├── godot_types.rs                # Serializable Godot-compatible types
│   │   └── mod.rs
│   │
│   ├── messaging/                         # Signal queue and engine message definitions
│   │   ├── messages.rs                   # EngineMessage enum for status and progress
│   │   ├── sync.rs                       # GodotSync queue for async dispatch
│   │   └── mod.rs
│   │
│   ├── signals/                           # Signal definitions and dispatch logic
│   │   ├── definitions.rs                # Signal metadata and identifiers
│   │   ├── dispatch.rs                   # Signal routing and emission
│   │   └── mod.rs
│   │
│   └── mod.rs                             # Godot integration exports
│
├── util/                                  # 🛠 Internal utilities and diagnostics
│   ├── config.rs                         # Internal config structs and helpers
│   ├── logging.rs                        # Logging macros and tracing utilities
│   ├── timing.rs                         # Tick budget and time management
│   └── mod.rs
│
├── shared/                                # 🧬 Shared types, traits, and math utilities
│   ├── math.rs                           # Vector math and spatial calculations
│   ├── types.rs                          # Common type aliases and primitives
│   ├── traits.rs                         # Core traits (Builder, Generator, etc.)
│   └── mod.rs
│
├── plugin/                                # 🧩 Godot plugin registration and entry point
│   ├── gdextension.rs                    # GDExtension entry point for Godot
│   ├── registration.rs                  # Class registration via add_class::<T>()
│   └── mod.rs
│
├── tests/                                 # 🧪 Integration and unit tests
│   ├── generation_tests.rs              # Noise, pattern, and tile placement tests
│   ├── pipeline_tests.rs                # Builder logic, chunk streaming, threading
│   ├── data_tests.rs                    # MapDataChunk, TileInfo, and serialization
│   ├── signal_tests.rs                  # EngineMessage and GodotSync behavior
│   ├── trait_tests.rs                   # Custom trait implementations (e.g., maze builders)
│   └── common.rs                        # Shared test utilities and fixtures
│
├── lib.rs                                 # 🚪 Crate entry point and module wiring
└── prelude.rs                             # 🌌 Common imports for ergonomic access
                  

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

========================
AETHERION GODOT API REFERENCE
========================

--------------------------------
AetherionConfig — Engine Configuration
--------------------------------
Purpose:
  Holds procedural generation parameters. Can be tweaked in the inspector or via GDScript.

Exported Fields:
  tile_size: int                # Size of a single tile in pixels/units
  chunk_width: int              # Width of a chunk in tiles
  chunk_height: int             # Height of a chunk in tiles
  seed: int                     # RNG seed for generation
  enable_voxel_mode: bool       # Toggle voxel mode

Commands:
  _ready() -> void
    Logs “⚙️ AetherionConfig loaded.”

  get_chunk_area() -> int
    Returns chunk_width * chunk_height.

  regenerate_seed() -> void
    Generates a new random seed.

Example:
  config.chunk_width = 16
  config.chunk_height = 16
  config.regenerate_seed()

--------------------------------
AetherionEngine — Core Runtime Controller
--------------------------------
Purpose:
  Orchestrates procedural generation, processes chunks, and emits signals.

Exported Fields:
  signals_node: AetherionSignals   # Node to emit engine signals
  target_tilemap: TileMap          # TileMap to place generated tiles into

Commands:
  aetherionoracle() -> void
    Manually processes queued chunks/signals once.

  build_map(width: int, height: int, seed: int, mode: String, animate: bool, black: Vector2i, blue: Vector2i) -> void
    Starts threaded map generation. mode is "automata" or "basic".

  set_tilemap(tilemap: TileMap) -> void
    Sets the target TileMap for placement.

  debug_place_tile(x: int, y: int) -> void
    Places a test tile at (x, y) in the target TileMap.

Example:
  engine.signals_node = $Signals
  engine.set_tilemap($TileMap)
  engine.build_map(100, 100, config.seed, "basic", true, Vector2i(0,0), Vector2i(1,0))

--------------------------------
AetherionSignals — Engine Event Emitter
--------------------------------
Purpose:
  Emits signals from the Rust core. Connect to these in GDScript to react to engine events.

Core Generation Signals:
  build_map_start()
  generation_progress(percent: int)
  generation_complete(results: Dictionary)
  map_building_status(status_message: GString)

Lifecycle & Diagnostics:
  tick_started()
  tick_completed()
  frame_budget_exceeded()
  engine_init_start()
  engine_initialized()
  pipeline_start()
  pipeline_complete()
  sync_required()
  rust_error()

Tilemap & Map Events:
  map_chunk_ready()
  tilemap_updated()
  tilemap_sync_complete()
  map_build_cancelled()
  map_build_failed()
  map_build_duration(duration: float)
  rust_extension_ready()

Example:
  signals.connect("generation_progress", self, "_on_progress")
  signals.connect("generation_complete", self, "_on_complete")

--------------------------------
AetherionGenerator — Procedural Tile Generator
--------------------------------
Purpose:
  Generates individual tiles procedurally for ad-hoc placement or testing.

Commands:
  generate_noise(x: float, y: float, seed: int) -> Dictionary
    Creates a tile based on noise at (x, y) with given seed.

  generate_pattern(pattern_name: String, x: int, y: int) -> Dictionary
    Creates a tile based on a named pattern.

Tile Dictionary Format:
  {
    "source_id": int,
    "atlas_coords": Vector2i,
    "alternate_id": int,
    "rotation": int,
    "layer": int
  }

Example:
  var tile = generator.generate_noise(1.0, 2.0, 42)
  tilemap.set_cell(0, Vector2i(5, 5), tile.source_id, tile.atlas_coords)

--------------------------------
AetherionMap — Runtime Tile/Voxel State
--------------------------------
Purpose:
  Holds the current map chunk in memory, accepts tile data from GDScript, and lets you query or clear it.

Commands:
  load_chunk(tiles: Array<Variant>) -> void
    Loads a chunk from raw tile data, skipping invalid entries.
    Each element should be a Dictionary with:
      source_id: int
      atlas_coords: Vector2i
      alternate_id: int
      rotation: int (0–255)
      layer: int (0–255)

  get_tile(index: int) -> Dictionary
    Retrieves tile info at the given index. Logs a warning if not found.

  clear_chunk() -> void
    Clears the current chunk from memory.

Example:
  var tiles = []
  tiles.append({
    "source_id": 0,
    "atlas_coords": Vector2i(0, 0),
    "alternate_id": 0,
    "rotation": 0,
    "layer": 0
  })
  map.load_chunk(tiles)
  var tile_data = map.get_tile(0)
  print(tile_data)
