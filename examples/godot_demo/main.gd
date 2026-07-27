extends Node2D

## MultiMesh tile size in world units (must match host_multimesh TILE_WORLD_SIZE).
const TILE := 16.0
## How long each completed stage stays on screen so times are readable.
const HOLD_SECS := 3.5

@onready var engine: Node = $AetherionEngine
@onready var tilemap: TileMap = $TileMap
@onready var mmi: MultiMeshInstance2D = $MultiMeshInstance2D
@onready var label: Label = $UI/Status
@onready var camera: Camera2D = $Camera2D

var _phase := "boot"
var _stage_clock := 0.0
var _hold_left := 0.0
var _last_gen_ms := 0
var _last_apply_ms := 0
var _last_tiles := 0
var _running_stage := false

func _ready() -> void:
	tilemap.scale = Vector2(2, 2)

	if engine:
		if engine.has_method("bind_multimesh"):
			engine.bind_multimesh(mmi)
		if engine.has_method("bind_tilemap"):
			engine.bind_tilemap(tilemap)
		if engine.has_method("set_prefer_multimesh"):
			engine.set_prefer_multimesh(false)

	await get_tree().process_frame
	var headless := DisplayServer.get_name() == "headless"

	# --- Stage 1: TileMap (visible grid) ---
	await _run_stage(
		"1/3 TileMap — 1,024 tiles (2×2 × 16²)",
		func ():
			if engine.has_method("set_prefer_multimesh"):
				engine.set_prefer_multimesh(false)
			# re-run small region onto TileMap (auto-smoke already ran; show deliberate timed pass)
			return engine.generate_region(0, 0, 2, 2, 16, 0, 42),
		Vector2.ZERO,
		Vector2(32 * 16 * 2, 32 * 16 * 2),
		1.08
	)

	# --- Stage 2: MultiMesh medium ---
	await _run_stage(
		"2/3 MultiMesh medium — 16,384 tiles (4×4 × 32²)",
		func ():
			if engine.has_method("set_prefer_multimesh"):
				engine.set_prefer_multimesh(true)
			return engine.bench_medium(),
		Vector2.ZERO,
		Vector2(128.0 * TILE, 128.0 * TILE),
		1.06
	)

	# --- Stage 3: 10M MultiMesh (window only) ---
	if not headless and engine and engine.has_method("flood_10m"):
		# 50×50 × 64² = 10_240_000 tiles spanning 3200×3200 cells
		var full := 50.0 * 64.0 * TILE
		var view := 100.0 * TILE
		var origin := Vector2((full - view) * 0.5, (full - view) * 0.5)
		await _run_stage(
			"3/3 MultiMesh BIG — 10,240,000 tiles (50×50 × 64²)",
			func ():
				if engine.has_method("set_prefer_multimesh"):
					engine.set_prefer_multimesh(true)
				return engine.flood_10m(),
			origin,
			Vector2(view, view),
			1.02
		)
		_phase = "DONE — pan/zoom to explore (WASD + wheel)"
	else:
		_phase = "DONE (headless skipped 10M host apply)"
	_refresh()

func _run_stage(title: String, work: Callable, frame_origin: Vector2, frame_size: Vector2, margin: float) -> void:
	_phase = title + " — RUNNING…"
	_stage_clock = 0.0
	_running_stage = true
	_refresh()
	await get_tree().process_frame

	var t0 := Time.get_ticks_msec()
	var summary: String = str(work.call())
	var wall_ms := Time.get_ticks_msec() - t0
	_running_stage = false

	_pull_engine_metrics()
	_phase = "%s — DONE in %.2f s wall" % [title, wall_ms / 1000.0]
	print("[demo] ", _phase, " | ", summary)
	_frame_rect(frame_origin, frame_size, margin)
	_hold_left = HOLD_SECS
	_refresh()

	while _hold_left > 0.0:
		await get_tree().process_frame

func _process(delta: float) -> void:
	if _running_stage:
		_stage_clock += delta
	if _hold_left > 0.0:
		_hold_left = maxf(0.0, _hold_left - delta)

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
		camera.position += pan.normalized() * (900.0 / maxf(camera.zoom.x, 0.01)) * delta

	_refresh()

func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.pressed:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			camera.zoom *= 1.12
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			camera.zoom *= 0.9
		camera.zoom = camera.zoom.clamp(Vector2(0.02, 0.02), Vector2(8, 8))

func _frame_rect(origin: Vector2, size: Vector2, margin: float = 1.1) -> void:
	camera.position = origin + size * 0.5
	var vp := get_viewport_rect().size
	if size.x <= 1.0 or size.y <= 1.0 or vp.x <= 1.0:
		return
	var z := minf(vp.x / (size.x * margin), vp.y / (size.y * margin))
	camera.zoom = Vector2(z, z)

func _pull_engine_metrics() -> void:
	if engine == null:
		return
	if engine.has_method("get_last_ms"):
		_last_gen_ms = int(engine.get_last_ms())
	if engine.has_method("get_last_apply_ms"):
		_last_apply_ms = int(engine.get_last_apply_ms())
	if engine.has_method("get_last_tiles"):
		_last_tiles = int(engine.get_last_tiles())

func _refresh() -> void:
	if label == null:
		return
	var ver := ""
	var summary := ""
	if engine:
		if engine.has_method("get_version"):
			ver = str(engine.get_version())
		if engine.has_method("get_last_summary"):
			summary = str(engine.get_last_summary())
		_pull_engine_metrics()

	var timer_line := ""
	if _running_stage:
		timer_line = "STAGE TIMER  %.2f s  (running…)" % _stage_clock
	elif _hold_left > 0.0:
		timer_line = "HOLD  %.1f s  (read the times)" % _hold_left
	else:
		timer_line = "TIMER idle — pan WASD / zoom wheel"

	label.text = "%s\n%s\n%s\n---\nTiles: %s\nGen: %s ms\nApply (host): %s ms\nTotal (gen+apply): %s ms\n---\n%s" % [
		ver,
		_phase,
		timer_line,
		_format_int(_last_tiles),
		_last_gen_ms,
		_last_apply_ms,
		_last_gen_ms + _last_apply_ms,
		summary,
	]

func _format_int(n: int) -> String:
	var s := str(n)
	var out := ""
	var c := 0
	for i in range(s.length() - 1, -1, -1):
		if c > 0 and c % 3 == 0:
			out = "," + out
		out = s[i] + out
		c += 1
	return out
