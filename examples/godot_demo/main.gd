extends Node2D
## AetherionEngine API showcase — every GDScript-callable, in order.
##
## Node tree:
##   Main (this script)
##   ├── World / Camera2D, TileMap, MultiMeshInstance2D
##   ├── AetherionEngine
##   └── UI / Status + StageLog
##
## Controls: WASD pan · wheel zoom · Space skip hold · R restart · Esc quit

const TILE := 16.0
const HOLD_SECS := 2.8
const HOLD_HEADLESS := 0.0
const LOG_MAX := 10

@onready var engine: Node = $AetherionEngine
@onready var camera: Camera2D = $World/Camera2D
@onready var tilemap: TileMap = $World/TileMap
@onready var mmi: MultiMeshInstance2D = $World/MultiMeshInstance2D
@onready var status: Label = $UI/Status
@onready var stage_log: Label = $UI/StageLog

var _phase := "boot"
var _stage_clock := 0.0
var _hold_left := 0.0
var _running_stage := false
var _last_gen_ms := 0
var _last_apply_ms := 0
var _last_tiles := 0
var _call_log: PackedStringArray = []
var _tour_running := false
var _headless := false


func _ready() -> void:
	tilemap.scale = Vector2(2, 2)
	_headless = DisplayServer.get_name() == "headless"
	await _run_tour()


func _run_tour() -> void:
	if _tour_running:
		return
	_tour_running = true
	_call_log.clear()
	_log("tour start")

	# ── 0  getters (version / ticks) ─────────────────────────────────────
	await _run_stage(
		"0/9  get_version + get_ticks",
		func () -> String:
			var ver := str(engine.get_version())
			var ticks := int(engine.get_ticks())
			_log("get_version() → %s" % ver)
			_log("get_ticks() → %s" % ticks)
			return "version=%s ticks=%s" % [ver, ticks],
		Vector2.ZERO,
		Vector2(400, 400),
		1.1,
		true
	)

	# ── 1  bind hosts ────────────────────────────────────────────────────
	await _run_stage(
		"1/9  bind_tilemap + bind_multimesh + set_prefer_multimesh(false)",
		func () -> String:
			engine.bind_multimesh(mmi)
			_log("bind_multimesh(MultiMeshInstance2D)")
			engine.bind_tilemap(tilemap)
			_log("bind_tilemap(TileMap)")
			engine.set_prefer_multimesh(false)
			_log("set_prefer_multimesh(false)  # TileMap host")
			return "hosts bound; prefer TileMap",
		Vector2.ZERO,
		Vector2(400, 400),
		1.1,
		true
	)

	# ── 2  generate_region checkerboard → TileMap ────────────────────────
	await _run_stage(
		"2/9  generate_region(mode=0 checkerboard) → TileMap",
		func () -> String:
			engine.set_prefer_multimesh(false)
			_log("set_prefer_multimesh(false)")
			var s: String = str(engine.generate_region(0, 0, 2, 2, 16, 0, 42))
			_log("generate_region(0,0, 2,2, 16, mode=0, seed=42)")
			_log_getters()
			return s,
		Vector2.ZERO,
		Vector2(32 * 16 * 2, 32 * 16 * 2),
		1.08,
		false
	)

	# ── 3  generate_region hash noise → TileMap ──────────────────────────
	await _run_stage(
		"3/9  generate_region(mode=1 hash noise) → TileMap",
		func () -> String:
			engine.set_prefer_multimesh(false)
			var s: String = str(engine.generate_region(0, 0, 3, 3, 16, 1, 99))
			_log("generate_region(0,0, 3,3, 16, mode=1, seed=99)")
			_log_getters()
			return s,
		Vector2.ZERO,
		Vector2(48 * 16 * 2, 48 * 16 * 2),
		1.08,
		false
	)

	# ── 4  generate_region_cpu (no host apply) ───────────────────────────
	await _run_stage(
		"4/9  generate_region_cpu (gen only, no draw)",
		func () -> String:
			var s: String = str(engine.generate_region_cpu(0, 0, 8, 8, 32, 1, 7))
			_log("generate_region_cpu(0,0, 8,8, 32, mode=1, seed=7)")
			_log_getters()
			return s,
		Vector2.ZERO,
		Vector2(400, 400),
		1.1,
		true
	)

	# ── 5  bench_medium → MultiMesh ──────────────────────────────────────
	await _run_stage(
		"5/9  set_prefer_multimesh(true) + bench_medium()",
		func () -> String:
			engine.set_prefer_multimesh(true)
			_log("set_prefer_multimesh(true)  # MultiMesh host")
			var s: String = str(engine.bench_medium())
			_log("bench_medium()  # 4×4 × 32² ≈ 16,384")
			_log_getters()
			return s,
		Vector2.ZERO,
		Vector2(128.0 * TILE, 128.0 * TILE),
		1.06,
		false
	)

	# ── 6  flood_million (window; headless uses smaller bench already done) ─
	if not _headless:
		await _run_stage(
			"6/9  flood_million()  # ~1,048,576 MultiMesh",
			func () -> String:
				engine.set_prefer_multimesh(true)
				var s: String = str(engine.flood_million())
				_log("flood_million()  # 16×16 × 64²")
				_log_getters()
				return s,
			Vector2.ZERO,
			Vector2(200.0 * TILE, 200.0 * TILE),
			1.05,
			false
		)
	else:
		_log("flood_million() skipped (headless smoke)")
		_phase = "6/9  flood_million() — SKIPPED (headless)"
		_refresh()

	# ── 7  flood_10m (window only — heavy) ───────────────────────────────
	if not _headless and engine.has_method("flood_10m"):
		var full := 50.0 * 64.0 * TILE
		var view := 100.0 * TILE
		var origin := Vector2((full - view) * 0.5, (full - view) * 0.5)
		await _run_stage(
			"7/9  flood_10m()  # 10,240,000 MultiMesh (heavy)",
			func () -> String:
				engine.set_prefer_multimesh(true)
				var s: String = str(engine.flood_10m())
				_log("flood_10m()  # 50×50 × 64²")
				_log_getters()
				return s,
			origin,
			Vector2(view, view),
			1.02,
			false
		)
	else:
		_log("flood_10m() skipped (headless)")
		_phase = "7/9  flood_10m() — SKIPPED (headless)"
		_refresh()

	# ── 8  bench_4m_cpu ──────────────────────────────────────────────────
	await _run_stage(
		"8/9  bench_4m_cpu()  # ~4.19M gen-only",
		func () -> String:
			var s: String = str(engine.bench_4m_cpu())
			_log("bench_4m_cpu()")
			_log_getters()
			return s,
		Vector2.ZERO,
		Vector2(400, 400),
		1.1,
		true
	)

	# ── 9  bench_10m_cpu ─────────────────────────────────────────────────
	await _run_stage(
		"9/9  bench_10m_cpu()  # ~10.24M gen-only",
		func () -> String:
			var s: String = str(engine.bench_10m_cpu())
			_log("bench_10m_cpu()")
			_log_getters()
			return s,
		Vector2.ZERO,
		Vector2(400, 400),
		1.1,
		true
	)

	_phase = "DONE — all callables exercised · R restart · WASD pan · wheel zoom"
	_log("tour complete")
	_refresh()
	_tour_running = false


func _run_stage(
	title: String,
	work: Callable,
	frame_origin: Vector2,
	frame_size: Vector2,
	margin: float,
	metrics_only: bool
) -> void:
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
	print("[showcase] ", _phase, " | ", summary)
	if not metrics_only:
		_frame_rect(frame_origin, frame_size, margin)
	_hold_left = HOLD_HEADLESS if _headless else HOLD_SECS
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
	if pan != Vector2.ZERO and camera:
		camera.position += pan.normalized() * (900.0 / maxf(camera.zoom.x, 0.01)) * delta

	_refresh()


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and not event.echo:
		match event.keycode:
			KEY_SPACE:
				_hold_left = 0.0
			KEY_R:
				if not _tour_running:
					_run_tour()
			KEY_ESCAPE:
				get_tree().quit()
	if event is InputEventMouseButton and event.pressed and camera:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			camera.zoom *= 1.12
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			camera.zoom *= 0.9
		camera.zoom = camera.zoom.clamp(Vector2(0.02, 0.02), Vector2(8, 8))


func _frame_rect(origin: Vector2, size: Vector2, margin: float = 1.1) -> void:
	if camera == null:
		return
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


func _log_getters() -> void:
	_pull_engine_metrics()
	_log("get_last_tiles() → %s" % _format_int(_last_tiles))
	_log("get_last_ms() → %s" % _last_gen_ms)
	_log("get_last_apply_ms() → %s" % _last_apply_ms)
	if engine.has_method("get_last_summary"):
		_log("get_last_summary() → %s" % str(engine.get_last_summary()))
	if engine.has_method("get_ticks"):
		_log("get_ticks() → %s" % int(engine.get_ticks()))


func _log(line: String) -> void:
	_call_log.append(line)
	while _call_log.size() > LOG_MAX:
		_call_log.remove_at(0)
	if stage_log:
		var body := ""
		for i in range(_call_log.size()):
			body += "• " + _call_log[i] + "\n"
		stage_log.text = "CALL LOG\n" + body.strip_edges()


func _refresh() -> void:
	if status == null:
		return
	var ver := "?"
	var ticks := 0
	var summary := ""
	if engine:
		if engine.has_method("get_version"):
			ver = str(engine.get_version())
		if engine.has_method("get_ticks"):
			ticks = int(engine.get_ticks())
		if engine.has_method("get_last_summary"):
			summary = str(engine.get_last_summary())
		_pull_engine_metrics()

	var timer_line := ""
	if _running_stage:
		timer_line = "STAGE TIMER  %.2f s  (running…)" % _stage_clock
	elif _hold_left > 0.0:
		timer_line = "HOLD  %.1f s  · Space skip" % _hold_left
	else:
		timer_line = "WASD pan · wheel zoom · R restart · Esc quit"

	status.text = "%s | ticks=%s\n%s\n%s\n---\nTiles: %s\nGen: %s ms\nApply (host): %s ms\nTotal (gen+apply): %s ms\n---\n%s" % [
		ver,
		ticks,
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
