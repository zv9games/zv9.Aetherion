AetherionEngine is a mythic core—a modular, dimension-agnostic 
procedural generation engine coded in Rust as a GDExtension 
for Godot 4+(5, 6 ->).

Directory/File Structure:

lexicon: ├── └── │



c:/zv9/zv9.aetherion/rust/src/

├── aetherion/                             # 🧠 Core engine modules for procedural generation and runtime orchestration
│   ├── codegen/                           # 🛠 DSL parsing and Rust code emission for procedural entities
│   │   ├── config.rs                      # Configuration options for code generation
│   │   ├── dsl.rs                         # DSL syntax definitions and parsing logic
│   │   ├── emitter.rs                     # Emits Rust code from parsed DSL structures
│   │   ├── mod.rs                         # Codegen module exports
│   │   └── parser.rs                      # Parses DSL into intermediate representations
│   │
│   ├── core/                              # ⏱ Runtime state, lifecycle, and orchestration
│   │   ├── conductor.rs                   # 🎼 Procedural command queue and orchestration logic
│   │   ├── dimension.rs                   # 2D/3D abstraction and switching logic
│   │   ├── lifecycle.rs                   # Engine state transitions and signal hooks
│   │   ├── mod.rs                         # Core module exports
│   │   └── runtime.rs                     # Tick progression and frame budget tracking
│   │
│   ├── generator/                         # ⚙️ Procedural content generation and configuration
│   │   ├── mod.rs                         # Generator module exports
│   │   ├── noise.rs                       # Noise algorithms (Basic, Cellular Automata, etc.)
│   │   ├── noise_config.rs                # Grid generation parameters and evolution rules
│   │   ├── pattern_type.rs                # Enum/type definitions for pattern overlays
│   │   └── patterns.rs                    # Pattern overlays and spatial logic
│   │
│   ├── interaction/                       # ✏️ In-game editing, modifiers, and tooling
│   │   ├── mod.rs                         # Interaction module exports
│   │   ├── modifiers.rs                   # Tile toggling, layer editing, and runtime mutation
│   │   └── tools.rs                       # Brush types, selection logic, and paint modes
│   │ 
│   ├── pipeline/                          # 🚚 Map construction, chunk streaming, and builder orchestration
│   │   ├── builder/                       # Builder trait and threaded execution
│   │   │   ├── builder.rs                 # Map builder logic and grid-to-chunk conversion
│   │   │   ├── mod.rs                     # Builder module exports
│   │   │   ├── streamer.rs                # 🧠 Smart chunk pacing, throttling, and prioritization
│   │   │   └── threaded.rs                # Threaded builder dispatch with signal streaming
│   │   │
│   │   ├── data/                          # Map data structures and build options
│   │   │   ├── chunk.rs                   # MapDataChunk container
│   │   │   ├── data.rs                    # Core map data logic
│   │   │   ├── grid.rs                    # Grid layout and spatial indexing
│   │   │   ├── map_build_options.rs       # Build options for procedural generation
│   │   │   ├── mod.rs                     # Data module exports
│   │   │   ├── options.rs                 # MapBuildOptions and GodotNoiseType
│   │   │   ├── tile.rs                    # TileInfo metadata and structure
│   │   │   └── vector.rs                  # Vector math and spatial utilities
│   │   └── mod.rs                         # Pipeline module exports
│   │
│   ├── structure/                         # 🧩 Spatial structures and placement logic
│   │   ├── generation.rs                  # Maze, dungeon, and grid structure algorithms
│   │   ├── mod.rs                         # Structure module exports
│   │   └── placement.rs                   # Structure-to-tilemap conversion and overlays
│   └── mod.rs                             # Re-exports core, generator, pipeline, structure, and interaction
│      
├── bin/                                   # 🖥️ Executable entry points and CLI tools
│   ├── mod.rs                             # Binary module exports
│   ├── aetherion_binary.rs                # Dev console and runtime diagnostics
│   ├── sync_audit.rs                      # Audit tool for sync state
│   └── sync_to_godot.rs                   # Sync bridge to Godot runtime
│   
├── docs/                                  # 📚 Internal documentation and manifest definitions
│   ├── manifest.rs                        # Engine manifest and metadata
│   └── mod.rs                             # Docs module exports
│
├── examples/                              # 🧪 Example projects and engine demos
│   ├── infinity.rs                        # Infinite terrain demo
│   ├── mod.rs                             # Examples module exports
│   ├── expansive.rs                       # Expansive map generation demo
│   └── racing.rs                          # Racer movement and physics demo
│
├── godot4/                                # 🎮 Godot engine integration and runtime bridge
│   ├── api/                               # Public-facing Godot classes and engine interface
│   │   ├── config.rs                      # AetherionConfig for runtime parameters
│   │   ├── engine.rs                      # AetherionEngine lifecycle and control
│   │   ├── generator.rs                   # AetherionGenerator exposing procedural logic
│   │   ├── map.rs                         # AetherionMap for tile/voxel state
│   │   ├── mod.rs                         # API module exports
│   │   ├── oracle.rs                      # Runtime query interface
│   │   └── signals.rs                     # AetherionSignals for Godot dispatch
│   │
│   ├── bindings/                          # 🔗 Type conversion between Rust and Godot
│   │   ├── godot_types.rs                 # Serializable Godot-compatible types
│   │   └── mod.rs                         # Bindings module exports
│   │
│   ├── interface/                         # 🧭 Godot-facing UI components and debug overlays
│   │   ├── controls.rs                    # UI toggles for in-game editing and runtime interaction
│   │   ├── diagnostics.rs                 # Real-time metrics, chunk flow, and engine status
│   │   └── mod.rs                         # Interface module exports
│   │
│   ├── messaging/                         # 📡 Signal queue and engine message definitions
│   │   ├── messages.rs                    # EngineMessage enum for status and progress
│   │   ├── mod.rs                         # Messaging module exports
│   │   └── sync.rs                        # GodotSync queue for async dispatch
│   │
│   ├── signals/                           # 📢 Signal definitions and dispatch logic
│   │   ├── definitions.rs                 # Signal metadata and identifiers
│   │   ├── dispatch.rs                    # Signal routing and emission
│   │   └── mod.rs                         # Signals module exports
│   │
│   └── mod.rs                             # Godot integration exports
│
├── shared/                                # 🧬 Shared types, traits, and math utilities
│   ├── grid_bounds.rs                     # Grid boundary logic and region math
│   ├── grid2d.rs                          # 2D grid utilities
│   ├── math.rs                            # Vector math and spatial calculations
│   ├── mod.rs                             # Shared module exports
│   ├── spatial.rs                         # 📐 Adjacency, bounds, region logic, and pathfinding stubs
│   ├── traits.rs                          # Core traits (Builder, Generator, etc.)
│   └── types.rs                           # Common type aliases and primitives
│
├── tests/                                 # 🧪 Integration and unit tests
│   ├── core/                              # Tests for core engine modules
│   │   ├── conductor_tests.rs
│   │   ├── dimension_tests.rs
│   │   ├── lifecycle_tests.rs
│   │   ├── mod.rs
│   │   └── runtime_tests.rs
│   ├── godot4/                            # Tests for Godot integration
│   │   ├── signal_tests.rs
│   │   └── sync_tests.rs
│   ├── pipeline/                          # Tests for pipeline and builder logic
│   │   ├── chunk_tests.rs
│   │   ├── grid_tests.rs
│   │   └── map_build_tests.rs
│   ├── shared/                            # Tests for shared math and traits
│   │   ├── math_tests.rs
│   │   ├── traits_tests.rs
│   │   └── types_tests.rs
│   ├── util/                              # Tests for utility modules
│   │   ├── logging_tests.rs
│   │   ├── profiling_tests.rs
│   │   └── timer_tests.rs
│   │
│   ├── common.rs                          # Shared test utilities and fixtures
│   ├── generation_tests.rs                # Noise, pattern, and tile placement tests
│   ├── godot_integration_tests.rs         # Full-stack Godot engine integration tests
│   ├── mod.rs                             # Test module exports
│   ├── pipeline_tests.rs                  # Builder logic, chunk streaming, threading
│   ├── signal_tests.rs                    # EngineMessage and GodotSync behavior
│   └── trait_tests.rs                     # Custom trait implementations and trait object behavior
│
├── trailkeeper/                           # 📘 Architectural ledger and in-code logging system
│   ├── entry.rs                           # TrailEntry struct and TrailStatus enum (Committed, Pending, Abandoned)
│   ├── macros.rs                          # log_entry! macro for structured in-code annotations
│   └── registry.rs                        # Optional: runtime collector or static ledger of entries
│
├── util/                                  # 🛠 Internal utilities and diagnostics
│   ├── config.rs                          # Internal config structs and helpers
│   ├── logging.rs                         # Logging macros and tracing utilities
│   ├── time.rs                            # Tick budget and time management
│   ├── profiling.rs                       # ⏱ Chunk throughput and performance diagnostics
│   ├── position.rs                        # Spatial position helpers
│   ├── direction.rs                       # Directional logic and orientation helpers
│   ├── timer.rs                           # Runtime timers and tick counters
│   ├── velocity.rs                        # Movement and physics helpers
│   └── mod.rs                             # Utility module exports
│
├── api.rs                                 # 🌐 External API surface (optional entry point for engine consumers)
├── lib.rs                                 # 🚪 Crate entry point and module wiring
└── prelude.rs                             # 🌐 Common imports and trait re-exports for ergonomic use


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

Godot Engine v4.5.beta7.official.4ebf67c12 - https://godotengine.org
Vulkan 1.4.312 - Forward Mobile - Using Device #0: NVIDIA - NVIDIA GeForce RTX 4070 Laptop GPU

🚀 Aetherion classes auto-registered.
AetherionTester: Launching initialization sequence...

🧭 Scene Tree Snapshot:
 ┖╴root
    ┠╴GlobalLogger
    ┖╴aetheriontester
       ┠╴main
       ┃  ┠╴AetherionOracle
       ┃  ┠╴expansive_tilemap
       ┃  ┃  ┖╴Layer0
       ┃  ┠╴tilemap
       ┃  ┃  ┠╴Layer0
       ┃  ┃  ┠╴clocktimer
       ┃  ┃  ┖╴clocklabel
       ┃  ┠╴controlpanel
       ┃  ┃  ┠╴progressbar
       ┃  ┃  ┠╴menutitle
       ┃  ┃  ┠╴gridwidthlabel
       ┃  ┃  ┠╴gridwidthspinbox
       ┃  ┃  ┃  ┠╴@SpinBoxLineEdit@22
       ┃  ┃  ┃  ┃  ┖╴@PopupMenu@9
       ┃  ┃  ┃  ┃     ┠╴@PanelContainer@4
       ┃  ┃  ┃  ┃     ┃  ┖╴@ScrollContainer@5
       ┃  ┃  ┃  ┃     ┃     ┠╴@Control@6
       ┃  ┃  ┃  ┃     ┃     ┠╴_h_scroll
       ┃  ┃  ┃  ┃     ┃     ┠╴_v_scroll
       ┃  ┃  ┃  ┃     ┃     ┖╴_focus
       ┃  ┃  ┃  ┃     ┠╴@Timer@7
       ┃  ┃  ┃  ┃     ┠╴@Timer@8
       ┃  ┃  ┃  ┃     ┠╴@PopupMenu@20
       ┃  ┃  ┃  ┃     ┃  ┠╴@PanelContainer@10
       ┃  ┃  ┃  ┃     ┃  ┃  ┖╴@ScrollContainer@11
       ┃  ┃  ┃  ┃     ┃  ┃     ┠╴@Control@12
       ┃  ┃  ┃  ┃     ┃  ┃     ┠╴_h_scroll
       ┃  ┃  ┃  ┃     ┃  ┃     ┠╴_v_scroll
       ┃  ┃  ┃  ┃     ┃  ┃     ┖╴_focus
       ┃  ┃  ┃  ┃     ┃  ┠╴@Timer@13
       ┃  ┃  ┃  ┃     ┃  ┖╴@Timer@14
       ┃  ┃  ┃  ┃     ┖╴@PopupMenu@21
       ┃  ┃  ┃  ┃        ┠╴@PanelContainer@15
       ┃  ┃  ┃  ┃        ┃  ┖╴@ScrollContainer@16
       ┃  ┃  ┃  ┃        ┃     ┠╴@Control@17
       ┃  ┃  ┃  ┃        ┃     ┠╴_h_scroll
       ┃  ┃  ┃  ┃        ┃     ┠╴_v_scroll
       ┃  ┃  ┃  ┃        ┃     ┖╴_focus
       ┃  ┃  ┃  ┃        ┠╴@Timer@18
       ┃  ┃  ┃  ┃        ┖╴@Timer@19
       ┃  ┃  ┃  ┖╴@Timer@23
       ┃  ┃  ┠╴gridheightlabel
       ┃  ┃  ┠╴gridheightspinbox
       ┃  ┃  ┃  ┠╴@SpinBoxLineEdit@42
       ┃  ┃  ┃  ┃  ┖╴@PopupMenu@29
       ┃  ┃  ┃  ┃     ┠╴@PanelContainer@24
       ┃  ┃  ┃  ┃     ┃  ┖╴@ScrollContainer@25
       ┃  ┃  ┃  ┃     ┃     ┠╴@Control@26
       ┃  ┃  ┃  ┃     ┃     ┠╴_h_scroll
       ┃  ┃  ┃  ┃     ┃     ┠╴_v_scroll
       ┃  ┃  ┃  ┃     ┃     ┖╴_focus
       ┃  ┃  ┃  ┃     ┠╴@Timer@27
       ┃  ┃  ┃  ┃     ┠╴@Timer@28
       ┃  ┃  ┃  ┃     ┠╴@PopupMenu@40
       ┃  ┃  ┃  ┃     ┃  ┠╴@PanelContainer@30
       ┃  ┃  ┃  ┃     ┃  ┃  ┖╴@ScrollContainer@31
       ┃  ┃  ┃  ┃     ┃  ┃     ┠╴@Control@32
       ┃  ┃  ┃  ┃     ┃  ┃     ┠╴_h_scroll
       ┃  ┃  ┃  ┃     ┃  ┃     ┠╴_v_scroll
       ┃  ┃  ┃  ┃     ┃  ┃     ┖╴_focus
       ┃  ┃  ┃  ┃     ┃  ┠╴@Timer@33
       ┃  ┃  ┃  ┃     ┃  ┖╴@Timer@34
       ┃  ┃  ┃  ┃     ┖╴@PopupMenu@41
       ┃  ┃  ┃  ┃        ┠╴@PanelContainer@35
       ┃  ┃  ┃  ┃        ┃  ┖╴@ScrollContainer@36
       ┃  ┃  ┃  ┃        ┃     ┠╴@Control@37
       ┃  ┃  ┃  ┃        ┃     ┠╴_h_scroll
       ┃  ┃  ┃  ┃        ┃     ┠╴_v_scroll
       ┃  ┃  ┃  ┃        ┃     ┖╴_focus
       ┃  ┃  ┃  ┃        ┠╴@Timer@38
       ┃  ┃  ┃  ┃        ┖╴@Timer@39
       ┃  ┃  ┃  ┖╴@Timer@43
       ┃  ┃  ┠╴seedlabel
       ┃  ┃  ┠╴seedlineedit
       ┃  ┃  ┠╴tiletypelabel
       ┃  ┃  ┠╴tiletypeoptionbutton
       ┃  ┃  ┃  ┖╴@PopupMenu@49
       ┃  ┃  ┃     ┠╴@PanelContainer@44
       ┃  ┃  ┃     ┃  ┖╴@ScrollContainer@45
       ┃  ┃  ┃     ┃     ┠╴@Control@46
       ┃  ┃  ┃     ┃     ┠╴_h_scroll
       ┃  ┃  ┃     ┃     ┠╴_v_scroll
       ┃  ┃  ┃     ┃     ┖╴_focus
       ┃  ┃  ┃     ┠╴@Timer@47
       ┃  ┃  ┃     ┖╴@Timer@48
       ┃  ┃  ┠╴placementlabel
       ┃  ┃  ┠╴placementoptionbutton
       ┃  ┃  ┃  ┖╴@PopupMenu@55
       ┃  ┃  ┃     ┠╴@PanelContainer@50
       ┃  ┃  ┃     ┃  ┖╴@ScrollContainer@51
       ┃  ┃  ┃     ┃     ┠╴@Control@52
       ┃  ┃  ┃     ┃     ┠╴_h_scroll
       ┃  ┃  ┃     ┃     ┠╴_v_scroll
       ┃  ┃  ┃     ┃     ┖╴_focus
       ┃  ┃  ┃     ┠╴@Timer@53
       ┃  ┃  ┃     ┖╴@Timer@54
       ┃  ┃  ┠╴animatecheckbox
       ┃  ┃  ┠╴billboard
       ┃  ┃  ┠╴ignition
       ┃  ┃  ┖╴toggleterminalbutton
       ┃  ┠╴cameras
       ┃  ┃  ┠╴camera1
       ┃  ┃  ┖╴camera2
       ┃  ┖╴scenescanner
       ┖╴init

🧭 End of Snapshot

