AetherionEngine is a mythic core—a modular, dimension-agnostic 
procedural generation engine coded in Rust as a GDExtension 
for Godot 4+(5, 6 ->).

Directory/File Structure:

lexicon: ├── └── │

c:/zv9/zv9.aetherionengine/
├──.git/(hidden)
├──aetherion_engine_tester/
├──cargo-aetherion/
├──rust/
├──/.gitignore.txt
├──/LICENSE.md
├──/README.md

c:/zv9/zv9.aetherion/.git/
├──not sure if this directory needs to be discovered.

c:/zv9/zv9.aetherion/aetherion_engine_tester/
├──.godot/
├──addons/
│	├──	aetherion_plugin
│	│	├──plugin.cfg
│	│	├──aetherion_engine.gd.uid
│	│	├──aetherion_engine.gd
│	│	├──Aetherion.gd
│	│	└──Aetherion.gd.uid
├──root_scenes
│	└──main.tscn
├──root_scripts
│	└──main.gd
├──.editorconfig
├──.gitattributes
├──.gitignore
├──aetherion.gdextension 
├──aetherion.gdextension.uid
├──Aetherion_Engine.dll
├──project.godot
lexicon: ├── └── │
c:/zv9/zv9.aetherion/cargo-aetherion/
├──src/
│	└──main.gd
├──target/
Cargo.toml
c:/zv9/zv9.aetherion/rust/
├──.cargo
├──src/
├──target/
├──build 
├──cargo.lock
├──cargo.toml
GDExtension.toml 

c:/zv9/zv9.aetherion/rust/src/
├── core/                                   # 🧠 Core Procedural Logic and Lifecyle Orchestration (formerly 'engine/')
│   ├── animator.rs                       # Manages animation of tiles/voxels
│   ├── dimension.rs                      # Abstracts 2D/3D coordinate systems
│   ├── generator.rs                      # Core procedural generation algorithms
│   ├── lifecycle.rs                      # Manages engine startup, tick, and shutdown
│   ├── map_builder.rs                    # Builds the map grid from processed data
│   ├── tile_placer.rs                    # Heavy lifter: places tiles/voxels based on map data (formerly tile_smasher)
│   ├── registry.rs                       # Central registry for entities
│   ├── runtime.rs                        # Engine runtime and state container
│   ├── types.rs                          # Shared data structures and traits
│   └── mod.rs                            # Core module manifest and re-exports
│
├── data_processing/                        # 🖼️ Utilities for handling external data and assets
│   ├── level_importer.rs                 # Reads external level data and config (formerly annotation_loader)
│   ├── tilemap_data_generator.rs         # Processes images to create tile placement data (formerly image_processor)
│   └── mod.rs
│
├── godot_bridge/                           # 🎮 Godot-facing bindings and communication layer (formerly 'interface/')
│   ├── bindings.rs                       # Native bindings for Godot classes
│   ├── commands.rs                       # Defines the Godot-facing public command API (formerly api_bot, echo_api)
│   ├── signals.rs                        # Handles emitting signals to Godot
│   └── mod.rs                            # GDExtension entrypoint and module manifest
│
├── logging/                                # 📋 Debugging, logging, and engine introspection (formerly 'audit/')
│   ├── debugger.rs                       # Debug tools for engine state
│   ├── logger.rs                         # Structured logging and audit trails
│   └── mod.rs
│
├── utilities/                              # 🛠️ Async ops, config, and helper utilities (formerly 'utils/')
│   ├── config.rs                         # Runtime configuration and presets
│   ├── concurrency.rs                    # Threading primitives and async coordination (consolidates all threading logic)
│   ├── helpers.rs                        # General helper functions and wrappers
│   ├── mapper.rs                         # Spatial mapping and coordinate transforms
│   ├── time.rs                           # Tick management and time-based transitions
│   └── mod.rs
│
├── game_logic/                             # 🕹️ Game-specific logic
│   └── scene_transitions.rs              # Handles scene loading and fading (formerly changeover)
│
├── prelude.rs                              # Re-exports for ergonomic imports
├── lib.rs                                  # Root crate manifest

AetherionEngine Changelog
v0.0.1 - 2025-08-22

Refactor: Foundational Codebase Restructure for Clarity and 
Maintainability. This entry marks the first major refactor 
of the AetherionEngine codebase. The primary goal of this 
release is to simplify the project's internal structure and 
remove abstract, metaphorical naming conventions that created 
a significant cognitive burden. The engine's core functionality 
remains the same, but the organization has been overhauled to be more intuitive, logical, and maintainable.

Key Changes:

Codebase Restructure: The entire rust/src directory has been reorganized into a clear, logical module structure (core, godot_bridge, data_processing, etc.).

Renamed Files: All files with metaphorical names (e.g., api_bot, tile_smasher, changeover) have been renamed to reflect their technical purpose directly.

Consolidated Logic: Related functionalities have been grouped and merged into single files or modules, specifically:

All Godot communication logic has been unified within the new godot_bridge module.

Threading primitives and async operations from various locations have been consolidated into utilities/concurrency.rs.

All external data processing logic is now housed in the data_processing module.

Deprecated Modules: The confusing and abstract pre_echo/ directory has been completely removed. Its functional components have been moved and integrated into the new structure.

Improved Clarity: The new file and module names are self-documenting, making the codebase significantly easier to navigate and understand for contributors.
