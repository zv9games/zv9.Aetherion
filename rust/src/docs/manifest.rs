AetherionEngine is a mythic core—a modular, dimension-agnostic 
procedural generation engine coded in Rust as a GDExtension 
for Godot 4+(5, 6 ->).

Directory/File Structure:

lexicon: ├── └── │



c:/zv9/zv9.aetherion/rust/src/

├── aetherion/                             
│   ├── codegen/                           
│   │   ├── config.rs                      
│   │   ├── dsl.rs                         
│   │   ├── emitter.rs                     
│   │   ├── mod.rs                         
│   │   └── parser.rs                      
│   │
│   ├── core/                              
│   │   ├── conductor.rs                   
│   │   ├── dimension.rs                   
│   │   ├── lifecycle.rs                   
│   │   ├── mod.rs                         
│   │   └── runtime.rs                     
│   │
│   ├── generator/                         
│   │   ├── mod.rs                         
│   │   ├── noise.rs                       
│   │   ├── noise_config.rs                
│   │   ├── pattern_type.rs                
│   │   └── patterns.rs                    
│   │
│   ├── interaction/                       
│   │   ├── mod.rs                         
│   │   ├── modifiers.rs                   
│   │   └── tools.rs                       
│   │ 
│   ├── pipeline/                          
│   │   ├── builder/                       
│   │   │   ├── builder.rs                 
│   │   │   ├── mod.rs                     
│   │   │   ├── streamer.rs                
│   │   │   └── threaded.rs                
│   │   │
│   │   ├── data/                          
│   │   │   ├── chunk.rs                   
│   │   │   ├── data.rs                    
│   │   │   ├── grid.rs                    
│   │   │   ├── map_build_options.rs       
│   │   │   ├── mod.rs                     
│   │   │   ├── options.rs                 
│   │   │   ├── tile.rs                    
│   │   │   └── vector.rs                  
│   │   └── mod.rs                         
│   │
│   ├── structure/                         
│   │   ├── generation.rs                  
│   │   ├── mod.rs                         
│   │   └── placement.rs                   
│   └── mod.rs                             
│      
├── bin/                                   
│   ├── mod.rs                             
│   ├── aetherion_binary.rs                
│   ├── sync_audit.rs                      
│   └── sync_to_godot.rs                   
│   
├── docs/                                  
│   ├── manifest.rs                        
│   └── mod.rs                             
│
├── examples/                              
│   ├── infinity.rs                        
│   ├── mod.rs                             
│   ├── expansive.rs                       
│   └── racing.rs                          
│
├── godot4/                                
│   ├── api/                               
│   │   ├── config.rs                      
│   │   ├── engine.rs                      
│   │   ├── generator.rs                   
│   │   ├── map.rs                         
│   │   ├── mod.rs                         
│   │   ├── oracle.rs                      
│   │   └── signals.rs                     
│   │
│   ├── bindings/                          
│   │   ├── godot_types.rs                 
│   │   └── mod.rs                         
│   │
│   ├── interface/                         
│   │   ├── controls.rs                    
│   │   ├── diagnostics.rs                 
│   │   └── mod.rs                         
│   │
│   ├── messaging/                         
│   │   ├── messages.rs                    
│   │   ├── mod.rs                         
│   │   └── sync.rs                        
│   │
│   ├── signals/                           
│   │   ├── definitions.rs                 
│   │   ├── dispatch.rs                    
│   │   └── mod.rs                         
│   │
│   └── mod.rs                             
│
├── shared/                                
│   ├── grid_bounds.rs                     
│   ├── grid2d.rs                          
│   ├── math.rs                            
│   ├── mod.rs                             
│   ├── spatial.rs                         
│   ├── traits.rs                          
│   └── types.rs                           
│
├── tests/                                 
│   ├── core/                              
│   │   ├── conductor_tests.rs
│   │   ├── dimension_tests.rs
│   │   ├── lifecycle_tests.rs
│   │   ├── mod.rs
│   │   └── runtime_tests.rs
│   ├── godot4/                            
│   │   ├── signal_tests.rs
│   │   └── sync_tests.rs
│   ├── pipeline/                          
│   │   ├── chunk_tests.rs
│   │   ├── grid_tests.rs
│   │   └── map_build_tests.rs
│   ├── shared/                            
│   │   ├── math_tests.rs
│   │   ├── traits_tests.rs
│   │   └── types_tests.rs
│   ├── util/                              
│   │   ├── logging_tests.rs
│   │   ├── profiling_tests.rs
│   │   └── timer_tests.rs
│   │
│   ├── common.rs                          
│   ├── generation_tests.rs                
│   ├── godot_integration_tests.rs         
│   ├── mod.rs                             
│   ├── pipeline_tests.rs                  
│   ├── signal_tests.rs                    
│   └── trait_tests.rs                     
│
├── trailkeeper/                           
│   ├── entry.rs                           
│   ├── macros.rs                          
│   └── registry.rs                        
│
├── util/                                  
│   ├── config.rs                          
│   ├── logging.rs                         
│   ├── time.rs                            
│   ├── profiling.rs                       
│   ├── position.rs                        
│   ├── direction.rs                       
│   ├── timer.rs                           
│   ├── velocity.rs                        
│   └── mod.rs                             
│
├── api.rs                                 
├── lib.rs                                 
└── prelude.rs                             


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

crate Aetherion_Engine
├── struct AetherionEXT: pub(crate)
│   └── fn on_level_init: pub(crate)
├── mod aetherion: pub
│   ├── mod codegen: pub
│   │   ├── mod config: pub
│   │   │   └── struct Config: pub
│   │   ├── mod dsl: pub
│   │   ├── mod emitter: pub
│   │   │   └── fn generate_code: pub
│   │   └── mod parser: pub
│   ├── mod core: pub
│   │   ├── mod conductor: pub
│   │   │   ├── struct Conductor: pub
│   │   │   │   ├── fn enqueue: pub
│   │   │   │   ├── fn has_pending: pub
│   │   │   │   ├── fn new: pub
│   │   │   │   ├── fn queue_len: pub
│   │   │   │   └── fn tick: pub
│   │   │   └── enum ProcCommand: pub
│   │   ├── mod dimension: pub
│   │   │   └── enum Dimension: pub
│   │   │       ├── fn as_str: pub
│   │   │       ├── fn flipped: pub
│   │   │       ├── fn is_2d: pub
│   │   │       └── fn is_3d: pub
│   │   ├── mod lifecycle: pub
│   │   │   ├── struct Lifecycle: pub
│   │   │   │   ├── fn initialize: pub
│   │   │   │   ├── fn is_active: pub
│   │   │   │   ├── fn is_shutting_down: pub
│   │   │   │   ├── fn is_terminated: pub
│   │   │   │   ├── fn mark_running: pub
│   │   │   │   ├── fn new: pub
│   │   │   │   ├── fn shutdown: pub
│   │   │   │   └── fn terminate: pub
│   │   │   └── enum LifecycleState: pub
│   │   └── mod runtime: pub
│   │       └── struct RuntimeState: pub
│   │           ├── fn average_tick_duration: pub
│   │           ├── fn budget: pub
│   │           ├── fn has_tick_listener: pub
│   │           ├── fn is_budget_exceeded: pub
│   │           ├── fn new: pub
│   │           ├── fn set_frame_budget: pub
│   │           ├── fn set_tick_listener: pub
│   │           ├── fn tick: pub
│   │           ├── fn ticks: pub
│   │           └── fn time_since_last_tick: pub
│   ├── mod generator: pub
│   │   ├── mod noise: pub
│   │   │   ├── enum NoiseType: pub
│   │   │   │   ├── fn as_str: pub
│   │   │   │   └── fn is_available: pub
│   │   │   ├── fn basic_noise: pub
│   │   │   ├── fn cellular_automata: pub
│   │   │   ├── fn count_alive_neighbors: pub(self)
│   │   │   ├── fn generate_grid_noise: pub
│   │   │   └── fn generate_noise: pub
│   │   ├── mod noise_config: pub
│   │   │   ├── struct NoiseConfig: pub
│   │   │   └── fn generate_grid_from_config: pub
│   │   ├── mod pattern_type: pub
│   │   │   ├── enum PatternType: pub
│   │   │   └── fn blend_noise_and_pattern: pub
│   │   └── mod patterns: pub
│   │       ├── fn checkerboard: pub
│   │       ├── fn horizontal_stripes: pub
│   │       ├── fn radial: pub
│   │       ├── fn vertical_stripes: pub
│   │       └── fn xor_fractal: pub
│   ├── mod interaction: pub
│   │   ├── mod modifiers: pub
│   │   │   ├── struct PaintTile: pub
│   │   │   │   └── fn apply: pub(self)
│   │   │   ├── trait TileModifier: pub
│   │   │   └── struct ToggleTile: pub
│   │   │       └── fn apply: pub(self)
│   │   └── mod tools: pub
│   ├── mod pipeline: pub
│   │   ├── mod builder: pub
│   │   │   ├── mod builder: pub
│   │   │   │   └── fn spawn_map_builder: pub
│   │   │   ├── mod streamer: pub
│   │   │   │   └── struct ChunkStreamer: pub
│   │   │   │       ├── fn enqueue_chunk: pub
│   │   │   │       ├── fn has_pending: pub
│   │   │   │       ├── fn new: pub
│   │   │   │       ├── fn pause: pub
│   │   │   │       ├── fn queue_len: pub
│   │   │   │       ├── fn resume: pub
│   │   │   │       ├── fn sync: pub
│   │   │   │       ├── fn sync_mut: pub
│   │   │   │       └── fn try_deliver: pub
│   │   │   └── mod threaded: pub
│   │   │       └── fn spawn_builder_thread: pub
│   │   └── mod data: pub
│   │       ├── mod chunk: pub
│   │       │   └── struct MapDataChunk: pub
│   │       │       ├── fn get: pub
│   │       │       ├── fn insert: pub
│   │       │       ├── fn into_inner: pub
│   │       │       ├── fn is_empty: pub
│   │       │       ├── fn iter: pub
│   │       │       ├── fn len: pub
│   │       │       ├── fn merge: pub
│   │       │       └── fn new: pub
│   │       ├── mod data: pub
│   │       ├── mod grid: pub
│   │       │   ├── struct MapGrid: pub
│   │       │   │   ├── fn get_tile: pub
│   │       │   │   ├── fn is_within_bounds: pub
│   │       │   │   ├── fn new: pub
│   │       │   │   └── fn set_tile: pub
│   │       │   └── enum TileType: pub
│   │       ├── mod map_build_options: pub
│   │       │   ├── enum GodotNoiseType: pub
│   │       │   │   ├── fn as_str: pub
│   │       │   │   ├── fn from_str: pub
│   │       │   │   └── fn to_internal: pub
│   │       │   └── struct MapBuildOptions: pub
│   │       │       ├── fn init: pub(self)
│   │       │       ├── fn noise_type: pub
│   │       │       └── fn to_noise_config: pub
│   │       ├── mod options: pub
│   │       │   ├── enum GodotNoiseType: pub
│   │       │   │   └── fn to_internal: pub
│   │       │   └── struct MapBuildOptions: pub
│   │       │       ├── fn default: pub
│   │       │       ├── fn is_animated: pub
│   │       │       ├── fn noise_type: pub
│   │       │       ├── fn to_noise_config: pub
│   │       │       └── fn total_tiles: pub
│   │       ├── mod tile: pub
│   │       │   ├── struct TileInfo: pub
│   │       │   └── mod tile_flags: pub
│   │       └── mod vector: pub
│   │           └── struct SerializableVector2i: pub
│   └── mod structure: pub
│       └── mod generation: pub
│           ├── struct RecursiveMaze: pub
│           │   └── fn generate: pub(self)
│           ├── struct RoomGrid: pub
│           │   └── fn generate: pub(self)
│           └── trait StructureGenerator: pub
├── mod examples: pub
│   ├── mod infinity: pub
│   │   └── struct Infinity: pub
│   │       ├── fn bounds: pub
│   │       ├── fn expand: pub
│   │       ├── fn new: pub
│   │       └── fn tick: pub
│   ├── mod pacman_expansive: pub
│   │   └── struct PacmanExpansive: pub
│   │       ├── fn expand: pub
│   │       ├── fn new: pub
│   │       ├── fn run: pub
│   │       └── fn try_turn: pub
│   └── mod racing: pub
│       ├── struct Racer: pub
│       │   ├── fn boost: pub
│       │   ├── fn new: pub
│       │   └── fn update: pub
│       └── struct Track: pub
│           └── fn is_lap_complete: pub
├── mod godot4: pub
│   ├── mod api: pub
│   │   ├── mod config: pub
│   │   │   └── struct AetherionConfig: pub
│   │   │       ├── fn _ready: pub(self)
│   │   │       ├── fn get_chunk_area: pub(self)
│   │   │       ├── fn init: pub(self)
│   │   │       └── fn regenerate_seed: pub(self)
│   │   ├── mod engine: pub
│   │   │   └── struct AetherionEngine: pub
│   │   │       ├── fn aetherionoracle: pub
│   │   │       ├── fn apply_chunks_to_tilemap: pub(self)
│   │   │       ├── fn build_map: pub
│   │   │       ├── fn debug_place_tile: pub
│   │   │       ├── fn emit_pending_signals: pub(self)
│   │   │       ├── fn get_status: pub
│   │   │       ├── fn init: pub(self)
│   │   │       ├── fn ping: pub
│   │   │       ├── fn process: pub(self)
│   │   │       ├── fn ready: pub(self)
│   │   │       └── fn set_tilemap: pub
│   │   ├── mod generator: pub
│   │   │   ├── struct AetherionGenerator: pub
│   │   │   │   ├── fn _ready: pub(self)
│   │   │   │   ├── fn generate_noise: pub(self)
│   │   │   │   ├── fn generate_pattern: pub(self)
│   │   │   │   ├── fn init: pub(self)
│   │   │   │   └── fn tile_to_dict: pub(self)
│   │   │   ├── fn generate_noise_tile: pub
│   │   │   └── fn generate_pattern_tile: pub
│   │   ├── mod map: pub
│   │   │   └── struct AetherionMap: pub
│   │   │       ├── fn _ready: pub(self)
│   │   │       ├── fn clear_chunk: pub(self)
│   │   │       ├── fn get_tile: pub(self)
│   │   │       ├── fn init: pub(self)
│   │   │       └── fn load_chunk: pub(self)
│   │   ├── mod oracle: pub
│   │   │   └── struct AetherionOracle: pub
│   │   │       ├── fn init: pub(self)
│   │   │       ├── fn ping: pub
│   │   │       ├── fn ready: pub(self)
│   │   │       ├── fn set_engine: pub
│   │   │       └── fn tick: pub
│   │   └── mod signals: pub
│   │       └── struct AetherionSignals: pub
│   │           ├── fn build_map_start: pub(self)
│   │           ├── fn engine_init_start: pub(self)
│   │           ├── fn engine_initialized: pub(self)
│   │           ├── fn frame_budget_exceeded: pub(self)
│   │           ├── fn generation_complete: pub(self)
│   │           ├── fn generation_progress: pub(self)
│   │           ├── fn map_build_cancelled: pub(self)
│   │           ├── fn map_build_duration: pub(self)
│   │           ├── fn map_build_failed: pub(self)
│   │           ├── fn map_building_status: pub(self)
│   │           ├── fn map_chunk_ready: pub(self)
│   │           ├── fn pipeline_complete: pub(self)
│   │           ├── fn pipeline_start: pub(self)
│   │           ├── fn rust_error: pub(self)
│   │           ├── fn rust_extension_ready: pub(self)
│   │           ├── fn sync_required: pub(self)
│   │           ├── fn tick_completed: pub(self)
│   │           ├── fn tick_started: pub(self)
│   │           ├── fn tilemap_sync_complete: pub(self)
│   │           └── fn tilemap_updated: pub(self)
│   ├── mod bindings: pub
│   │   └── mod godot_types: pub
│   ├── mod messaging: pub
│   │   ├── mod messages: pub
│   │   │   └── enum EngineMessage: pub
│   │   └── mod sync: pub
│   │       ├── struct GodotSync: pub
│   │       │   ├── fn add_chunk: pub
│   │       │   ├── fn add_signal: pub
│   │       │   ├── fn drain_chunks: pub
│   │       │   ├── fn drain_signals: pub
│   │       │   ├── fn has_pending: pub
│   │       │   └── fn init: pub
│   │       └── struct GodotSyncInner: pub(self)
│   └── mod signals: pub
│       ├── mod definitions: pub
│       └── mod dispatch: pub
│           └── fn emit_from_message: pub
├── mod shared: pub
│   ├── mod grid2d: pub
│   │   └── struct Grid2D: pub
│   │       ├── fn filled: pub
│   │       ├── fn get: pub
│   │       └── fn set: pub
│   ├── mod grid_bounds: pub
│   │   └── struct GridBounds: pub
│   │       └── fn contains: pub
│   ├── mod math: pub
│   │   └── fn clamp: pub
│   ├── mod spatial: pub
│   │   ├── fn all_neighbors: pub
│   │   ├── fn cardinal_neighbors: pub
│   │   └── fn in_bounds: pub
│   ├── mod traits: pub
│   │   ├── trait Serializable: pub
│   │   └── trait Tickable: pub
│   └── mod types: pub
│       ├── type Coord: pub
│       ├── struct EntityId: pub
│       │   ├── fn from_raw: pub
│       │   └── fn value: pub
│       └── type Timestamp: pub
├── mod tests: pub
│   ├── mod common: pub
│   │   ├── struct SignalReceiver: pub
│   │   │   ├── fn on_progress: pub(self)
│   │   │   └── fn on_status: pub(self)
│   │   ├── fn dummy_results: pub
│   │   └── fn gstr: pub
│   ├── mod core: pub
│   │   ├── mod conductor_tests: pub
│   │   ├── mod dimension_tests: pub
│   │   ├── mod lifecycle_tests: pub
│   │   └── mod runtime_tests: pub
│   ├── mod generation_tests: pub
│   ├── mod godot_integration_tests: pub
│   ├── mod pipeline_tests: pub
│   ├── mod signal_tests: pub
│   └── mod trait_tests: pub
├── mod trailkeeper: pub
│   ├── mod collector: pub
│   │   ├── struct LOG_REGISTRY: pub(self)
│   │   │   └── fn initialize: pub(self)
│   │   └── struct Trailkeeper: pub
│   │       ├── fn all: pub
│   │       ├── fn query: pub
│   │       └── fn record: pub
│   ├── mod config: pub
│   │   └── fn check_config_change: pub
│   ├── mod entry: pub
│   │   ├── enum EventType: pub
│   │   ├── struct LogEntry: pub
│   │   └── enum LogStatus: pub
│   ├── mod export: pub
│   │   ├── struct SerializableLogEntry: pub(self)
│   │   └── fn export_json: pub
│   ├── mod macros: pub
│   ├── mod registry: pub
│   ├── mod scan: pub
│   │   └── fn scan_git_diff: pub
│   └── mod watch: pub
│       └── fn start_file_watch: pub
└── mod util: pub
    ├── mod config: pub
    │   └── struct EngineConfig: pub
    │       ├── fn is_multithreaded: pub
    │       └── fn tick_interval: pub
    ├── mod direction: pub
    │   └── struct Direction: pub
    │       ├── fn all: pub
    │       ├── fn left: pub
    │       ├── fn reverse: pub
    │       └── fn right: pub
    ├── mod logging: pub
    │   ├── fn init_logging: pub
    │   ├── fn log_debug: pub
    │   ├── fn log_error: pub
    │   ├── fn log_info: pub
    │   └── fn log_warn: pub
    ├── mod position: pub
    │   └── struct Position: pub
    │       ├── fn distance_to: pub
    │       ├── fn max: pub
    │       ├── fn min: pub
    │       ├── fn new: pub
    │       └── fn step: pub
    ├── mod time: pub
    │   └── struct TickTimer: pub
    │       ├── fn new: pub
    │       ├── fn should_tick: pub
    │       ├── fn tick_interval: pub
    │       └── fn time_since_last_tick: pub
    ├── mod timer: pub
    │   └── struct Timer: pub
    │       ├── fn elapsed: pub
    │       ├── fn new: pub
    │       └── fn reset: pub
    └── mod velocity: pub
        └── struct Velocity: pub
            ├── fn apply: pub
            ├── fn increase: pub
            ├── fn new: pub
            ├── fn scale: pub
            └── fn zero: pub
PS C:\zv9\zv9.aetherion\rust>