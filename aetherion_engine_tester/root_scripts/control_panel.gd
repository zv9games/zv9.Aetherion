extends Control

# ------------------------------------------------------------------------------
# --- CORE ENGINE CONSTANTS ---
# ------------------------------------------------------------------------------
const CHUNK_SIZE_X: int = 32
const CHUNK_SIZE_Y: int = 32
const DEFAULT_CHUNKS: int = 4 # Default map size 128x128 (4 chunks * 32 tiles/chunk)

# ------------------------------------------------------------------------------
# 🧭 UI NODE REFERENCES
# ------------------------------------------------------------------------------
@onready var grid_width: SpinBox = $gridwidthspinbox
@onready var grid_height: SpinBox = $gridheightspinbox
@onready var seed_input: LineEdit = $seedlineedit
@onready var placement_mode_selector: OptionButton = $placementoptionbutton
# 🛑 FIX: Changed OptionBox to the correct Godot type: OptionButton
@onready var tile_type_selector: OptionButton = $tiletypeoptionbutton 
@onready var animate_checkbox: CheckBox = $animatecheckbox
@onready var status_label: Label = $billboard
@onready var generate_button: Button = $ignition
@onready var progress_bar: ProgressBar = $progressbar
@onready var toggle_terminal_button: Button = $toggleterminalbutton 
@onready var engine_timer: Timer = $enginetimer
@onready var engine_timer_label: Label = $enginetimerlabel
@onready var tiles_placed_label: Label = $tilesplacedlabel
@onready var tile_placement_time_label: Label = $tiletimeofplacement


# 🧠 EXTERNAL SCENE REFERENCES (Using SSXL node names from console output)
@onready var main: Node2D = get_parent() as Node2D
@onready var aetherion_engine: Node = main.get_node("AetherionEngine")
@onready var aetherion_signals: Node = main.get_node("AetherionSignals")
@onready var expansive_tilemap: TileMap = main.get_node("expansive_tilemap") as TileMap
@onready var clock_label: Label = main.get_node("tilemap/clocklabel") as Label
@onready var clock_timer: Timer = main.get_node("tilemap/clocktimer") as Timer
@onready var tilemap_node: Node2D = main.get_node("tilemap") as Node2D
@onready var cameras: Node = main.get_node("cameras")
@onready var camera1: Camera2D = main.get_node("cameras/camera1") as Camera2D
@onready var camera2: Camera2D = main.get_node("cameras/camera2") as Camera2D

# 📊 STATE VARIABLES
var last_percent: int = -1
var tile_size: Vector2 = Vector2(16, 16)
var panel_collapsed: bool = false
var engine_tick_count: int = 0
var total_tiles_placed: int = 0
var is_generating: bool = false
var initial_zoom_set: bool = false
var current_camera_id: int = 1 
var generation_start_time_ms: int = 0

# 🛑 FIX STATE: RACE CONDITION TRACKING
var total_chunks_expected: int = 0
var chunks_processed: int = 0
var engine_signaled_complete: bool = false # Flag to track Rust engine completion signal

# ------------------------------------------------------------------------------
# 🧭 BOOT SEQUENCE
# ------------------------------------------------------------------------------
func _ready() -> void:
	# Critical path check
	if not is_instance_valid(expansive_tilemap):
		push_error("❌ FATAL: Initialization Error: expansive_tilemap is missing or invalid.")
		status_label.text = "❌ FATAL: TileMap missing."
		return

	_setup_timers()
	_setup_engine_links()
	_setup_ui()
	_connect_signals()
	
	if cameras and cameras.has_method("switch_to_camera"):
		cameras.switch_to_camera(current_camera_id)

	var tileset: TileSet = expansive_tilemap.get_tileset()
	if tileset:
		tile_size = tileset.get_tile_size()

func _setup_timers() -> void:
	engine_timer.wait_time = 1.0
	engine_timer.one_shot = false
	engine_timer.autostart = false
	# Functions must be declared later, but the parser should find them.
	engine_timer.timeout.connect(_on_engine_timer_timeout) 
	
	if is_instance_valid(clock_timer):
		clock_timer.timeout.connect(_on_clock_timer_timeout)

func _setup_engine_links() -> void:
	# Link resources with the AetherionEngine (using SSXL names)
	if aetherion_engine:
		if aetherion_signals and aetherion_engine.has_method("set_signals_node"):
			aetherion_engine.set_signals_node(aetherion_signals)
		if expansive_tilemap and aetherion_engine.has_method("set_tilemap"):
			aetherion_engine.set_tilemap(expansive_tilemap)
	else:
		push_error("❌ AetherionEngine (SSXLEngine) not found.")


func _setup_ui() -> void:
	# Set max/step values for grid dimensions
	grid_width.max_value = 9.0e15
	grid_height.max_value = 9.0e15
	grid_width.step = float(CHUNK_SIZE_X)
	grid_height.step = float(CHUNK_SIZE_Y)
	
	# Set default size (128.0)
	var default_size: float = float(CHUNK_SIZE_X * DEFAULT_CHUNKS)
	grid_width.value = default_size
	grid_height.value = default_size

	# Populate placement modes (Rust generator names)
	placement_mode_selector.clear()
	placement_mode_selector.add_item("perlin_basic_2d")
	placement_mode_selector.add_item("cellular_automata_checkerboard")
	placement_mode_selector.add_item("cellular_automata_basic")
	placement_mode_selector.add_item("cellular_automata_maze")
	placement_mode_selector.add_item("cellular_automata_solid")
	placement_mode_selector.select(0)

	# Populate tile types
	tile_type_selector.clear()
	tile_type_selector.add_item("Basic")
	tile_type_selector.add_item("Automata")
	tile_type_selector.select(0)

	# Setup progress bar
	progress_bar.min_value = 0.0
	progress_bar.max_value = 100.0
	progress_bar.value = 0.0
	progress_bar.visible = false

	tiles_placed_label.text = "Tiles Placed: 0"
	tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"
	status_label.text = "🟢 Ready to generate."


func _connect_signals() -> void:
	generate_button.pressed.connect(_on_generate_pressed)
	toggle_terminal_button.pressed.connect(_on_toggle_terminal_button_pressed) 

	if aetherion_engine and aetherion_engine.has_method("status_updated"):
		if not aetherion_engine.status_updated.is_connected(_on_engine_status_updated):
			aetherion_engine.status_updated.connect(_on_engine_status_updated)

	if aetherion_signals:
		# Build Map Start
		if aetherion_signals.has_signal("build_map_start"):
			if not aetherion_signals.build_map_start.is_connected(_on_build_map_start):
				aetherion_signals.build_map_start.connect(_on_build_map_start)
		
		# Chunk Data Ready
		if aetherion_signals.has_signal("chunk_data_ready"):
			if not aetherion_signals.chunk_data_ready.is_connected(_on_chunk_data_ready_received):
				aetherion_signals.chunk_data_ready.connect(_on_chunk_data_ready_received)
			
		# Build Map Complete
		if aetherion_signals.has_signal("build_map_complete"):
			if not aetherion_signals.build_map_complete.is_connected(_on_build_map_complete_received):
				aetherion_signals.build_map_complete.connect(_on_build_map_complete_received)
			
		# Generation Error
		if aetherion_signals.has_signal("generation_error"):
			if not aetherion_signals.generation_error.is_connected(_on_generation_error):
				aetherion_signals.generation_error.connect(_on_generation_error)

# ------------------------------------------------------------------------------
# 🎮 INPUT HANDLING
# ------------------------------------------------------------------------------
func _input(event: InputEvent) -> void:
	if event is InputEventKey and event.is_pressed() and not event.is_echo():
		if event.keycode == KEY_SPACE:
			var focused_control: Control = get_viewport().gui_get_focus_owner()
			
			if focused_control is LineEdit or focused_control is SpinBox:
				return

			if is_generating or not generate_button.disabled:
				_toggle_camera()
				get_viewport().set_input_as_handled()
				
# ------------------------------------------------------------------------------
# 🔄 ENGINE LOOP (STATUS)
# ------------------------------------------------------------------------------
func _process(_delta: float) -> void:
	if not is_generating:
		return
		
	# Update Engine Status Label
	if aetherion_engine and aetherion_engine.has_method("get_status"):
		var status: String = aetherion_engine.call("get_status")
		status_label.text = "🧠 Engine Status: %s" % status


# ------------------------------------------------------------------------------
# 🚀 GENERATION START & END
# ------------------------------------------------------------------------------
func _on_generate_pressed() -> void:
	# 🛑 GUARD
	if is_generating or not aetherion_engine or not aetherion_signals or not is_instance_valid(expansive_tilemap):
		print("⚠️ Generation guard triggered.")
		return

	# Input gathering and validation
	var width: int = int(grid_width.value)
	var height: int = int(grid_height.value)
	var generator_name: String = placement_mode_selector.get_item_text(placement_mode_selector.selected)

	# Seed handling
	var seed: int
	if seed_input.text.is_valid_int():
		seed = int(seed_input.text)
	else:
		seed = randi() % 1_000_000
		seed_input.text = str(seed)
		status_label.text = "⚠️ Invalid seed. Using random seed: %d" % seed
		
	# Check for valid map size (multiple of chunk size)
	if width % CHUNK_SIZE_X != 0 or height % CHUNK_SIZE_Y != 0 or width <= 0 or height <= 0:
		status_label.text = "⚠️ Invalid grid size. Width/Height must be a multiple of chunk size (%d)." % CHUNK_SIZE_X
		return

	# State Setup
	_clear_generation_state()
	is_generating = true
	initial_zoom_set = false

	# 🛑 FIX STATE: Set expected chunk count
	var total_chunks_x: float = float(width) / CHUNK_SIZE_X
	var total_chunks_y: float = float(height) / CHUNK_SIZE_Y
	total_chunks_expected = int(total_chunks_x * total_chunks_y)
	chunks_processed = 0
	engine_signaled_complete = false # Reset final flag

	generation_start_time_ms = Time.get_ticks_msec()
	
	# Progress bar setup
	progress_bar.max_value = float(total_chunks_expected * CHUNK_SIZE_X * CHUNK_SIZE_Y)
	progress_bar.value = 0.0
	progress_bar.visible = true
	
	status_label.text = "🧬 Generating map with mode: %s..." % [generator_name]
	generate_button.disabled = true
	engine_timer.start()

	# Clear map and prepare for drawing
	expansive_tilemap.clear()
	expansive_tilemap.emit_signal("tile_data_changed", 0, Vector2i.ZERO, Vector2i(width, height))
	await get_tree().process_frame

	# Switch camera to map view
	if cameras and cameras.has_method("switch_to_camera"):
		current_camera_id = 2
		cameras.switch_to_camera(current_camera_id)

	# FFI CALL: Initiates the asynchronous generation loop in Rust
	aetherion_engine.build_map(width, height, str(seed), generator_name)
	print("🧪 ControlPanel: build_map called with seed %d, expecting %d chunks" % [seed, total_chunks_expected])

# --- COMPLETION HANDLER ---
func _on_build_map_complete_received() -> void:
	# 🛑 FIX: The Rust engine has completed, but we may still have deferred chunk data in the queue.
	# Set the flag, but DO NOT call _on_build_map_complete() yet unless all chunks are processed.
	print("📡 Signal: build_map_complete received from Engine. Waiting for all chunks to be processed.")
	engine_signaled_complete = true
	
	# Safety check: If all chunks have been processed already, finalize now.
	if chunks_processed == total_chunks_expected:
		_on_build_map_complete()
	else:
		print("⏳ Waiting for %d more chunks to be processed before finalization." % (total_chunks_expected - chunks_processed))


func _on_build_map_complete() -> void:
	print("✅ Finalizing map generation.")
	progress_bar.visible = false
	
	# Metrics calculation
	var elapsed_placement_time_ms: int = Time.get_ticks_msec() - generation_start_time_ms
	var elapsed_placement_time_sec: float = float(elapsed_placement_time_ms) / 1000.0
	
	var final_tile_count: int = total_tiles_placed
	tiles_placed_label.text = "Tiles Placed: %d" % final_tile_count
	tile_placement_time_label.text = "⏱️ Real-World Placement Time: %.2fs" % elapsed_placement_time_sec
	engine_timer_label.text = "⏱️ Engine Tick Runtime: %.2fs | Ticks: %d" % [float(engine_tick_count), engine_tick_count]

	# Final camera positioning and zoom
	if camera2:
		var map_width_tiles: int = int(grid_width.value)
		var map_height_tiles: int = int(grid_height.value)
		
		var full_map_width: float = float(map_width_tiles) * tile_size.x
		var full_map_height: float = float(map_height_tiles) * tile_size.y

		camera2.global_position = Vector2(full_map_width / 2.0, full_map_height / 2.0)
		
		var viewport_size: Vector2 = get_viewport_rect().size
		var zoom_factor: float = min(viewport_size.x / full_map_width, viewport_size.y / full_map_height) * 0.9
		
		camera2.zoom = Vector2(clampf(zoom_factor, 0.05, 1.0), clampf(zoom_factor, 0.05, 1.0))
		initial_zoom_set = true

	# Final full map redraw
	expansive_tilemap.emit_signal("tile_data_changed", 0, Vector2i.ZERO, Vector2i(int(grid_width.value), int(grid_height.value)))
	await get_tree().process_frame

	_reset_temporary_state()
	status_label.text = "✅ Generation Complete. Map built. (%d tiles)" % final_tile_count


# ------------------------------------------------------------------------------
# 📡 SIGNAL HANDLERS
# ------------------------------------------------------------------------------
func _on_engine_status_updated(status_message: String) -> void:
	status_label.text = status_message

func _on_build_map_start() -> void:
	print("📡 Signal: build_map_start")

# --- ASYNCHRONOUS DATA RECEIVER ---
func _on_chunk_data_ready_received(chunk_dict: Dictionary) -> void:
	# Defer the tile-laying to the next idle frame.
	call_deferred("_process_chunk_data", chunk_dict)
	
	var chunk_x: int = chunk_dict.get("key_x", -1)
	var chunk_y: int = chunk_dict.get("key_y", -1)
	status_label.text = "🏗️ Chunk (%d, %d) data received, queueing for drawing..." % [chunk_x, chunk_y]


# --- DEFERRED CHUNK PROCESSING (The Asynchronous Data Application) ---
func _process_chunk_data(chunk_dict: Dictionary) -> void:
	# 1. Robustness: Check for invalid or empty data
	if not is_instance_valid(chunk_dict) or chunk_dict.is_empty():
		push_error("❌ CRITICAL: Received invalid or empty dictionary. Rust failed to send data.")
		return
		
	# Extract data
	var chunk_x: int = chunk_dict.get("key_x", -1)
	var chunk_y: int = chunk_dict.get("key_y", -1)
	var tile_array: Array = chunk_dict.get("tiles", [])

	if tile_array.is_empty():
		push_warning("⚠️ WARNING: Received empty tile array for chunk (%d, %d). Assuming this means all ID 0 tiles." % [chunk_x, chunk_y])
		pass 

	# 2. TileMap Setup Check (Existing logic)
	var layer := 0
	var source_id := 0 
	var tile_index := 0
	
	var tileset: TileSet = expansive_tilemap.get_tileset()
	var atlas_source: TileSetAtlasSource = null
	
	if tileset:
		atlas_source = tileset.get_source(source_id)
		if atlas_source == null:
			push_error("TileMap source ID %d not found in TileSet. Check TileSet configuration." % source_id)
			return
	else:
		push_error("TileMap is missing a TileSet. Cannot draw tiles.")
		return

	# 3. Iterate, Validate, and Place Tiles
	for tile_data_variant in tile_array:
		var tile_data: Dictionary = tile_data_variant
		var tile_id: int = tile_data.get("id", 0)
		
		var local_x = tile_index % CHUNK_SIZE_X
		var local_y = tile_index / CHUNK_SIZE_X
		
		var global_x = (chunk_x * CHUNK_SIZE_X) + local_x
		var global_y = (chunk_y * CHUNK_SIZE_Y) + local_y
		
		var tile_coords = Vector2i(global_x, global_y)
		var atlas_coords = Vector2i(tile_id, 0) # MAPPING: Rust ID (int) -> Atlas X-Coordinate
		
		if atlas_source.has_tile(atlas_coords):
			# Place the tile (layer, coordinates, source, atlas_coords)
			expansive_tilemap.set_cell(layer, tile_coords, source_id, atlas_coords)
		else:
			# If this fires often, the Rust generator is outputting IDs not defined in the TileSet Atlas.
			push_error("❌ TILEMAP ERROR: Invalid Tile ID **%d** from Rust for chunk (%d, %d). Cell erased." % [
				tile_id, chunk_x, chunk_y
			])
			expansive_tilemap.erase_cell(layer, tile_coords)

		tile_index += 1

	# 4. Update Progress Bar & UI
	var tiles_processed_in_chunk: int = tile_array.size()
	progress_bar.value += float(tiles_processed_in_chunk)
	total_tiles_placed += tiles_processed_in_chunk
	
	tiles_placed_label.text = "Tiles Placed: %d" % total_tiles_placed
	
	# 🛑 FIX: Increment chunk processed counter
	chunks_processed += 1 

	var percent: int = int(progress_bar.value / progress_bar.max_value * 100.0)

	# Throttled status update
	if percent != last_percent and (percent % 1 == 0 or percent == 100):
		status_label.text = "🏗️ Chunk (%d, %d) applied... %d%%" % [chunk_x, chunk_y, percent]
		
	last_percent = percent
	expansive_tilemap.queue_redraw()
	
	# 🛑 FIX: Check for completion AFTER processing. This is the new safe finalization step.
	if engine_signaled_complete and chunks_processed == total_chunks_expected:
		print("✅ Finalizing Generation: Last chunk processed and Engine completed.")
		_on_build_map_complete()
	elif chunks_processed > total_chunks_expected:
		push_error("❌ CRITICAL LOGIC ERROR: Processed more chunks than expected! Engine/ControlPanel logic mismatch.")

	
func _on_generation_error(error_message: String) -> void:
	print("❌ ERROR: Generation failed: %s" % error_message)
	_reset_temporary_state()
	status_label.text = "❌ ERROR: Generation failed. Check console."


# ------------------------------------------------------------------------------
# --- UTILITY AND MISC. ---
# ------------------------------------------------------------------------------
func _toggle_camera() -> void:
	if not cameras or not cameras.has_method("switch_to_camera"):
		return
		
	current_camera_id = 2 if current_camera_id == 1 else 1
		
	cameras.switch_to_camera(current_camera_id)
	print("Camera toggled to: %d" % current_camera_id)

func _reset_temporary_state() -> void:
	is_generating = false
	generate_button.disabled = false
	engine_timer.stop()
	engine_tick_count = 0

func _clear_generation_state() -> void:
	_reset_temporary_state()
	
	total_tiles_placed = 0
	generation_start_time_ms = 0
	
	# Reset tracking state
	total_chunks_expected = 0
	chunks_processed = 0
	engine_signaled_complete = false
	
	tiles_placed_label.text = "Tiles Placed: 0"
	tile_placement_time_label.text = "⏱️ Tile Placement Time: N/A"


func _on_clock_timer_timeout() -> void:
	if clock_label:
		clock_label.text = "🕒 " + Time.get_datetime_string_from_system()
	engine_tick_count += 1

func _on_toggle_terminal_button_pressed() -> void:
	panel_collapsed = not panel_collapsed

	# Hide/Show elements based on the panel_collapsed state
	for child in get_children():
		# Filter to only affect the UI controls we want to collapse
		if child is Control and child != toggle_terminal_button and child != status_label and child != engine_timer_label and child != tiles_placed_label and child != tile_placement_time_label:
			child.visible = not panel_collapsed
	
	# Control visibility of external references
	if clock_label:
		clock_label.visible = not panel_collapsed
	if progress_bar:
		# Keep progress bar visible if generating, even if panel is collapsed
		progress_bar.visible = not panel_collapsed and is_generating
	
	if tilemap_node:
		tilemap_node.visible = not panel_collapsed
	

func _on_engine_timer_timeout() -> void:
	var elapsed: float = engine_timer.wait_time - engine_timer.time_left
	
	if is_instance_valid(engine_timer_label):
		engine_timer_label.text = "⏱️ Runtime: %.2fs | Ticks: %d" % [elapsed, engine_tick_count]
	
	# FFI CALL: Sends a tick to the native engine
	if aetherion_engine and aetherion_engine.has_method("tick"):
		aetherion_engine.tick(engine_tick_count)
