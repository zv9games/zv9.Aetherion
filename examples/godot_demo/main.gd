extends Node2D

@onready var engine: Node = $AetherionEngine
@onready var tilemap: TileMap = $TileMap
@onready var mmi: MultiMeshInstance2D = $MultiMeshInstance2D
@onready var label: Label = $UI/Status
@onready var camera: Camera2D = $Camera2D

func _ready() -> void:
	if engine:
		if engine.has_method("bind_multimesh"):
			engine.bind_multimesh(mmi)
		if engine.has_method("bind_tilemap"):
			engine.bind_tilemap(tilemap)
		# Prefer MultiMesh for large floods; small auto-smoke still uses TileMap if preferred false.
		if engine.has_method("set_prefer_multimesh"):
			engine.set_prefer_multimesh(true)
	await get_tree().process_frame
	_refresh()
	if engine and engine.has_method("bench_medium"):
		print("[demo] bench_medium => ", engine.bench_medium())
	_refresh()
	# Million-tile MultiMesh flood only with a window (skip headless smoke).
	var headless := DisplayServer.get_name() == "headless"
	if not headless and engine and engine.has_method("flood_million"):
		print("[demo] flood_million => ", engine.flood_million())
		camera.position = Vector2(512, 512)
		camera.zoom = Vector2(0.15, 0.15)
	_refresh()

func _process(_delta: float) -> void:
	if engine and label and engine.has_method("get_ticks"):
		if int(engine.get_ticks()) % 60 == 0:
			_refresh()

func _refresh() -> void:
	if engine == null or label == null:
		return
	var ver := str(engine.get_version()) if engine.has_method("get_version") else ""
	var summary := str(engine.get_last_summary()) if engine.has_method("get_last_summary") else ""
	var apply_ms := str(engine.get_last_apply_ms()) if engine.has_method("get_last_apply_ms") else "?"
	label.text = "%s\n%s\napply_ms=%s ticks=%s" % [
		ver, summary, apply_ms,
		engine.get_ticks() if engine.has_method("get_ticks") else "?"
	]
