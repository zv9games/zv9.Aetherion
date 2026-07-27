extends Node2D

## MultiMesh tile size in world units (must match host_multimesh TILE_WORLD_SIZE).
const TILE := 16.0

@onready var engine: Node = $AetherionEngine
@onready var tilemap: TileMap = $TileMap
@onready var mmi: MultiMeshInstance2D = $MultiMeshInstance2D
@onready var label: Label = $UI/Status
@onready var camera: Camera2D = $Camera2D

var _phase := "boot"

func _ready() -> void:
	# TileMap default tile size is often 16px already; scale up so small floods read well.
	tilemap.scale = Vector2(2, 2)

	if engine:
		if engine.has_method("bind_multimesh"):
			engine.bind_multimesh(mmi)
		if engine.has_method("bind_tilemap"):
			engine.bind_tilemap(tilemap)
		if engine.has_method("set_prefer_multimesh"):
			engine.set_prefer_multimesh(false) # start on TileMap so first flood is clear

	await get_tree().process_frame
	var headless := DisplayServer.get_name() == "headless"

	# --- Phase 1: small TileMap checkerboard (easy to see) ---
	_phase = "TileMap smoke (32×32 tiles)"
	_frame_tiles(32, 32, 2.0) # tilemap scaled ×2 → world ~64×64 tiles * 16? TileMap uses pixels
	# TileMap with scale 2: cell is ~32 screen px at zoom 1 if tile is 16 — frame cell grid
	_frame_rect(Vector2.ZERO, Vector2(32 * 16 * 2, 32 * 16 * 2), 1.05)
	_refresh()
	await get_tree().create_timer(1.2).timeout

	# --- Phase 2: medium MultiMesh (128×128 cells @ 16u) ---
	if engine and engine.has_method("set_prefer_multimesh"):
		engine.set_prefer_multimesh(true)
	if engine and engine.has_method("bench_medium"):
		_phase = "MultiMesh medium (~16k tiles)"
		print("[demo] bench_medium => ", engine.bench_medium())
		# 4×4 chunks of 32 = 128×128 tiles
		_frame_rect(Vector2.ZERO, Vector2(128.0 * TILE, 128.0 * TILE), 1.08)
	_refresh()
	await get_tree().create_timer(2.0).timeout

	# --- Phase 3: million MultiMesh — frame a readable window into the field ---
	if not headless and engine and engine.has_method("flood_million"):
		_phase = "MultiMesh flood (~1M tiles) — viewing center patch"
		print("[demo] flood_million => ", engine.flood_million())
		# 16×16 chunks of 64 = 1024×1024 tiles; show ~80×80 tile window at center
		var full := 1024.0 * TILE
		var view := 90.0 * TILE
		var origin := Vector2((full - view) * 0.5, (full - view) * 0.5)
		_frame_rect(origin, Vector2(view, view), 1.02)
	_refresh()

func _process(_delta: float) -> void:
	# Arrow keys / WASD pan; wheel zoom — so you can explore the flood.
	var pan := Vector2.ZERO
	if Input.is_key_pressed(KEY_A) or Input.is_key_pressed(KEY_LEFT):
		pan.x -= 1.0
	if Input.is_key_pressed(KEY_D) or Input.is_key_pressed(KEY_RIGHT):
		pan.x += 1.0
	if Input.is_key_pressed(KEY_W) or Input.is_key_pressed(KEY_UP):
		pan.y -= 1.0
	if Input.is_key_pressed(KEY_S) or Input.is_key_pressed(KEY_DOWN):
		pan.y += 1.0
	if pan != Vector2.ZERO:
		camera.position += pan.normalized() * (900.0 / maxf(camera.zoom.x, 0.01)) * _delta

	if engine and label and engine.has_method("get_ticks"):
		if int(engine.get_ticks()) % 30 == 0:
			_refresh()

func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			camera.zoom *= 1.12
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			camera.zoom *= 0.9
		camera.zoom = camera.zoom.clamp(Vector2(0.02, 0.02), Vector2(8, 8))

## Fit camera so `size` world rect at `origin` fills the viewport (with margin).
func _frame_rect(origin: Vector2, size: Vector2, margin: float = 1.1) -> void:
	camera.position = origin + size * 0.5
	var vp := get_viewport_rect().size
	if size.x <= 1.0 or size.y <= 1.0 or vp.x <= 1.0:
		return
	var zx := vp.x / (size.x * margin)
	var zy := vp.y / (size.y * margin)
	var z := minf(zx, zy)
	camera.zoom = Vector2(z, z)

func _frame_tiles(tiles_x: float, tiles_y: float, cell_px: float) -> void:
	_frame_rect(Vector2.ZERO, Vector2(tiles_x * cell_px, tiles_y * cell_px), 1.1)

func _refresh() -> void:
	if engine == null or label == null:
		return
	var ver := str(engine.get_version()) if engine.has_method("get_version") else ""
	var summary := str(engine.get_last_summary()) if engine.has_method("get_last_summary") else ""
	var apply_ms := str(engine.get_last_apply_ms()) if engine.has_method("get_last_apply_ms") else "?"
	label.text = "%s\nPhase: %s\n%s\napply_ms=%s | pan: WASD/arrows | zoom: mouse wheel" % [
		ver, _phase, summary, apply_ms
	]
