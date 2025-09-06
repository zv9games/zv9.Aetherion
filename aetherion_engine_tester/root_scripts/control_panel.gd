extends Control

# 🧭 UI Node References
@onready var grid_width: SpinBox = $gridwidthspinbox
@onready var grid_height: SpinBox = $gridheightspinbox
@onready var seed_input: LineEdit = $seedlineedit
@onready var placement_mode_selector: OptionButton = $placementoptionbutton
@onready var tile_type_selector: OptionButton = $tiletypeoptionbutton
@onready var animate_checkbox: CheckBox = $animatecheckbox
@onready var status_label: Label = $billboard
@onready var generate_button: Button = $ignition
@onready var progress_bar: ProgressBar = $progressbar
@onready var toggle_terminal_button: Button = $toggleterminalbutton
@onready var engine_timer: Timer = $enginetimer
@onready var engine_timer_label: Label = $enginetimerlabel

# 🧠 External Scene References
@onready var main := get_node("../")
@onready var aetherion_engine := main.get_node("AetherionEngine")
@onready var aetherion_signals := main.get_node("AetherionSignals")
@onready var expansive_tilemap := main.get_node("expansive_tilemap")
@onready var camera_tilemap := main.get_node("cameras/camera2")
@onready var clock_label := main.get_node("tilemap/clocklabel")
@onready var clock_timer := main.get_node("tilemap/clocktimer")
@onready var tilemap := main.get_node("tilemap")
@onready var cameras := main.get_node("cameras")

# 📊 State Variables
var last_percent := -1
var tile_size := Vector2(16, 16)
var panel_collapsed := false
var engine_tick_count := 0

# 🧭 Boot Sequence
func _ready() -> void:
	engine_timer_label.text = "✅ Label is alive"

	clock_timer.timeout.connect(_on_clock_timer_timeout)
	engine_timer.timeout.connect(_on_engine_timer_timeout)
	engine_timer.wait_time = 1.0
	engine_timer.one_shot = false
	engine_timer.autostart = false

	_setup_ui()
	_connect_signals()

	var tileset: TileSet = expansive_tilemap.get_tileset()

	if tileset:
		tile_size = tileset.get_tile_size()

# 🧩 UI Setup
func _setup_ui() -> void:
	grid_width.max_value = 1_000_000_000
	grid_height.max_value = 1_000_000_000
	grid_width.step = 1
	grid_height.step = 1
	grid_width.value = 10
	grid_height.value = 10

	placement_mode_selector.clear()
	placement_mode_selector.add_item("Noise")
	placement_mode_selector.add_item("Checkerboard")
	placement_mode_selector.add_item("Clustered")
	placement_mode_selector.select(0)

	tile_type_selector.clear()
	tile_type_selector.add_item("Basic")
	tile_type_selector.add_item("Automata")
	tile_type_selector.select(0)

	progress_bar.min_value = 0
	progress_bar.max_value = 100
	progress_bar.value = 0
	progress_bar.visible = false

	status_label.text = "🟢 Ready to generate."

# 🔗 Signal Wiring
func _connect_signals() -> void:
	generate_button.pressed.connect(_on_generate_pressed)
	toggle_terminal_button.pressed.connect(_on_toggle_terminal_button_pressed)

	if not aetherion_signals.build_map_start.is_connected(_on_build_map_start):
		aetherion_signals.build_map_start.connect(_on_build_map_start)
	if not aetherion_signals.map_building_status.is_connected(_on_map_building_status):
		aetherion_signals.map_building_status.connect(_on_map_building_status)
	if not aetherion_signals.generation_progress.is_connected(_on_generation_progress):
		aetherion_signals.generation_progress.connect(_on_generation_progress)
	if not aetherion_signals.generation_complete.is_connected(_on_generation_complete):
		aetherion_signals.generation_complete.connect(_on_generation_complete)

# 🔄 Engine Status Poll
func _process(_delta: float) -> void:
	if aetherion_engine.has_method("get_status"):
		var status: String = aetherion_engine.call("get_status")

		status_label.text = "🧠 Engine Status: %s" % status

# 🚀 Generation Trigger
func _on_generate_pressed() -> void:
	var width := int(grid_width.value)
	var height := int(grid_height.value)
	var seed_text := seed_input.text
	var animate := animate_checkbox.button_pressed
	var mode := tile_type_selector.get_item_text(tile_type_selector.selected).to_lower()
	var placement := placement_mode_selector.get_item_text(placement_mode_selector.selected).to_lower()

	if width <= 0 or height <= 0 or width * height > 1_000_000_000:
		status_label.text = "⚠️ Invalid grid size. Must be positive and total tiles must not exceed 1 billion."
		return

	var seed := int(seed_text) if seed_text.is_valid_int() else randi() % 1_000_000

	if not seed_text.is_valid_int():
		seed_input.text = str(seed)
		status_label.text = "⚠️ Invalid seed. Using random seed: %d" % seed

	progress_bar.value = 0
	progress_bar.visible = true
	last_percent = -1
	status_label.text = "🧬 Generating map with mode: %s, animate: %s..." % [mode, str(animate)]
	generate_button.disabled = true
	engine_timer.start()

	expansive_tilemap.clear()
	camera_tilemap.enabled = true
	camera_tilemap.make_current()
	camera_tilemap.zoom = Vector2(1.0, 1.0)

	aetherion_engine.call("set_signals_node", aetherion_signals)
	aetherion_engine.call("set_tilemap", expansive_tilemap)
	aetherion_engine.call("build_map", width, height, seed, mode, animate, Vector2i(0, 0), Vector2i(1, 0))

# 📡 Signal Handlers
func _on_build_map_start() -> void:
	status_label.text = "🚀 Map generation started..."

func _on_map_building_status(status_message: String) -> void:
	status_label.text = "📢 %s" % status_message

func _on_generation_progress(percent: int) -> void:
	progress_bar.value = percent
	last_percent = percent
	expansive_tilemap.force_update(0)

func _on_generation_complete(results: Dictionary) -> void:
	progress_bar.visible = false
	generate_button.disabled = false
	engine_timer.stop()

	var width: int = results.get("width", 0)
	var height: int = results.get("height", 0)
	var mode: String = results.get("mode", "unknown")
	var animate: bool = results.get("animate", false)
	var duration: float = results.get("duration", 0.0)

	var elapsed := engine_timer.wait_time - engine_timer.time_left

	status_label.text = "✅ Map generation complete: %dx%d, mode: %s, animate: %s\n⏱️ Duration: %.2fs (%.2fs elapsed)" % [
		width, height, mode, str(animate), duration, elapsed
	]

	engine_timer_label.text = "⏱️ Final Runtime: %.2fs" % elapsed

	camera_tilemap.global_position = Vector2(width * tile_size.x / 2, height * tile_size.y / 2)
	camera_tilemap.zoom = Vector2(1.0 / max(width / 10.0, 1.0), 1.0 / max(height / 10.0, 1.0))
	expansive_tilemap.force_update(0)

# 🕒 Clock Update
func _on_clock_timer_timeout() -> void:
	clock_label.text = "🕒 " + Time.get_datetime_string_from_system()
	engine_tick_count += 1
	engine_timer_label.text = "⏱️ Engine Ticks: %d" % engine_tick_count

# 🪄 Terminal Toggle
func _on_toggle_terminal_button_pressed() -> void:
	panel_collapsed = !panel_collapsed

	for child in get_children():
		if child != toggle_terminal_button and child is Control:
			child.visible = not panel_collapsed

	clock_label.visible = not panel_collapsed
	progress_bar.visible = not panel_collapsed
	tilemap.visible = not panel_collapsed

	cameras.call("_toggle_camera")

# ⏱️ Engine Timer Tick
func _on_engine_timer_timeout() -> void:
	var elapsed := engine_timer.wait_time - engine_timer.time_left
	engine_timer_label.text = "⏱️ Engine Runtime: %.2fs" % elapsed
