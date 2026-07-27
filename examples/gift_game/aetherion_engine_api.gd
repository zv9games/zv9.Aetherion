extends AetherionEngine
## ═══════════════════════════════════════════════════════════════════════════
##  AetherionEngine API bible (gift script — attached to this node by menu g)
## ═══════════════════════════════════════════════════════════════════════════
##
## Every GDScript-callable on AetherionEngine is used below. Read this file,
## watch the Output dock + on-screen HUD, then remix for your game.
##
## Callables:
##   get_version() get_ticks()
##   get_last_tiles() get_last_ms() get_last_apply_ms() get_last_summary()
##   bind_tilemap(TileMap)  bind_multimesh(MultiMeshInstance2D)
##   set_prefer_multimesh(bool)
##   generate_region(ox, oy, cx, cy, size, mode, seed)   # mode 0=checker 1=noise
##   generate_region_cpu(...)                            # gen only, no draw
##   bench_medium()  flood_million()  flood_10m()
##   bench_4m_cpu()  bench_10m_cpu()
##
## Controls (while running): Space skip hold · 1 run light tour · 2 medium · 3 heavy
## ═══════════════════════════════════════════════════════════════════════════

const TILE := 16.0
const HOLD := 2.2

var _label: Label
var _camera: Camera2D
var _tilemap: TileMap
var _mmi: MultiMeshInstance2D
var _phase := "boot"
var _hold := 0.0
var _busy := false
var _log: PackedStringArray = []


func _ready() -> void:
	_ensure_scene_helpers()
	print("")
	print("╔══════════════════════════════════════════════════════════╗")
	print("║  AetherionEngine API bible — script on this node         ║")
	print("╚══════════════════════════════════════════════════════════╝")
	print("  get_version() → ", get_version())
	print("  get_ticks()   → ", get_ticks())
	print("  Edit res://aetherion_engine_api.gd — this is your tutorial.")
	print("")
	# Small delay so the first auto-smoke from Rust finishes first.
	await get_tree().create_timer(0.15).timeout
	await run_api_tour(false)


func _process(delta: float) -> void:
	if _hold > 0.0:
		_hold = maxf(0.0, _hold - delta)
	_refresh_hud()


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed and not event.echo:
		match event.keycode:
			KEY_SPACE:
				_hold = 0.0
			KEY_1:
				if not _busy:
					run_api_tour(false)
			KEY_2:
				if not _busy:
					run_api_tour(true)
			KEY_3:
				if not _busy:
					_run_heavy_only()
			KEY_ESCAPE:
				get_tree().quit()
	if event is InputEventMouseButton and event.pressed and _camera:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			_camera.zoom *= 1.12
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			_camera.zoom *= 0.9
		_camera.zoom = _camera.zoom.clamp(Vector2(0.05, 0.05), Vector2(6, 6))


## Full API tour. If with_floods: include flood_million (still skips 10M unless key 3).
func run_api_tour(with_floods: bool = false) -> void:
	if _busy:
		return
	_busy = true
	_log.clear()

	# ── getters ──────────────────────────────────────────────────────────
	await _step("get_version / get_ticks", func ():
		_note("get_version() → %s" % get_version())
		_note("get_ticks() → %s" % get_ticks())
		return "inspect OK"
	)

	# ── bind hosts ───────────────────────────────────────────────────────
	await _step("bind_tilemap + bind_multimesh + set_prefer_multimesh", func ():
		bind_multimesh(_mmi)
		_note("bind_multimesh(MultiMeshInstance2D)")
		bind_tilemap(_tilemap)
		_note("bind_tilemap(TileMap)")
		set_prefer_multimesh(false)
		_note("set_prefer_multimesh(false)  # draw to TileMap")
		return "hosts bound"
	)

	# ── generate_region → TileMap ────────────────────────────────────────
	await _step("generate_region(mode=0 checkerboard)", func ():
		set_prefer_multimesh(false)
		var s: String = str(generate_region(0, 0, 2, 2, 16, 0, 42))
		_note("generate_region(0,0, 2,2, 16, mode=0, seed=42)")
		_note_metrics()
		_frame_tiles(2, 2, 16, true)
		return s
	)

	await _step("generate_region(mode=1 hash noise)", func ():
		set_prefer_multimesh(false)
		var s: String = str(generate_region(0, 0, 3, 3, 16, 1, 99))
		_note("generate_region(0,0, 3,3, 16, mode=1, seed=99)")
		_note_metrics()
		_frame_tiles(3, 3, 16, true)
		return s
	)

	# ── CPU only ─────────────────────────────────────────────────────────
	await _step("generate_region_cpu (no draw)", func ():
		var s: String = str(generate_region_cpu(0, 0, 4, 4, 32, 1, 7))
		_note("generate_region_cpu(0,0, 4,4, 32, mode=1, seed=7)")
		_note_metrics()
		return s
	)

	# ── MultiMesh presets ────────────────────────────────────────────────
	await _step("bench_medium()", func ():
		set_prefer_multimesh(true)
		_note("set_prefer_multimesh(true)")
		var s: String = str(bench_medium())
		_note("bench_medium()  # ~16k MultiMesh")
		_note_metrics()
		_frame_world(128.0 * TILE, 128.0 * TILE)
		return s
	)

	if with_floods:
		await _step("flood_million()", func ():
			set_prefer_multimesh(true)
			var s: String = str(flood_million())
			_note("flood_million()  # ~1M MultiMesh")
			_note_metrics()
			_frame_world(200.0 * TILE, 200.0 * TILE)
			return s
		)
	else:
		_note("flood_million() / flood_10m() — press 2 for million, 3 for 10M")

	# ── CPU benches ──────────────────────────────────────────────────────
	await _step("bench_4m_cpu()", func ():
		var s: String = str(bench_4m_cpu())
		_note("bench_4m_cpu()  # ~4.19M gen-only")
		_note_metrics()
		return s
	)

	await _step("bench_10m_cpu()", func ():
		var s: String = str(bench_10m_cpu())
		_note("bench_10m_cpu()  # ~10.24M gen-only")
		_note_metrics()
		return s
	)

	_phase = "DONE — edit aetherion_engine_api.gd · 1 light · 2 +1M · 3 +10M"
	_note("tour complete — all core callables exercised")
	print("[AetherionEngine API] ", _phase)
	_busy = false


func _run_heavy_only() -> void:
	if _busy:
		return
	_busy = true
	_ensure_scene_helpers()
	bind_multimesh(_mmi)
	bind_tilemap(_tilemap)
	set_prefer_multimesh(true)
	await _step("flood_10m() HEAVY", func ():
		var s: String = str(flood_10m())
		_note("flood_10m()  # 10,240,000 MultiMesh — needs RAM")
		_note_metrics()
		var full := 50.0 * 64.0 * TILE
		var view := 100.0 * TILE
		if _camera:
			_camera.position = Vector2(full, full) * 0.5
			var vp: Vector2 = get_viewport().get_visible_rect().size
			var z := minf(vp.x / (view * 1.05), vp.y / (view * 1.05))
			_camera.zoom = Vector2(z, z)
		return s
	)
	_phase = "DONE heavy — 1 light tour · 2 million · 3 again"
	_busy = false


func _step(title: String, work: Callable) -> void:
	_phase = title + " — RUNNING…"
	_refresh_hud()
	await get_tree().process_frame
	var t0 := Time.get_ticks_msec()
	var summary: String = str(work.call())
	var wall := Time.get_ticks_msec() - t0
	_phase = "%s — DONE %.2fs | %s" % [title, wall / 1000.0, summary]
	print("[AetherionEngine API] ", _phase)
	_hold = HOLD
	_refresh_hud()
	while _hold > 0.0:
		await get_tree().process_frame


func _note(line: String) -> void:
	_log.append(line)
	while _log.size() > 12:
		_log.remove_at(0)
	print("  · ", line)


func _note_metrics() -> void:
	_note("get_last_tiles() → %s" % get_last_tiles())
	_note("get_last_ms() → %s" % get_last_ms())
	_note("get_last_apply_ms() → %s" % get_last_apply_ms())
	_note("get_last_summary() → %s" % get_last_summary())
	_note("get_ticks() → %s" % get_ticks())


func _refresh_hud() -> void:
	if _label == null:
		return
	var hold_s := ""
	if _hold > 0.0:
		hold_s = "HOLD %.1fs · Space skip" % _hold
	else:
		hold_s = "1 light tour · 2 +flood_million · 3 flood_10m · Esc quit"
	var body := ""
	for i in range(_log.size()):
		body += "• " + _log[i] + "\n"
	_label.text = "%s | ticks=%s\n%s\n%s\n---\nTiles %s | gen %s ms | apply %s ms\n---\n%s" % [
		get_version(),
		get_ticks(),
		_phase,
		hold_s,
		get_last_tiles(),
		get_last_ms(),
		get_last_apply_ms(),
		body.strip_edges(),
	]


func _frame_tiles(cx: int, cy: int, csize: int, scaled_tilemap: bool) -> void:
	var scale := 2.0 if scaled_tilemap else 1.0
	var w := float(cx * csize) * 16.0 * scale
	var h := float(cy * csize) * 16.0 * scale
	_frame_world(w, h)


func _frame_world(w: float, h: float) -> void:
	if _camera == null or w < 1.0 or h < 1.0:
		return
	_camera.position = Vector2(w, h) * 0.5
	# AetherionEngine is a Node (not CanvasItem) — no get_viewport_rect().
	var vp: Vector2 = get_viewport().get_visible_rect().size
	var z := minf(vp.x / (w * 1.1), vp.y / (h * 1.1))
	_camera.zoom = Vector2(z, z)


## Create TileMap / MultiMesh / Camera / HUD under Main if the gift scene is bare.
func _ensure_scene_helpers() -> void:
	var main := get_parent()
	if main == null:
		main = self

	_tilemap = main.get_node_or_null("TileMap") as TileMap
	if _tilemap == null:
		_tilemap = TileMap.new()
		_tilemap.name = "TileMap"
		main.add_child(_tilemap)
		_tilemap.owner = main
	_tilemap.scale = Vector2(2, 2)

	_mmi = main.get_node_or_null("MultiMeshInstance2D") as MultiMeshInstance2D
	if _mmi == null:
		_mmi = MultiMeshInstance2D.new()
		_mmi.name = "MultiMeshInstance2D"
		main.add_child(_mmi)
		_mmi.owner = main

	_camera = main.get_node_or_null("Camera2D") as Camera2D
	if _camera == null:
		_camera = Camera2D.new()
		_camera.name = "Camera2D"
		main.add_child(_camera)
		_camera.owner = main
		_camera.make_current()

	var ui := main.get_node_or_null("UI") as CanvasLayer
	if ui == null:
		ui = CanvasLayer.new()
		ui.name = "UI"
		ui.layer = 10
		main.add_child(ui)
		ui.owner = main

	_label = ui.get_node_or_null("Status") as Label
	if _label == null:
		var panel := ColorRect.new()
		panel.name = "Panel"
		panel.color = Color(0.04, 0.05, 0.09, 0.9)
		panel.offset_left = 10
		panel.offset_top = 10
		panel.offset_right = 720
		panel.offset_bottom = 340
		ui.add_child(panel)

		_label = Label.new()
		_label.name = "Status"
		_label.offset_left = 20
		_label.offset_top = 18
		_label.offset_right = 700
		_label.offset_bottom = 320
		_label.add_theme_font_size_override("font_size", 16)
		_label.add_theme_color_override("font_color", Color(0.95, 0.97, 1))
		_label.add_theme_color_override("font_outline_color", Color(0, 0, 0))
		_label.add_theme_constant_override("outline_size", 4)
		ui.add_child(_label)
