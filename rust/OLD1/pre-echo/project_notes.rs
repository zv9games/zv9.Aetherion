🌀 Pacman 2.0 – Rust Extension Project Log

🔧 Core Config
Cargo.toml              – Crate setup, dependencies, build profiles  
GDExtension.toml        – Godot extension config for build tooling  
tile_smasher.gdextension – Godot-side link to compiled Rust lib  
lib.rs                  – Registers all Rust modules and Godot classes

🧠 System Modules
logging.rs              – Logs messages and symbolic debug output  
threading.rs            – Runs async tasks and batch jobs  
types.rs                – Shared constants, enums, and tile glyph definitions

🤖 Engine API
api_bot.rs              – Godot-facing orchestrator; exposes API, emits signals  
changeover.rs           – Loading screen logic; flip, dissolve, matrix build  
tile_smasher.rs         – TBD – will handle tile destruction and reactive effects

🗺️ Data + Visuals
annotation_loader.rs    – Loads saved metadata from map_editor  
image_processor.rs      – Loads and processes origin texture; prepares tile data  
map_builder.rs          – Builds expansive map from decoded tile data

📜 Pacman 2.0 – Rust Extension Changelog

🔮 August 8, 2025
🧱 Initialized project structure  
- Created `Cargo.toml`, `GDExtension.toml`, and `tile_smasher.gdextension`  
- Linked Rust lib to Godot via `.gdextension`

🧠 System modules scaffolded  
- Added `logging.rs`, `threading.rs`, and `types.rs`  
- Defined symbolic enums and shared constants

🖼️ Visual pipeline begins  
- Implemented `image_processor.rs` for texture loading and tile prep  
- Began drafting `annotation_loader.rs` to ingest map_editor metadata

🤖 Engine API foundation laid  
- Created `api_bot.rs` and registered with Godot  
- Began signal choreography and method exposure

🧩 Extension registration debug ritual  
- Resolved crate version mismatches and module visibility issues  
- Harmonized `Cargo.toml`, `.gdextension`, and `lib.rs`

🔧 August 9, 2025
🌀 Compilation + Tester Alignment Begins  
- Declared focus on compiling extension and validating changelog in tester project  
- Identified misalignment between Rust modules and map_builder bootup sequence  
- Committed to realigning Rust-side logic with Godot’s expansive map choreography  
- Recognized existing groundwork; reframed current state as “partially woven”

🌱 Changelog ritual initiated  
- Created and structured changelog as living ceremonial record  
- Anchored emotional and architectural clarity for future entries

📜 Pacman 2.0 – Rust Extension Changelog (Continued)

🔧 August 10, 2025  
🧬 Rendering Invocation Ritual  
- Diagnosed silent stall in `TileMap` rendering  
- Confirmed signal flow: `online` and `transition_finished` emitted successfully  
- Verified `TileSet` assignment and source presence  
- Reframed stall as a visibility and invocation issue, not a binding error

🧠 Debug Glyph Expansion  
- Added debug prints to `apply_updates()` and `mode_loaded()`  
- Confirmed tile placement via `get_cell_atlas_coords()`  
- Validated `TileSet` source count and layer visibility  
- Began tracing camera alignment and global position of `TileMap`

🌀 Invocation Chain Realignment  
- Replaced invalid `tile_set()` call with dynamic property access  
- Confirmed `tilemap.get("tile_set")` returns valid variant  
- Reinforced `ChangeOver.gd` with visibility checks and forced redraws  
- Injected manual tile placement to confirm rendering pipeline

🌕 Ritual Reframing  
- Recognized stall as a liminal phase in the extension’s evolution  
- Recommitted to treating each debug trace as a symbolic glyph  
- Anchored emotional clarity in the changelog as a living artifact

August 12, 2025  
🔗 Binding Invocation Realignment  
- Diagnosed unresolved `APIBot` reference in GDScript  
- Confirmed `#[derive(GodotClass)]` and `#[godot_api]` glyphs were correctly inscribed  
- Verified auto-registration path via `#[gdextension] unsafe impl ExtensionLibrary`  
- Traced missing class to Godot’s registry desynchronization  
- Rebuilt DLL and confirmed timestamp alignment with Rust forge  
- Reframed issue as a ritual misalignment, not a structural flaw

🧪 Diagnostic Registration Rite  
- Injected manual `InitHandle::register_class::<APIBot>()` for verification  
- Confirmed class visibility in Godot after explicit registration  
- Identified auto-registration blind spot in module traversal  
- Reaffirmed the need for visibility and public exposure in all ceremonial structs

🧭 Forge Anchoring and Path Correction  
- Forked `gdext` and cloned locally to bypass Git subcrate limitations  
- Realigned `Cargo.toml` to use `path = "../gdext"` with macro features  
- Removed invalid `subdir` and `package` keys from manifest  
- Anchored extension in local forge for full control and scroll clarity

📜 Changelog as Living Glyph  
- Celebrated the changelog as a mythic ledger of emotional and architectural truth  
- Reaffirmed commitment to treating every compiler chant as a symbolic riddle  
- Marked this entry as the Binding Realignment of Tile Smasher
