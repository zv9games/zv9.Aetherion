AetherionEngine is a mythic core—a modular, dimension-agnostic 
procedural generation engine coded in Rust as a GDExtension 
for Godot 4+(5, 6 ->).

Directory/File Structure:

lexicon: ├── └── │

c:/zv9/zv9.aetherion/rust/src/

src/
│   ├── zv9__aetherion__codegen__config.rs
│   ├── zv9__aetherion__codegen__dsl.rs
│   ├── zv9__aetherion__codegen__emitter.rs
│   ├── zv9__aetherion__codegen__parser.rs
│   ├── zv9__aetherion__core__conductor.rs
│   ├── zv9__aetherion__core__dimension.rs
│   ├── zv9__aetherion__core__lifecycle.rs
│   ├── zv9__aetherion__core__runtime.rs
│   ├── zv9__aetherion__generator__noise.rs
│   ├── zv9__aetherion__generator__noise_config.rs
│   ├── zv9__aetherion__generator__pattern_type.rs
│   ├── zv9__aetherion__generator__patterns.rs
│   ├── zv9__aetherion__interaction__modifiers.rs
│   ├── zv9__aetherion__interaction__tools.rs
│   ├── zv9__aetherion__pipeline_builder__builder.rs
│   ├── zv9__aetherion__pipeline_builder__streamer.rs
│   ├── zv9__aetherion__pipeline_builder__threaded.rs
│   ├── zv9__aetherion__pipeline_data__chunk.rs
│   ├── zv9__aetherion__pipeline_data__data.rs
│   ├── zv9__aetherion__pipeline_data__grid.rs
│   ├── zv9__aetherion__pipeline_data__map_build_options.rs
│   ├── zv9__aetherion__pipeline_data__options.rs
│   ├── zv9__aetherion__pipeline_data__tile.rs
│   ├── zv9__aetherion__pipeline_data__vector.rs
│   ├── zv9__aetherion__structure__generation.rs
│   ├── zv9__aetherion__structure__placement.rs
│   ├── zv9__godot_interface__api__config.rs
│   ├── zv9__godot_interface__api__engine.rs
│   ├── zv9__godot_interface__api__generator.rs
│   ├── zv9__godot_interface__api__map.rs
│   ├── zv9__godot_interface__api__oracle.rs
│   ├── zv9__godot_interface__api__signals.rs
│   ├── zv9__godot_interface__bindings__godot_types.rs
│   ├── zv9__godot_interface__interface__controls.rs
│   ├── zv9__godot_interface__interface__diagnostics.rs
│   ├── zv9__godot_interface__messaging__messages.rs
│   ├── zv9__godot_interface__messaging__sync.rs
│   ├── zv9__godot_interface__signals__definitions.rs
│   ├── zv9__godot_interface__signals__dispatch.rs
│   ├── zv9__shared__grid_bounds.rs
│   ├── zv9__shared__grid2d.rs
│   ├── zv9__shared__math.rs
│   ├── zv9__shared__spatial.rs
│   ├── zv9__shared__traits.rs
│   ├── zv9__shared__types.rs
│   ├── zv9__trailkeeper__entry.rs
│   ├── zv9__trailkeeper__macros.rs
│   ├── zv9__trailkeeper__registry.rs
│   ├── zv9__util__config.rs
│   ├── zv9__util__logging.rs
│   ├── zv9__util__time.rs
│   ├── zv9__util__profiling.rs
│   ├── zv9__util__position.rs
│   ├── zv9__util__direction.rs
│   ├── zv9__util__timer.rs
│   ├── zv9__util__velocity.rs
│	├── zv9__api.rs
│	├── zv9__lib.rs
│	└── zv9__prelude.rs
│
├── bin/
│   ├── zv9__bin__aetherion_binary.rs
│   ├── zv9__bin__sync_audit.rs
│   └── zv9__bin__sync_to_godot.rs
│
├── docs/
│   └── zv9__docs__manifest.rs

   


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
warning: unused import: `crate::zv9__aetherion__structure__placement::*`
  --> src\zv9__prelude.rs:47:9
   |
47 | pub use crate::zv9__aetherion__structure__placement::*;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `crate::zv9__trailkeeper__registry::*`
  --> src\zv9__prelude.rs:81:9
   |
81 | pub use crate::zv9__trailkeeper__registry::*;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__trailkeeper__macros::*`
  --> src\zv9__prelude.rs:82:9
   |
82 | pub use crate::zv9__trailkeeper__macros::*;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__core__dimension.rs:34:5
   |
34 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__core__lifecycle.rs:41:5
   |
41 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__codegen__config.rs:10:5
   |
10 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashMap`
 --> src\zv9__aetherion__codegen__dsl.rs:8:5
  |
8 | use std::collections::HashMap;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__generator__noise.rs:42:5
   |
42 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__generator__patterns.rs:41:5
   |
41 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__generator__pattern_type.rs:31:5
   |
31 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `godot::prelude::*`
  --> src\zv9__aetherion__pipeline_data__vector.rs:39:5
   |
39 | use godot::prelude::*;
   |     ^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__aetherion__structure__placement.rs:36:5
   |
36 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__godot_interface__signals__definitions.rs:44:5
   |
44 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__shared__grid2d.rs:36:5
   |
36 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__shared__math.rs:44:5
   |
44 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__shared__traits.rs:43:5
   |
43 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__shared__types.rs:41:5
   |
41 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__util__config.rs:43:5
   |
43 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__util__direction.rs:36:5
   |
36 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__util__time.rs:42:5
   |
42 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__util__velocity.rs:35:5
   |
35 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__trailkeeper__entry.rs:37:5
   |
37 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__trailkeeper__macros.rs:35:5
   |
35 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude::*`
  --> src\zv9__trailkeeper__registry.rs:34:5
   |
34 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^^^^

error[E0592]: duplicate definitions with name `set_terrain_mode`
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^ duplicate definitions for `set_terrain_mode`
...
104 |     fn set_terrain_mode(&mut self, mode: String) {
    |     -------------------------------------------- other definition for `set_terrain_mode`
    |
    = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0592]: duplicate definitions with name `set_structure_mode`
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^ duplicate definitions for `set_structure_mode`
...
110 |     fn set_structure_mode(&mut self, mode: String) {
    |     ---------------------------------------------- other definition for `set_structure_mode`
    |
    = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0592]: duplicate definitions with name `set_terrain_mode`
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^ duplicate definitions for `set_terrain_mode`
...
70 | #[godot_api]
   | ------------ other definition for `set_terrain_mode`
   |
   = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0592]: duplicate definitions with name `set_structure_mode`
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^ duplicate definitions for `set_structure_mode`
...
70 | #[godot_api]
   | ------------ other definition for `set_structure_mode`
   |
   = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no variant or associated item named `from_str` found for enum `GodotNoiseType` in the current scope
  --> src\zv9__aetherion__pipeline_builder__threaded.rs:57:25
   |
57 | ... GodotNoiseType::from_str(&options.mode.to_string()...
   |                     ^^^^^^^^ variant or associated item not found in `GodotNoiseType`
   |
  ::: src\zv9__aetherion__structure__generation.rs:53:1
   |
53 | pub enum GodotNoiseType {
   | ----------------------- variant or associated item `from_str` not found for this enum
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `from_str`, perhaps you need to implement it:
           candidate #1: `FromStr`
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8357335806657853199.txt'
   = note: consider using `--verbose` to print the full type name to the console
help: there is an associated function `from` with a similar name
   |
57 -         GodotNoiseType::from_str(&options.mode.to_string()).to_internal(),
57 +         GodotNoiseType::from(&options.mode.to_string()).to_internal(),
   |

error[E0599]: `GodotNoiseType` doesn't implement `std::fmt::Display`
    --> src\zv9__aetherion__pipeline_builder__threaded.rs:57:48
     |
  57 | ...(&options.mode.to_string()).to_internal(),
     |                   ^^^^^^^^^ method cannot be called on `GodotNoiseType` due to unsatisfied trait bounds
     |
    ::: src\zv9__aetherion__structure__generation.rs:53:1
     |
  53 | pub enum GodotNoiseType {
     | ----------------------- method `to_string` not found for this enum because it doesn't satisfy `_: Display` or `_: ToString`
     |
     = note: the following trait bounds were not satisfied:
             `zv9__aetherion__structure__generation::GodotNoiseType: std::fmt::Display`
             which is required by `zv9__aetherion__structure__generation::GodotNoiseType: ToString`
note: the trait `std::fmt::Display` must be implemented
    --> C:\Users\grego\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\fmt\mod.rs:1006:1
     |
1006 | pub trait Display: PointeeSized {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     = help: items from traits can only be used if the trait is implemented and in scope
     = note: the following traits define an item `to_string`, perhaps you need to implement one of them:
             candidate #1: `ScriptInstance`
             candidate #2: `ToString`
             candidate #3: `godot::classes::IAStar2D`
             candidate #4: `godot::classes::IAStar3D`
             candidate #5: `godot::classes::IAStarGrid2D`
             candidate #6: `godot::classes::IAcceptDialog`
             candidate #7: `godot::classes::IAesContext`
             candidate #8: `godot::classes::IAnimatableBody2D`
             candidate #9: `godot::classes::IAnimatableBody3D`
             candidate #10: `godot::classes::IAnimatedSprite2D`
             candidate #11: `godot::classes::IAnimatedSprite3D`
             candidate #12: `godot::classes::IAnimatedTexture`
             candidate #13: `godot::classes::IAnimation`
             candidate #14: `godot::classes::IAnimationLibrary`
             candidate #15: `godot::classes::IAnimationNode`
             candidate #16: `godot::classes::IAnimationNodeAdd2`
             candidate #17: `godot::classes::IAnimationNodeAdd3`
             candidate #18: `godot::classes::IAnimationNodeAnimation`
             candidate #19: `godot::classes::IAnimationNodeBlend2`
             candidate #20: `godot::classes::IAnimationNodeBlend3`
             candidate #21: `godot::classes::IAnimationNodeBlendSpace1D`
             candidate #22: `godot::classes::IAnimationNodeBlendSpace2D`
             candidate #23: `godot::classes::IAnimationNodeBlendTree`
             candidate #24: `godot::classes::IAnimationNodeExtension`
             candidate #25: `godot::classes::IAnimationNodeOneShot`
             candidate #26: `godot::classes::IAnimationNodeOutput`
             candidate #27: `godot::classes::IAnimationNodeStateMachine`
             candidate #28: `godot::classes::IAnimationNodeStateMachinePlayback`
             candidate #29: `godot::classes::IAnimationNodeStateMachineTransition`
             candidate #30: `godot::classes::IAnimationNodeSub2`
             candidate #31: `godot::classes::IAnimationNodeSync`
             candidate #32: `godot::classes::IAnimationNodeTimeScale`
             candidate #33: `godot::classes::IAnimationNodeTimeSeek`
             candidate #34: `godot::classes::IAnimationNodeTransition`
             candidate #35: `godot::classes::IAnimationPlayer`
             candidate #36: `godot::classes::IAnimationRootNode`
             candidate #37: `godot::classes::IAnimationTree`
             candidate #38: `godot::classes::IArea2D`
             candidate #39: `godot::classes::IArea3D`
             candidate #40: `godot::classes::IArrayMesh`
             candidate #41: `godot::classes::IArrayOccluder3D`
             candidate #42: `godot::classes::IAspectRatioContainer`
             candidate #43: `godot::classes::IAtlasTexture`
             candidate #44: `godot::classes::IAudioBusLayout`
             candidate #45: `godot::classes::IAudioEffect`
             candidate #46: `godot::classes::IAudioEffectAmplify`
             candidate #47: `godot::classes::IAudioEffectBandLimitFilter`
             candidate #48: `godot::classes::IAudioEffectBandPassFilter`
             candidate #49: `godot::classes::IAudioEffectCapture`
             candidate #50: `godot::classes::IAudioEffectChorus`
             candidate #51: `godot::classes::IAudioEffectCompressor`
             candidate #52: `godot::classes::IAudioEffectDelay`
             candidate #53: `godot::classes::IAudioEffectDistortion`
             candidate #54: `godot::classes::IAudioEffectEq`
             candidate #55: `godot::classes::IAudioEffectEq10`
             candidate #56: `godot::classes::IAudioEffectEq21`
             candidate #57: `godot::classes::IAudioEffectEq6`
             candidate #58: `godot::classes::IAudioEffectFilter`
             candidate #59: `godot::classes::IAudioEffectHardLimiter`
             candidate #60: `godot::classes::IAudioEffectHighPassFilter`
             candidate #61: `godot::classes::IAudioEffectHighShelfFilter`
             candidate #62: `godot::classes::IAudioEffectInstance`
             candidate #63: `godot::classes::IAudioEffectLimiter`
             candidate #64: `godot::classes::IAudioEffectLowPassFilter`
             candidate #65: `godot::classes::IAudioEffectLowShelfFilter`
             candidate #66: `godot::classes::IAudioEffectNotchFilter`
             candidate #67: `godot::classes::IAudioEffectPanner`
             candidate #68: `godot::classes::IAudioEffectPhaser`
             candidate #69: `godot::classes::IAudioEffectPitchShift`
             candidate #70: `godot::classes::IAudioEffectRecord`
             candidate #71: `godot::classes::IAudioEffectReverb`
             candidate #72: `godot::classes::IAudioEffectSpectrumAnalyzer`
             candidate #73: `godot::classes::IAudioEffectStereoEnhance`
             candidate #74: `godot::classes::IAudioListener2D`
             candidate #75: `godot::classes::IAudioListener3D`
             candidate #76: `godot::classes::IAudioStream`
             candidate #77: `godot::classes::IAudioStreamGenerator`
             candidate #78: `godot::classes::IAudioStreamInteractive`
             candidate #79: `godot::classes::IAudioStreamMicrophone`
             candidate #80: `godot::classes::IAudioStreamMp3`
             candidate #81: `godot::classes::IAudioStreamOggVorbis`
             candidate #82: `godot::classes::IAudioStreamPlayback`
             candidate #83: `godot::classes::IAudioStreamPlaybackOggVorbis`
             candidate #84: `godot::classes::IAudioStreamPlaybackResampled`
             candidate #85: `godot::classes::IAudioStreamPlayer`
             candidate #86: `godot::classes::IAudioStreamPlayer2D`
             candidate #87: `godot::classes::IAudioStreamPlayer3D`
             candidate #88: `godot::classes::IAudioStreamPlaylist`
             candidate #89: `godot::classes::IAudioStreamPolyphonic`
             candidate #90: `godot::classes::IAudioStreamRandomizer`
             candidate #91: `godot::classes::IAudioStreamSynchronized`
             candidate #92: `godot::classes::IAudioStreamWav`
             candidate #93: `godot::classes::IBackBufferCopy`
             candidate #94: `godot::classes::IBaseButton`
             candidate #95: `godot::classes::IBitMap`
             candidate #96: `godot::classes::IBone2D`
             candidate #97: `godot::classes::IBoneAttachment3D`
             candidate #98: `godot::classes::IBoneMap`
             candidate #99: `godot::classes::IBoxContainer`
             candidate #100: `godot::classes::IBoxMesh`
             candidate #101: `godot::classes::IBoxOccluder3D`
             candidate #102: `godot::classes::IBoxShape3D`
             candidate #103: `godot::classes::IButton`
             candidate #104: `godot::classes::IButtonGroup`
             candidate #105: `godot::classes::ICallbackTweener`
             candidate #106: `godot::classes::ICamera2D`
             candidate #107: `godot::classes::ICamera3D`
             candidate #108: `godot::classes::ICameraAttributes`
             candidate #109: `godot::classes::ICameraAttributesPhysical`
             candidate #110: `godot::classes::ICameraAttributesPractical`
             candidate #111: `godot::classes::ICameraFeed`
             candidate #112: `godot::classes::ICameraTexture`
             candidate #113: `godot::classes::ICanvasGroup`
             candidate #114: `godot::classes::ICanvasItemMaterial`
             candidate #115: `godot::classes::ICanvasLayer`
             candidate #116: `godot::classes::ICanvasModulate`
             candidate #117: `godot::classes::ICanvasTexture`
             candidate #118: `godot::classes::ICapsuleMesh`
             candidate #119: `godot::classes::ICapsuleShape2D`
             candidate #120: `godot::classes::ICapsuleShape3D`
             candidate #121: `godot::classes::ICenterContainer`
             candidate #122: `godot::classes::ICharFxTransform`
             candidate #123: `godot::classes::ICharacterBody2D`
             candidate #124: `godot::classes::ICharacterBody3D`
             candidate #125: `godot::classes::ICheckBox`
             candidate #126: `godot::classes::ICheckButton`
             candidate #127: `godot::classes::ICircleShape2D`
             candidate #128: `godot::classes::ICodeEdit`
             candidate #129: `godot::classes::ICodeHighlighter`
             candidate #130: `godot::classes::ICollisionPolygon2D`
             candidate #131: `godot::classes::ICollisionPolygon3D`
             candidate #132: `godot::classes::ICollisionShape2D`
             candidate #133: `godot::classes::ICollisionShape3D`
             candidate #134: `godot::classes::IColorPalette`
             candidate #135: `godot::classes::IColorPicker`
             candidate #136: `godot::classes::IColorPickerButton`
             candidate #137: `godot::classes::IColorRect`
             candidate #138: `godot::classes::ICompressedCubemap`
             candidate #139: `godot::classes::ICompressedCubemapArray`
             candidate #140: `godot::classes::ICompressedTexture2D`
             candidate #141: `godot::classes::ICompressedTexture2DArray`
             candidate #142: `godot::classes::ICompressedTexture3D`
             candidate #143: `godot::classes::IConcavePolygonShape2D`
             candidate #144: `godot::classes::IConcavePolygonShape3D`
             candidate #145: `godot::classes::IConeTwistJoint3D`
             candidate #146: `godot::classes::IConfigFile`
             candidate #147: `godot::classes::IConfirmationDialog`
             candidate #148: `godot::classes::IContainer`
             candidate #149: `godot::classes::IControl`
             candidate #150: `godot::classes::IConvexPolygonShape2D`
             candidate #151: `godot::classes::IConvexPolygonShape3D`
             candidate #152: `godot::classes::ICpuParticles2D`
             candidate #153: `godot::classes::ICpuParticles3D`
             candidate #154: `godot::classes::ICrypto`
             candidate #155: `godot::classes::ICryptoKey`
             candidate #156: `godot::classes::ICsgBox3D`
             candidate #157: `godot::classes::ICsgCombiner3D`
             candidate #158: `godot::classes::ICsgCylinder3D`
             candidate #159: `godot::classes::ICsgMesh3D`
             candidate #160: `godot::classes::ICsgPolygon3D`
             candidate #161: `godot::classes::ICsgSphere3D`
             candidate #162: `godot::classes::ICsgTorus3D`
             candidate #163: `godot::classes::ICubemap`
             candidate #164: `godot::classes::ICubemapArray`
             candidate #165: `godot::classes::ICurve`
             candidate #166: `godot::classes::ICurve2D`
             candidate #167: `godot::classes::ICurve3D`
             candidate #168: `godot::classes::ICurveTexture`
             candidate #169: `godot::classes::ICurveXyzTexture`
             candidate #170: `godot::classes::ICylinderMesh`
             candidate #171: `godot::classes::ICylinderShape3D`
             candidate #172: `godot::classes::IDampedSpringJoint2D`
             candidate #173: `godot::classes::IDecal`
             candidate #174: `godot::classes::IDirectionalLight2D`
             candidate #175: `godot::classes::IDirectionalLight3D`
             candidate #176: `godot::classes::IDtlsServer`
             candidate #177: `godot::classes::IENetConnection`
             candidate #178: `godot::classes::IENetMultiplayerPeer`
             candidate #179: `godot::classes::IEditorCommandPalette`
             candidate #180: `godot::classes::IEditorContextMenuPlugin`
             candidate #181: `godot::classes::IEditorDebuggerPlugin`
             candidate #182: `godot::classes::IEditorExportPlatformAndroid`
             candidate #183: `godot::classes::IEditorExportPlatformExtension`
             candidate #184: `godot::classes::IEditorExportPlatformIos`
             candidate #185: `godot::classes::IEditorExportPlatformLinuxBsd`
             candidate #186: `godot::classes::IEditorExportPlatformMacOs`
             candidate #187: `godot::classes::IEditorExportPlatformWeb`
             candidate #188: `godot::classes::IEditorExportPlatformWindows`
             candidate #189: `godot::classes::IEditorExportPlugin`
             candidate #190: `godot::classes::IEditorFeatureProfile`
             candidate #191: `godot::classes::IEditorFileDialog`
             candidate #192: `godot::classes::IEditorFileSystemDirectory`
             candidate #193: `godot::classes::IEditorFileSystemImportFormatSupportQuery`
             candidate #194: `godot::classes::IEditorImportPlugin`
             candidate #195: `godot::classes::IEditorInspector`
             candidate #196: `godot::classes::IEditorInspectorPlugin`
             candidate #197: `godot::classes::IEditorNode3DGizmo`
             candidate #198: `godot::classes::IEditorNode3DGizmoPlugin`
             candidate #199: `godot::classes::IEditorPaths`
             candidate #200: `godot::classes::IEditorPlugin`
             candidate #201: `godot::classes::IEditorProperty`
             candidate #202: `godot::classes::IEditorResourceConversionPlugin`
             candidate #203: `godot::classes::IEditorResourcePicker`
             candidate #204: `godot::classes::IEditorResourcePreviewGenerator`
             candidate #205: `godot::classes::IEditorResourceTooltipPlugin`
             candidate #206: `godot::classes::IEditorSceneFormatImporter`
             candidate #207: `godot::classes::IEditorSceneFormatImporterBlend`
             candidate #208: `godot::classes::IEditorSceneFormatImporterFbx2gltf`
             candidate #209: `godot::classes::IEditorSceneFormatImporterGltf`
             candidate #210: `godot::classes::IEditorSceneFormatImporterUfbx`
             candidate #211: `godot::classes::IEditorScenePostImport`
             candidate #212: `godot::classes::IEditorScenePostImportPlugin`
             candidate #213: `godot::classes::IEditorScript`
             candidate #214: `godot::classes::IEditorScriptPicker`
             candidate #215: `godot::classes::IEditorSelection`
             candidate #216: `godot::classes::IEditorSettings`
             candidate #217: `godot::classes::IEditorSpinSlider`
             candidate #218: `godot::classes::IEditorSyntaxHighlighter`
             candidate #219: `godot::classes::IEditorTranslationParserPlugin`
             candidate #220: `godot::classes::IEditorVcsInterface`
             candidate #221: `godot::classes::IEncodedObjectAsId`
             candidate #222: `godot::classes::IEngineProfiler`
             candidate #223: `godot::classes::IEnvironment`
             candidate #224: `godot::classes::IExpression`
             candidate #225: `godot::classes::IExternalTexture`
             candidate #226: `godot::classes::IFastNoiseLite`
             candidate #227: `godot::classes::IFbxDocument`
             candidate #228: `godot::classes::IFbxState`
             candidate #229: `godot::classes::IFileDialog`
             candidate #230: `godot::classes::IFlowContainer`
             candidate #231: `godot::classes::IFogMaterial`
             candidate #232: `godot::classes::IFogVolume`
             candidate #233: `godot::classes::IFontFile`
             candidate #234: `godot::classes::IFontVariation`
             candidate #235: `godot::classes::IFramebufferCacheRd`
             candidate #236: `godot::classes::IGDExtension`
             candidate #237: `godot::classes::IGDScript`
             candidate #238: `godot::classes::IGDScriptSyntaxHighlighter`
             candidate #239: `godot::classes::IGeneric6DofJoint3D`
             candidate #240: `godot::classes::IGeometryInstance3D`
             candidate #241: `godot::classes::IGltfAccessor`
             candidate #242: `godot::classes::IGltfAnimation`
             candidate #243: `godot::classes::IGltfBufferView`
             candidate #244: `godot::classes::IGltfCamera`
             candidate #245: `godot::classes::IGltfDocument`
             candidate #246: `godot::classes::IGltfDocumentExtension`
             candidate #247: `godot::classes::IGltfDocumentExtensionConvertImporterMesh`
             candidate #248: `godot::classes::IGltfLight`
             candidate #249: `godot::classes::IGltfMesh`
             candidate #250: `godot::classes::IGltfNode`
             candidate #251: `godot::classes::IGltfObjectModelProperty`
             candidate #252: `godot::classes::IGltfPhysicsBody`
             candidate #253: `godot::classes::IGltfPhysicsShape`
             candidate #254: `godot::classes::IGltfSkeleton`
             candidate #255: `godot::classes::IGltfSkin`
             candidate #256: `godot::classes::IGltfSpecGloss`
             candidate #257: `godot::classes::IGltfState`
             candidate #258: `godot::classes::IGltfTexture`
             candidate #259: `godot::classes::IGltfTextureSampler`
             candidate #260: `godot::classes::IGpuParticles2D`
             candidate #261: `godot::classes::IGpuParticles3D`
             candidate #262: `godot::classes::IGpuParticlesAttractorBox3D`
             candidate #263: `godot::classes::IGpuParticlesAttractorSphere3D`
             candidate #264: `godot::classes::IGpuParticlesAttractorVectorField3D`
             candidate #265: `godot::classes::IGpuParticlesCollisionBox3D`
             candidate #266: `godot::classes::IGpuParticlesCollisionHeightField3D`
             candidate #267: `godot::classes::IGpuParticlesCollisionSdf3d`
             candidate #268: `godot::classes::IGpuParticlesCollisionSphere3D`
             candidate #269: `godot::classes::IGradient`
             candidate #270: `godot::classes::IGradientTexture1D`
             candidate #271: `godot::classes::IGradientTexture2D`
             candidate #272: `godot::classes::IGridContainer`
             candidate #273: `godot::classes::IGridMap`
             candidate #274: `godot::classes::IGridMapEditorPlugin`
             candidate #275: `godot::classes::IGrooveJoint2D`
             candidate #276: `godot::classes::IHBoxContainer`
             candidate #277: `godot::classes::IHFlowContainer`
             candidate #278: `godot::classes::IHScrollBar`
             candidate #279: `godot::classes::IHSeparator`
             candidate #280: `godot::classes::IHSlider`
             candidate #281: `godot::classes::IHSplitContainer`
             candidate #282: `godot::classes::IHashingContext`
             candidate #283: `godot::classes::IHeightMapShape3D`
             candidate #284: `godot::classes::IHingeJoint3D`
             candidate #285: `godot::classes::IHmacContext`
             candidate #286: `godot::classes::IHttpClient`
             candidate #287: `godot::classes::IHttpRequest`
             candidate #288: `godot::classes::IImage`
             candidate #289: `godot::classes::IImageFormatLoaderExtension`
             candidate #290: `godot::classes::IImageTexture`
             candidate #291: `godot::classes::IImageTexture3D`
             candidate #292: `godot::classes::IImmediateMesh`
             candidate #293: `godot::classes::IImporterMesh`
             candidate #294: `godot::classes::IImporterMeshInstance3D`
             candidate #295: `godot::classes::IInputEventAction`
             candidate #296: `godot::classes::IInputEventJoypadButton`
             candidate #297: `godot::classes::IInputEventJoypadMotion`
             candidate #298: `godot::classes::IInputEventKey`
             candidate #299: `godot::classes::IInputEventMagnifyGesture`
             candidate #300: `godot::classes::IInputEventMidi`
             candidate #301: `godot::classes::IInputEventMouseButton`
             candidate #302: `godot::classes::IInputEventMouseMotion`
             candidate #303: `godot::classes::IInputEventPanGesture`
             candidate #304: `godot::classes::IInputEventScreenDrag`
             candidate #305: `godot::classes::IInputEventScreenTouch`
             candidate #306: `godot::classes::IInputEventShortcut`
             candidate #307: `godot::classes::IIntervalTweener`
             candidate #308: `godot::classes::IItemList`
             candidate #309: `godot::classes::IJavaObject`
             candidate #310: `godot::classes::IJson`
             candidate #311: `godot::classes::IJsonRpc`
             candidate #312: `godot::classes::IKinematicCollision2D`
             candidate #313: `godot::classes::IKinematicCollision3D`
             candidate #314: `godot::classes::ILabel`
             candidate #315: `godot::classes::ILabel3D`
             candidate #316: `godot::classes::ILabelSettings`
             candidate #317: `godot::classes::ILightOccluder2D`
             candidate #318: `godot::classes::ILightmapGi`
             candidate #319: `godot::classes::ILightmapGiData`
             candidate #320: `godot::classes::ILightmapProbe`
             candidate #321: `godot::classes::ILightmapperRd`
             candidate #322: `godot::classes::ILine2D`
             candidate #323: `godot::classes::ILineEdit`
             candidate #324: `godot::classes::ILinkButton`
             candidate #325: `godot::classes::ILookAtModifier3D`
             candidate #326: `godot::classes::IMainLoop`
             candidate #327: `godot::classes::IMarginContainer`
             candidate #328: `godot::classes::IMarker2D`
             candidate #329: `godot::classes::IMarker3D`
             candidate #330: `godot::classes::IMaterial`
             candidate #331: `godot::classes::IMenuBar`
             candidate #332: `godot::classes::IMenuButton`
             candidate #333: `godot::classes::IMesh`
             candidate #334: `godot::classes::IMeshConvexDecompositionSettings`
             candidate #335: `godot::classes::IMeshDataTool`
             candidate #336: `godot::classes::IMeshInstance2D`
             candidate #337: `godot::classes::IMeshInstance3D`
             candidate #338: `godot::classes::IMeshLibrary`
             candidate #339: `godot::classes::IMeshTexture`
             candidate #340: `godot::classes::IMethodTweener`
             candidate #341: `godot::classes::IMissingNode`
             candidate #342: `godot::classes::IMissingResource`
             candidate #343: `godot::classes::IMobileVrInterface`
             candidate #344: `godot::classes::IMovieWriter`
             candidate #345: `godot::classes::IMultiMesh`
             candidate #346: `godot::classes::IMultiMeshInstance2D`
             candidate #347: `godot::classes::IMultiMeshInstance3D`
             candidate #348: `godot::classes::IMultiplayerApiExtension`
             candidate #349: `godot::classes::IMultiplayerPeerExtension`
             candidate #350: `godot::classes::IMultiplayerSpawner`
             candidate #351: `godot::classes::IMultiplayerSynchronizer`
             candidate #352: `godot::classes::INinePatchRect`
             candidate #353: `godot::classes::INoiseTexture2D`
             candidate #354: `godot::classes::INoiseTexture3D`
             candidate #355: `godot::classes::IOccluderInstance3D`
             candidate #356: `godot::classes::IOccluderPolygon2D`
             candidate #357: `godot::classes::IOfflineMultiplayerPeer`
             candidate #358: `godot::classes::IOggPacketSequence`
             candidate #359: `godot::classes::IOggPacketSequencePlayback`
             candidate #360: `godot::classes::IOmniLight3D`
             candidate #361: `godot::classes::IOpenXrAction`
             candidate #362: `godot::classes::IOpenXrActionBindingModifier`
             candidate #363: `godot::classes::IOpenXrActionMap`
             candidate #364: `godot::classes::IOpenXrActionSet`
             candidate #365: `godot::classes::IOpenXrAnalogThresholdModifier`
             candidate #366: `godot::classes::IOpenXrApiExtension`
             candidate #367: `godot::classes::IOpenXrBindingModifierEditor`
             candidate #368: `godot::classes::IOpenXrCompositionLayerCylinder`
             candidate #369: `godot::classes::IOpenXrCompositionLayerEquirect`
             candidate #370: `godot::classes::IOpenXrCompositionLayerQuad`
             candidate #371: `godot::classes::IOpenXrDpadBindingModifier`
             candidate #372: `godot::classes::IOpenXrExtensionWrapperExtension`
             candidate #373: `godot::classes::IOpenXrHand`
             candidate #374: `godot::classes::IOpenXrHapticVibration`
             candidate #375: `godot::classes::IOpenXrInteractionProfile`
             candidate #376: `godot::classes::IOpenXrInteractionProfileEditor`
             candidate #377: `godot::classes::IOpenXrInteractionProfileMetadata`
             candidate #378: `godot::classes::IOpenXrInterface`
             candidate #379: `godot::classes::IOpenXrIpBinding`
             candidate #380: `godot::classes::IOpenXrVisibilityMask`
             candidate #381: `godot::classes::IOpenXripBindingModifier`
             candidate #382: `godot::classes::IOptimizedTranslation`
             candidate #383: `godot::classes::IOptionButton`
             candidate #384: `godot::classes::IOrmMaterial3D`
             candidate #385: `godot::classes::IPackedDataContainer`
             candidate #386: `godot::classes::IPacketPeerDtls`
             candidate #387: `godot::classes::IPacketPeerExtension`
             candidate #388: `godot::classes::IPacketPeerStream`
             candidate #389: `godot::classes::IPacketPeerUdp`
             candidate #390: `godot::classes::IPanel`
             candidate #391: `godot::classes::IPanelContainer`
             candidate #392: `godot::classes::IPanoramaSkyMaterial`
             candidate #393: `godot::classes::IParallaxBackground`
             candidate #394: `godot::classes::IParallaxLayer`
             candidate #395: `godot::classes::IParticleProcessMaterial`
             candidate #396: `godot::classes::IPath2D`
             candidate #397: `godot::classes::IPath3D`
             candidate #398: `godot::classes::IPathFollow2D`
             candidate #399: `godot::classes::IPathFollow3D`
             candidate #400: `godot::classes::IPckPacker`
             candidate #401: `godot::classes::IPhysicalBone2D`
             candidate #402: `godot::classes::IPhysicalBone3D`
             candidate #403: `godot::classes::IPhysicalBoneSimulator3D`
             candidate #404: `godot::classes::IPhysicalSkyMaterial`
             candidate #405: `godot::classes::IPhysicsDirectBodyState2DExtension`
             candidate #406: `godot::classes::IPhysicsDirectBodyState3DExtension`
             candidate #407: `godot::classes::IPhysicsDirectSpaceState2DExtension`
             candidate #408: `godot::classes::IPhysicsDirectSpaceState3DExtension`
             candidate #409: `godot::classes::IPhysicsMaterial`
             candidate #410: `godot::classes::IPhysicsPointQueryParameters2D`
             candidate #411: `godot::classes::IPhysicsPointQueryParameters3D`
             candidate #412: `godot::classes::IPhysicsRayQueryParameters2D`
             candidate #413: `godot::classes::IPhysicsRayQueryParameters3D`
             candidate #414: `godot::classes::IPhysicsServer2DExtension`
             candidate #415: `godot::classes::IPhysicsServer3DExtension`
             candidate #416: `godot::classes::IPhysicsServer3DRenderingServerHandler`
             candidate #417: `godot::classes::IPhysicsShapeQueryParameters2D`
             candidate #418: `godot::classes::IPhysicsShapeQueryParameters3D`
             candidate #419: `godot::classes::IPhysicsTestMotionParameters2D`
             candidate #420: `godot::classes::IPhysicsTestMotionParameters3D`
             candidate #421: `godot::classes::IPhysicsTestMotionResult2D`
             candidate #422: `godot::classes::IPhysicsTestMotionResult3D`
             candidate #423: `godot::classes::IPinJoint2D`
             candidate #424: `godot::classes::IPinJoint3D`
             candidate #425: `godot::classes::IPlaceholderCubemap`
             candidate #426: `godot::classes::IPlaceholderCubemapArray`
             candidate #427: `godot::classes::IPlaceholderMaterial`
             candidate #428: `godot::classes::IPlaceholderMesh`
             candidate #429: `godot::classes::IPlaceholderTexture2D`
             candidate #430: `godot::classes::IPlaceholderTexture2DArray`
             candidate #431: `godot::classes::IPlaceholderTexture3D`
             candidate #432: `godot::classes::IPlaneMesh`
             candidate #433: `godot::classes::IPointLight2D`
             candidate #434: `godot::classes::IPointMesh`
             candidate #435: `godot::classes::IPolygon2D`
             candidate #436: `godot::classes::IPolygonOccluder3D`
             candidate #437: `godot::classes::IPolygonPathFinder`
             candidate #438: `godot::classes::IPopup`
             candidate #439: `godot::classes::IPopupMenu`
             candidate #440: `godot::classes::IPopupPanel`
             candidate #441: `godot::classes::IPortableCompressedTexture2D`
             candidate #442: `godot::classes::IPrimitiveMesh`
             candidate #443: `godot::classes::IPrismMesh`
             candidate #444: `godot::classes::IProceduralSkyMaterial`
             candidate #445: `godot::classes::IProgressBar`
             candidate #446: `godot::classes::IPropertyTweener`
             candidate #447: `godot::classes::IQuadMesh`
             candidate #448: `godot::classes::IQuadOccluder3D`
             candidate #449: `godot::classes::IRandomNumberGenerator`
             candidate #450: `godot::classes::IRange`
             candidate #451: `godot::classes::IRayCast2D`
             candidate #452: `godot::classes::IRayCast3D`
             candidate #453: `godot::classes::IRdAttachmentFormat`
             candidate #454: `godot::classes::IRdFramebufferPass`
             candidate #455: `godot::classes::IRdPipelineColorBlendState`
             candidate #456: `godot::classes::IRdPipelineColorBlendStateAttachment`
             candidate #457: `godot::classes::IRdPipelineDepthStencilState`
             candidate #458: `godot::classes::IRdPipelineMultisampleState`
             candidate #459: `godot::classes::IRdPipelineRasterizationState`
             candidate #460: `godot::classes::IRdPipelineSpecializationConstant`
             candidate #461: `godot::classes::IRdSamplerState`
             candidate #462: `godot::classes::IRdShaderFile`
             candidate #463: `godot::classes::IRdShaderSource`
             candidate #464: `godot::classes::IRdShaderSpirv`
             candidate #465: `godot::classes::IRdTextureFormat`
             candidate #466: `godot::classes::IRdTextureView`
             candidate #467: `godot::classes::IRdUniform`
             candidate #468: `godot::classes::IRdVertexAttribute`
             candidate #469: `godot::classes::IRectangleShape2D`
             candidate #470: `godot::classes::IReferenceRect`
             candidate #471: `godot::classes::IReflectionProbe`
             candidate #472: `godot::classes::IRegEx`
             candidate #473: `godot::classes::IRegExMatch`
             candidate #474: `godot::classes::IRemoteTransform2D`
             candidate #475: `godot::classes::IRemoteTransform3D`
             candidate #476: `godot::classes::IRenderDataExtension`
             candidate #477: `godot::classes::IRenderDataRd`
             candidate #478: `godot::classes::IRenderSceneBuffersConfiguration`
             candidate #479: `godot::classes::IRenderSceneBuffersExtension`
             candidate #480: `godot::classes::IRenderSceneBuffersRd`
             candidate #481: `godot::classes::IRenderSceneDataExtension`
             candidate #482: `godot::classes::IRenderSceneDataRd`
             candidate #483: `godot::classes::IResourceFormatLoader`
             candidate #484: `godot::classes::IResourceFormatSaver`
             candidate #485: `godot::classes::IResourceImporterBitMap`
             candidate #486: `godot::classes::IResourceImporterBmFont`
             candidate #487: `godot::classes::IResourceImporterCsvTranslation`
             candidate #488: `godot::classes::IResourceImporterDynamicFont`
             candidate #489: `godot::classes::IResourceImporterImage`
             candidate #490: `godot::classes::IResourceImporterImageFont`
             candidate #491: `godot::classes::IResourceImporterLayeredTexture`
             candidate #492: `godot::classes::IResourceImporterMp3`
             candidate #493: `godot::classes::IResourceImporterObj`
             candidate #494: `godot::classes::IResourceImporterOggVorbis`
             candidate #495: `godot::classes::IResourceImporterScene`
             candidate #496: `godot::classes::IResourceImporterShaderFile`
             candidate #497: `godot::classes::IResourceImporterTexture`
             candidate #498: `godot::classes::IResourceImporterTextureAtlas`
             candidate #499: `godot::classes::IResourceImporterWav`
             candidate #500: `godot::classes::IResourcePreloader`
             candidate #501: `godot::classes::IRetargetModifier3D`
             candidate #502: `godot::classes::IRibbonTrailMesh`
             candidate #503: `godot::classes::IRichTextEffect`
             candidate #504: `godot::classes::IRichTextLabel`
             candidate #505: `godot::classes::IRigidBody2D`
             candidate #506: `godot::classes::IRigidBody3D`
             candidate #507: `godot::classes::IRootMotionView`
             candidate #508: `godot::classes::ISceneMultiplayer`
             candidate #509: `godot::classes::ISceneReplicationConfig`
             candidate #510: `godot::classes::IScriptCreateDialog`
             candidate #511: `godot::classes::IScriptExtension`
             candidate #512: `godot::classes::IScriptLanguageExtension`
             candidate #513: `godot::classes::IScrollContainer`
             candidate #514: `godot::classes::ISegmentShape2D`
             candidate #515: `godot::classes::ISeparationRayShape2D`
             candidate #516: `godot::classes::ISeparationRayShape3D`
             candidate #517: `godot::classes::IShaderGlobalsOverride`
             candidate #518: `godot::classes::IShaderInclude`
             candidate #519: `godot::classes::IShaderIncludeDb`
             candidate #520: `godot::classes::IShaderMaterial`
             candidate #521: `godot::classes::IShapeCast2D`
             candidate #522: `godot::classes::IShapeCast3D`
             candidate #523: `godot::classes::IShortcut`
             candidate #524: `godot::classes::ISkeleton2D`
             candidate #525: `godot::classes::ISkeleton3D`
             candidate #526: `godot::classes::ISkeletonIk3d`
             candidate #527: `godot::classes::ISkeletonModifier3D`
             candidate #528: `godot::classes::ISkeletonProfile`
             candidate #529: `godot::classes::ISkeletonProfileHumanoid`
             candidate #530: `godot::classes::ISkin`
             candidate #531: `godot::classes::ISky`
             candidate #532: `godot::classes::ISliderJoint3D`
             candidate #533: `godot::classes::ISoftBody3D`
             candidate #534: `godot::classes::ISphereMesh`
             candidate #535: `godot::classes::ISphereOccluder3D`
             candidate #536: `godot::classes::ISphereShape3D`
             candidate #537: `godot::classes::ISpinBox`
             candidate #538: `godot::classes::ISplitContainer`
             candidate #539: `godot::classes::ISpotLight3D`
             candidate #540: `godot::classes::ISpringArm3D`
             candidate #541: `godot::classes::ISpringBoneCollision3D`
             candidate #542: `godot::classes::ISpringBoneCollisionCapsule3D`
             candidate #543: `godot::classes::ISpringBoneCollisionPlane3D`
             candidate #544: `godot::classes::ISpringBoneCollisionSphere3D`
             candidate #545: `godot::classes::ISpringBoneSimulator3D`
             candidate #546: `godot::classes::ISprite2D`
             candidate #547: `godot::classes::ISprite3D`
             candidate #548: `godot::classes::ISpriteFrames`
             candidate #549: `godot::classes::IStandardMaterial3D`
             candidate #550: `godot::classes::IStaticBody2D`
             candidate #551: `godot::classes::IStaticBody3D`
             candidate #552: `godot::classes::IStatusIndicator`
             candidate #553: `godot::classes::IStreamPeerBuffer`
             candidate #554: `godot::classes::IStreamPeerExtension`
             candidate #555: `godot::classes::IStreamPeerTcp`
             candidate #556: `godot::classes::IStreamPeerTls`
             candidate #557: `godot::classes::IStyleBox`
             candidate #558: `godot::classes::IStyleBoxEmpty`
             candidate #559: `godot::classes::IStyleBoxFlat`
             candidate #560: `godot::classes::IStyleBoxLine`
             candidate #561: `godot::classes::IStyleBoxTexture`
             candidate #562: `godot::classes::ISubViewport`
             candidate #563: `godot::classes::ISubViewportContainer`
             candidate #564: `godot::classes::ISubtweenTweener`
             candidate #565: `godot::classes::ISurfaceTool`
             candidate #566: `godot::classes::ISyntaxHighlighter`
             candidate #567: `godot::classes::ISystemFont`
             candidate #568: `godot::classes::ITabBar`
             candidate #569: `godot::classes::ITabContainer`
             candidate #570: `godot::classes::ITcpServer`
             candidate #571: `godot::classes::ITextEdit`
             candidate #572: `godot::classes::ITextLine`
             candidate #573: `godot::classes::ITextMesh`
             candidate #574: `godot::classes::ITextParagraph`
             candidate #575: `godot::classes::ITextServerAdvanced`
             candidate #576: `godot::classes::ITextServerDummy`
             candidate #577: `godot::classes::ITextServerExtension`
             candidate #578: `godot::classes::ITexture`
             candidate #579: `godot::classes::ITexture2D`
             candidate #580: `godot::classes::ITexture2DArray`
             candidate #581: `godot::classes::ITexture2DArrayRd`
             candidate #582: `godot::classes::ITexture2Drd`
             candidate #583: `godot::classes::ITexture3D`
             candidate #584: `godot::classes::ITexture3Drd`
             candidate #585: `godot::classes::ITextureButton`
             candidate #586: `godot::classes::ITextureCubemapArrayRd`
             candidate #587: `godot::classes::ITextureCubemapRd`
             candidate #588: `godot::classes::ITextureLayered`
             candidate #589: `godot::classes::ITextureProgressBar`
             candidate #590: `godot::classes::ITextureRect`
             candidate #591: `godot::classes::ITheme`
             candidate #592: `godot::classes::ITileData`
             candidate #593: `godot::classes::ITileMap`
             candidate #594: `godot::classes::ITileMapLayer`
             candidate #595: `godot::classes::ITileMapPattern`
             candidate #596: `godot::classes::ITileSet`
             candidate #597: `godot::classes::ITileSetAtlasSource`
             candidate #598: `godot::classes::ITileSetScenesCollectionSource`
             candidate #599: `godot::classes::ITimer`
             candidate #600: `godot::classes::ITorusMesh`
             candidate #601: `godot::classes::ITouchScreenButton`
             candidate #602: `godot::classes::ITranslation`
             candidate #603: `godot::classes::ITranslationDomain`
             candidate #604: `godot::classes::ITree`
             candidate #605: `godot::classes::ITriangleMesh`
             candidate #606: `godot::classes::ITubeTrailMesh`
             candidate #607: `godot::classes::ITween`
             candidate #608: `godot::classes::IUdpServer`
             candidate #609: `godot::classes::IUndoRedo`
             candidate #610: `godot::classes::IUniformSetCacheRd`
             candidate #611: `godot::classes::IUpnp`
             candidate #612: `godot::classes::IUpnpDevice`
             candidate #613: `godot::classes::IVBoxContainer`
             candidate #614: `godot::classes::IVFlowContainer`
             candidate #615: `godot::classes::IVScrollBar`
             candidate #616: `godot::classes::IVSeparator`
             candidate #617: `godot::classes::IVSlider`
             candidate #618: `godot::classes::IVSplitContainer`
             candidate #619: `godot::classes::IVehicleBody3D`
             candidate #620: `godot::classes::IVehicleWheel3D`
             candidate #621: `godot::classes::IVideoStream`
             candidate #622: `godot::classes::IVideoStreamPlayback`
             candidate #623: `godot::classes::IVideoStreamPlayer`
             candidate #624: `godot::classes::IVideoStreamTheora`
             candidate #625: `godot::classes::IViewportTexture`
             candidate #626: `godot::classes::IVisibleOnScreenEnabler2D`
             candidate #627: `godot::classes::IVisibleOnScreenEnabler3D`
             candidate #628: `godot::classes::IVisibleOnScreenNotifier2D`
             candidate #629: `godot::classes::IVisibleOnScreenNotifier3D`
             candidate #630: `godot::classes::IVisualInstance3D`
             candidate #631: `godot::classes::IVisualShader`
             candidate #632: `godot::classes::IVisualShaderNodeBillboard`
             candidate #633: `godot::classes::IVisualShaderNodeBooleanConstant`
             candidate #634: `godot::classes::IVisualShaderNodeBooleanParameter`
             candidate #635: `godot::classes::IVisualShaderNodeClamp`
             candidate #636: `godot::classes::IVisualShaderNodeColorConstant`
             candidate #637: `godot::classes::IVisualShaderNodeColorFunc`
             candidate #638: `godot::classes::IVisualShaderNodeColorOp`
             candidate #639: `godot::classes::IVisualShaderNodeColorParameter`
             candidate #640: `godot::classes::IVisualShaderNodeComment`
             candidate #641: `godot::classes::IVisualShaderNodeCompare`
             candidate #642: `godot::classes::IVisualShaderNodeCubemap`
             candidate #643: `godot::classes::IVisualShaderNodeCubemapParameter`
             candidate #644: `godot::classes::IVisualShaderNodeCurveTexture`
             candidate #645: `godot::classes::IVisualShaderNodeCurveXyzTexture`
             candidate #646: `godot::classes::IVisualShaderNodeCustom`
             candidate #647: `godot::classes::IVisualShaderNodeDerivativeFunc`
             candidate #648: `godot::classes::IVisualShaderNodeDeterminant`
             candidate #649: `godot::classes::IVisualShaderNodeDistanceFade`
             candidate #650: `godot::classes::IVisualShaderNodeDotProduct`
             candidate #651: `godot::classes::IVisualShaderNodeExpression`
             candidate #652: `godot::classes::IVisualShaderNodeFaceForward`
             candidate #653: `godot::classes::IVisualShaderNodeFloatConstant`
             candidate #654: `godot::classes::IVisualShaderNodeFloatFunc`
             candidate #655: `godot::classes::IVisualShaderNodeFloatOp`
             candidate #656: `godot::classes::IVisualShaderNodeFloatParameter`
             candidate #657: `godot::classes::IVisualShaderNodeFrame`
             candidate #658: `godot::classes::IVisualShaderNodeFresnel`
             candidate #659: `godot::classes::IVisualShaderNodeGlobalExpression`
             candidate #660: `godot::classes::IVisualShaderNodeIf`
             candidate #661: `godot::classes::IVisualShaderNodeInput`
             candidate #662: `godot::classes::IVisualShaderNodeIntConstant`
             candidate #663: `godot::classes::IVisualShaderNodeIntFunc`
             candidate #664: `godot::classes::IVisualShaderNodeIntOp`
             candidate #665: `godot::classes::IVisualShaderNodeIntParameter`
             candidate #666: `godot::classes::IVisualShaderNodeIs`
             candidate #667: `godot::classes::IVisualShaderNodeLinearSceneDepth`
             candidate #668: `godot::classes::IVisualShaderNodeMix`
             candidate #669: `godot::classes::IVisualShaderNodeMultiplyAdd`
             candidate #670: `godot::classes::IVisualShaderNodeOuterProduct`
             candidate #671: `godot::classes::IVisualShaderNodeParameterRef`
             candidate #672: `godot::classes::IVisualShaderNodeParticleAccelerator`
             candidate #673: `godot::classes::IVisualShaderNodeParticleBoxEmitter`
             candidate #674: `godot::classes::IVisualShaderNodeParticleConeVelocity`
             candidate #675: `godot::classes::IVisualShaderNodeParticleEmit`
             candidate #676: `godot::classes::IVisualShaderNodeParticleMeshEmitter`
             candidate #677: `godot::classes::IVisualShaderNodeParticleMultiplyByAxisAngle`
             candidate #678: `godot::classes::IVisualShaderNodeParticleOutput`
             candidate #679: `godot::classes::IVisualShaderNodeParticleRandomness`
             candidate #680: `godot::classes::IVisualShaderNodeParticleRingEmitter`
             candidate #681: `godot::classes::IVisualShaderNodeParticleSphereEmitter`
             candidate #682: `godot::classes::IVisualShaderNodeProximityFade`
             candidate #683: `godot::classes::IVisualShaderNodeRandomRange`
             candidate #684: `godot::classes::IVisualShaderNodeRemap`
             candidate #685: `godot::classes::IVisualShaderNodeReroute`
             candidate #686: `godot::classes::IVisualShaderNodeRotationByAxis`
             candidate #687: `godot::classes::IVisualShaderNodeScreenNormalWorldSpace`
             candidate #688: `godot::classes::IVisualShaderNodeScreenUvToSdf`
             candidate #689: `godot::classes::IVisualShaderNodeSdfRaymarch`
             candidate #690: `godot::classes::IVisualShaderNodeSdfToScreenUv`
             candidate #691: `godot::classes::IVisualShaderNodeSmoothStep`
             candidate #692: `godot::classes::IVisualShaderNodeStep`
             candidate #693: `godot::classes::IVisualShaderNodeSwitch`
             candidate #694: `godot::classes::IVisualShaderNodeTexture`
             candidate #695: `godot::classes::IVisualShaderNodeTexture2DArray`
             candidate #696: `godot::classes::IVisualShaderNodeTexture2DArrayParameter`
             candidate #697: `godot::classes::IVisualShaderNodeTexture2DParameter`
             candidate #698: `godot::classes::IVisualShaderNodeTexture3D`
             candidate #699: `godot::classes::IVisualShaderNodeTexture3DParameter`
             candidate #700: `godot::classes::IVisualShaderNodeTextureParameterTriplanar`
             candidate #701: `godot::classes::IVisualShaderNodeTextureSdf`
             candidate #702: `godot::classes::IVisualShaderNodeTextureSdfNormal`
             candidate #703: `godot::classes::IVisualShaderNodeTransformCompose`
             candidate #704: `godot::classes::IVisualShaderNodeTransformConstant`
             candidate #705: `godot::classes::IVisualShaderNodeTransformDecompose`
             candidate #706: `godot::classes::IVisualShaderNodeTransformFunc`
             candidate #707: `godot::classes::IVisualShaderNodeTransformOp`
             candidate #708: `godot::classes::IVisualShaderNodeTransformParameter`
             candidate #709: `godot::classes::IVisualShaderNodeTransformVecMult`
             candidate #710: `godot::classes::IVisualShaderNodeUIntConstant`
             candidate #711: `godot::classes::IVisualShaderNodeUIntFunc`
             candidate #712: `godot::classes::IVisualShaderNodeUIntOp`
             candidate #713: `godot::classes::IVisualShaderNodeUIntParameter`
             candidate #714: `godot::classes::IVisualShaderNodeUvFunc`
             candidate #715: `godot::classes::IVisualShaderNodeUvPolarCoord`
             candidate #716: `godot::classes::IVisualShaderNodeVaryingGetter`
             candidate #717: `godot::classes::IVisualShaderNodeVaryingSetter`
             candidate #718: `godot::classes::IVisualShaderNodeVec2Constant`
             candidate #719: `godot::classes::IVisualShaderNodeVec2Parameter`
             candidate #720: `godot::classes::IVisualShaderNodeVec3Constant`
             candidate #721: `godot::classes::IVisualShaderNodeVec3Parameter`
             candidate #722: `godot::classes::IVisualShaderNodeVec4Constant`
             candidate #723: `godot::classes::IVisualShaderNodeVec4Parameter`
             candidate #724: `godot::classes::IVisualShaderNodeVectorCompose`
             candidate #725: `godot::classes::IVisualShaderNodeVectorDecompose`
             candidate #726: `godot::classes::IVisualShaderNodeVectorDistance`
             candidate #727: `godot::classes::IVisualShaderNodeVectorFunc`
             candidate #728: `godot::classes::IVisualShaderNodeVectorLen`
             candidate #729: `godot::classes::IVisualShaderNodeVectorOp`
             candidate #730: `godot::classes::IVisualShaderNodeVectorRefract`
             candidate #731: `godot::classes::IVisualShaderNodeWorldPositionFromDepth`
             candidate #732: `godot::classes::IVoxelGi`
             candidate #733: `godot::classes::IVoxelGiData`
             candidate #734: `godot::classes::IWeakRef`
             candidate #735: `godot::classes::IWebRtcDataChannelExtension`
             candidate #736: `godot::classes::IWebRtcMultiplayerPeer`
             candidate #737: `godot::classes::IWebRtcPeerConnection`
             candidate #738: `godot::classes::IWebRtcPeerConnectionExtension`
             candidate #739: `godot::classes::IWebSocketMultiplayerPeer`
             candidate #740: `godot::classes::IWebSocketPeer`
             candidate #741: `godot::classes::IWindow`
             candidate #742: `godot::classes::IWorld2D`
             candidate #743: `godot::classes::IWorld3D`
             candidate #744: `godot::classes::IWorldBoundaryShape2D`
             candidate #745: `godot::classes::IWorldBoundaryShape3D`
             candidate #746: `godot::classes::IWorldEnvironment`
             candidate #747: `godot::classes::IX509Certificate`
             candidate #748: `godot::classes::IXmlParser`
             candidate #749: `godot::classes::IXrAnchor3D`
             candidate #750: `godot::classes::IXrCamera3D`
             candidate #751: `godot::classes::IXrController3D`
             candidate #752: `godot::classes::IXrControllerTracker`
             candidate #753: `godot::classes::IXrHandModifier3D`
             candidate #754: `godot::classes::IXrHandTracker`
             candidate #755: `godot::classes::IXrInterfaceExtension`
             candidate #756: `godot::classes::IXrNode3D`
             candidate #757: `godot::classes::IXrOrigin3D`
             candidate #758: `godot::classes::IXrPose`
             candidate #759: `godot::classes::IXrPositionalTracker`
             candidate #760: `godot::classes::IXrvrs`
             candidate #761: `godot::classes::IZipPacker`
             candidate #762: `godot::classes::IZipReader`
             candidate #763: `godot::prelude::INode`
             candidate #764: `godot::prelude::INode2D`
             candidate #765: `godot::prelude::INode3D`
             candidate #766: `godot::prelude::IObject`
             candidate #767: `godot::prelude::IPackedScene`
             candidate #768: `godot::prelude::IRefCounted`
             candidate #769: `godot::prelude::IResource`
             candidate #770: `godot::prelude::ISceneTree`
     = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8357335806657853199.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0308]: mismatched types
   --> src\zv9__godot_interface__api__engine.rs:212:13
    |
212 |             seed,
    |             ^^^^ expected `u64`, found `i64`
    |
help: you can convert an `i64` to a `u64` and panic if the converted value doesn't fit
    |
212 |             seed: seed.try_into().unwrap(),
    |             +++++     ++++++++++++++++++++

error[E0277]: the trait bound `GodotNoiseType: From<&str>` is not satisfied
   --> src\zv9__godot_interface__api__engine.rs:214:42
    |
214 |                 "automata" => "automata".into(),
    |                                          ^^^^ unsatisfied trait bound
    |
    = help: the trait `From<&str>` is not implemented for `GodotNoiseType`
            but trait `From<godot::prelude::GString>` is implemented for it
    = help: for that trait implementation, expected `godot::prelude::GString`, found `&str`
    = note: required for `&str` to implement `Into<GodotNoiseType>`
    = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8357335806657853199.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0277]: the trait bound `GodotNoiseType: From<&str>` is not satisfied
   --> src\zv9__godot_interface__api__engine.rs:215:36
    |
215 |                 "basic" => "basic".into(),
    |                                    ^^^^ unsatisfied trait bound
    |
    = help: the trait `From<&str>` is not implemented for `GodotNoiseType`
            but trait `From<godot::prelude::GString>` is implemented for it
    = help: for that trait implementation, expected `godot::prelude::GString`, found `&str`
    = note: required for `&str` to implement `Into<GodotNoiseType>`
    = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8357335806657853199.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0308]: mismatched types
   --> src\zv9__godot_interface__api__engine.rs:219:13
    |
219 |             black,
    |             ^^^^^ expected `SerializableVector2i`, found `Vector2i`
    |
help: call `Into::into` on this expression to convert `godot::prelude::Vector2i` into `zv9__aetherion__pipeline_data__vector::SerializableVector2i`
    |
219 |             black: black.into(),
    |             ++++++      +++++++

error[E0308]: mismatched types
   --> src\zv9__godot_interface__api__engine.rs:220:13
    |
220 |             blue,
    |             ^^^^ expected `SerializableVector2i`, found `Vector2i`
    |
help: call `Into::into` on this expression to convert `godot::prelude::Vector2i` into `zv9__aetherion__pipeline_data__vector::SerializableVector2i`
    |
220 |             blue: blue.into(),
    |             +++++     +++++++

error[E0277]: the trait bound `GodotNoiseType: From<&str>` is not satisfied
   --> src\zv9__godot_interface__api__engine.rs:216:30
    |
216 |                 _ => "basic".into(),
    |                              ^^^^ unsatisfied trait bound
    |
    = help: the trait `From<&str>` is not implemented for `GodotNoiseType`
            but trait `From<godot::prelude::GString>` is implemented for it
    = help: for that trait implementation, expected `godot::prelude::GString`, found `&str`
    = note: required for `&str` to implement `Into<GodotNoiseType>`
    = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8357335806657853199.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0277]: `#[var]` properties require `Var` trait; #[export] ones require `Export` trait
  --> src\zv9__godot_interface__interface__controls.rs:55:19
   |
55 |     terrain_mode: String,
   |                   ^^^^^^ type cannot be used as a property
   |
   = help: the trait `Var` is not implemented for `std::string::String`
   = note: see also: https://godot-rust.github.io/book/register/properties.html
   = help: the following other types implement trait `Var`:
             Aabb
             Basis
             Callable
             DynGd<T, D>
             NodePath
             OnEditor<DynGd<T, D>>
             OnEditor<T>
             OnEditor<godot::prelude::Gd<T>>
           and 36 others

error[E0277]: `#[var]` properties require `Var` trait; #[export] ones require `Export` trait
  --> src\zv9__godot_interface__interface__controls.rs:59:21
   |
59 |     structure_mode: String,
   |                     ^^^^^^ type cannot be used as a property
   |
   = help: the trait `Var` is not implemented for `std::string::String`
   = note: see also: https://godot-rust.github.io/book/register/properties.html
   = help: the following other types implement trait `Var`:
             Aabb
             Basis
             Callable
             DynGd<T, D>
             NodePath
             OnEditor<DynGd<T, D>>
             OnEditor<T>
             OnEditor<godot::prelude::Gd<T>>
           and 36 others

error[E0277]: `#[var]` properties require `Var` trait; #[export] ones require `Export` trait
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^ type cannot be used as a property
   |
   = help: the trait `Export` is not implemented for `std::string::String`
   = note: see also: https://godot-rust.github.io/book/register/properties.html
   = note: `Gd` and `DynGd` cannot be exported directly; wrap them in `Option<...>` or `OnEditor<...>`.
   = help: the following other types implement trait `Export`:
             Aabb
             Basis
             NodePath
             OnEditor<DynGd<T, D>>
             OnEditor<T>
             OnEditor<godot::prelude::Gd<T>>
             PackedArray<T>
             PhantomVar<T>
           and 34 others
note: required by a bound in `register_export`
  --> C:\zv9\zv9.gdext\godot-core\src\registry\godot_register_wrappers.rs:20:42
   |
20 | pub fn register_export<C: GodotClass, T: Export>(
   |                                          ^^^^^^ required by this bound in `register_export`
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-5120371895506982694.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0034]: multiple applicable items in scope
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^ multiple `set_terrain_mode` found
   |
note: candidate #1 is defined in an impl for the type `__godot_ControlPanel_Funcs`
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `__godot_ControlPanel_Funcs`
  --> src\zv9__godot_interface__interface__controls.rs:70:1
   |
70 | #[godot_api]
   | ^^^^^^^^^^^^
   = note: this error originates in the derive macro `GodotClass` which comes from the expansion of the attribute macro `godot_api` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `#[var]` properties require `Var` trait; #[export] ones require `Export` trait
  --> src\zv9__godot_interface__interface__controls.rs:55:19
   |
55 |     terrain_mode: String,
   |                   ^^^^^^ type cannot be used as a property
   |
   = help: the trait `Export` is not implemented for `std::string::String`
   = note: see also: https://godot-rust.github.io/book/register/properties.html
   = note: `Gd` and `DynGd` cannot be exported directly; wrap them in `Option<...>` or `OnEditor<...>`.
   = help: the following other types implement trait `Export`:
             Aabb
             Basis
             NodePath
             OnEditor<DynGd<T, D>>
             OnEditor<T>
             OnEditor<godot::prelude::Gd<T>>
             PackedArray<T>
             PhantomVar<T>
           and 34 others
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-5120371895506982694.txt'
   = note: consider using `--verbose` to print the full type name to the console

error[E0034]: multiple applicable items in scope
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^ multiple `set_structure_mode` found
   |
note: candidate #1 is defined in an impl for the type `__godot_ControlPanel_Funcs`
  --> src\zv9__godot_interface__interface__controls.rs:47:10
   |
47 | #[derive(GodotClass)]
   |          ^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `__godot_ControlPanel_Funcs`
  --> src\zv9__godot_interface__interface__controls.rs:70:1
   |
70 | #[godot_api]
   | ^^^^^^^^^^^^
   = note: this error originates in the derive macro `GodotClass` which comes from the expansion of the attribute macro `godot_api` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `#[var]` properties require `Var` trait; #[export] ones require `Export` trait
  --> src\zv9__godot_interface__interface__controls.rs:59:21
   |
59 |     structure_mode: String,
   |                     ^^^^^^ type cannot be used as a property
   |
   = help: the trait `Export` is not implemented for `std::string::String`
   = note: see also: https://godot-rust.github.io/book/register/properties.html
   = note: `Gd` and `DynGd` cannot be exported directly; wrap them in `Option<...>` or `OnEditor<...>`.
   = help: the following other types implement trait `Export`:
             Aabb
             Basis
             NodePath
             OnEditor<DynGd<T, D>>
             OnEditor<T>
             OnEditor<godot::prelude::Gd<T>>
             PackedArray<T>
             PhantomVar<T>
           and 34 others
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-5120371895506982694.txt'
   = note: consider using `--verbose` to print the full type name to the console

error[E0034]: multiple applicable items in scope
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^ multiple `set_terrain_mode` found
    |
note: candidate #1 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:104:5
    |
104 |     fn set_terrain_mode(&mut self, mode: String) {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0034]: multiple applicable items in scope
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^ multiple `set_structure_mode` found
    |
note: candidate #1 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:110:5
    |
110 |     fn set_structure_mode(&mut self, mode: String) {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no variant or associated item named `from_str` found for enum `GodotNoiseType` in the current scope
  --> src\zv9__godot_interface__interface__controls.rs:86:35
   |
86 | ... GodotNoiseType::from_str(&self.terrain_mode),
   |                     ^^^^^^^^ variant or associated item not found in `GodotNoiseType`
   |
  ::: src\zv9__aetherion__structure__generation.rs:53:1
   |
53 | pub enum GodotNoiseType {
   | ----------------------- variant or associated item `from_str` not found for this enum
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `from_str`, perhaps you need to implement it:
           candidate #1: `FromStr`
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8357335806657853199.txt'
   = note: consider using `--verbose` to print the full type name to the console
help: there is an associated function `from` with a similar name
   |
86 -             mode: GodotNoiseType::from_str(&self.terrain_mode),
86 +             mode: GodotNoiseType::from(&self.terrain_mode),
   |

error[E0560]: struct `zv9__aetherion__structure__generation::MapBuildOptions` has no field named `structure`
  --> src\zv9__godot_interface__interface__controls.rs:87:13
   |
87 |             structure: self.structure_mode.clone(),
   |             ^^^^^^^^^ `zv9__aetherion__structure__generation::MapBuildOptions` does not have this field
   |
   = note: available fields are: `width`, `height`, `seed`, `black`, `blue`

error[E0560]: struct `zv9__aetherion__structure__generation::MapBuildOptions` has no field named `pacing_ms`
  --> src\zv9__godot_interface__interface__controls.rs:88:13
   |
88 |             pacing_ms: self.pacing_ms,
   |             ^^^^^^^^^ `zv9__aetherion__structure__generation::MapBuildOptions` does not have this field
   |
   = note: available fields are: `width`, `height`, `seed`, `black`, `blue`

error[E0599]: no function or associated item named `spawn_map` found for struct `AetherionEngine` in the current scope
  --> src\zv9__godot_interface__interface__controls.rs:93:26
   |
93 |         AetherionEngine::spawn_map(options);
   |                          ^^^^^^^^^ function or associated item not found in `AetherionEngine`
   |
  ::: src\zv9__godot_interface__api__engine.rs:48:1
   |
48 | pub struct AetherionEngine {
   | -------------------------- function or associated item `spawn_map` not found for this struct
   |
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8158811718138552742.txt'
   = note: consider using `--verbose` to print the full type name to the console

error[E0034]: multiple applicable items in scope
   --> src\zv9__godot_interface__interface__controls.rs:104:8
    |
104 |     fn set_terrain_mode(&mut self, mode: String) {
    |        ^^^^^^^^^^^^^^^^ multiple `set_terrain_mode` found
    |
note: candidate #1 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:104:5
    |
104 |     fn set_terrain_mode(&mut self, mode: String) {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0034]: multiple applicable items in scope
   --> src\zv9__godot_interface__interface__controls.rs:110:8
    |
110 |     fn set_structure_mode(&mut self, mode: String) {
    |        ^^^^^^^^^^^^^^^^^^ multiple `set_structure_mode` found
    |
note: candidate #1 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:47:10
    |
 47 | #[derive(GodotClass)]
    |          ^^^^^^^^^^
note: candidate #2 is defined in an impl for the type `ControlPanel`
   --> src\zv9__godot_interface__interface__controls.rs:110:5
    |
110 |     fn set_structure_mode(&mut self, mode: String) {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: this error originates in the derive macro `GodotClass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `set_text` found for struct `godot::prelude::Base<T>` in the current scope
  --> src\zv9__godot_interface__interface__diagnostics.rs:63:19
   |
63 |         self.base.set_text(text.into());
   |                   ^^^^^^^^ method not found in `Base<Label>`
   |
   = note: the full name for the type has been written to 'C:\zv9\zv9.aetherion\rust\target\debug\deps\aetherion_engine.long-type-8290666096188749636.txt'
   = note: consider using `--verbose` to print the full type name to the console

warning: unused import: `crate::zv9__prelude`
  --> src\zv9__aetherion__core__runtime.rs:41:5
   |
41 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
 --> src\zv9__aetherion__codegen__dsl.rs:6:5
  |
6 | use crate::zv9__prelude::*;
  |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
 --> src\zv9__util__profiling.rs:2:5
  |
2 | use crate::zv9__prelude::*;
  |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
  --> src\zv9__godot_interface__api__config.rs:39:5
   |
39 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
  --> src\zv9__util__logging.rs:43:5
   |
43 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
  --> src\zv9__shared__spatial.rs:41:5
   |
41 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
  --> src\zv9__godot_interface__interface__diagnostics.rs:39:5
   |
39 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::zv9__prelude`
  --> src\zv9__godot_interface__api__signals.rs:38:5
   |
38 | use crate::zv9__prelude::*;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused variable: `script`
  --> src\zv9__aetherion__codegen__dsl.rs:56:13
   |
56 |     let mut script = DslScript::new();
   |             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_script`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `source`
  --> src\zv9__aetherion__codegen__dsl.rs:55:18
   |
55 | pub fn parse_dsl(source: &str) -> Result<DslScript, St...
   |                  ^^^^^^ help: if this is intentional, prefix it with an underscore: `_source`

warning: variable does not need to be mutable
  --> src\zv9__aetherion__codegen__dsl.rs:56:9
   |
56 |     let mut script = DslScript::new();
   |         ----^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

error[E0596]: cannot borrow `debouncer` as mutable, as it is not declared as mutable
  --> src\zv9__trailkeeper__watch.rs:90:9
   |
90 | ...   debouncer.watcher().watch(path, RecursiveMode::N...
   |       ^^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
54 |     let mut debouncer = new_debouncer(debounce_duration, move |result: Result<Vec<DebouncedEvent>, Error>| {
   |         +++

Some errors have detailed explanations: E0034, E0277, E0308, E0560, E0592, E0596, E0599.
For more information about an error, try `rustc --explain E0034`.
warning: `aetherion_engine` (lib) generated 35 warnings
error: could not compile `aetherion_engine` (lib) due to 35 previous errors; 35 warnings emitted
PS C:\zv9\zv9.aetherion\rust>