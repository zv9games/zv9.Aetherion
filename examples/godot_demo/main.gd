extends Node2D

@onready var engine: Node = $AetherionEngine
@onready var tilemap: TileMap = $TileMap
@onready var label: Label = $UI/Status

func _ready() -> void:
	if engine and engine.has_method("bind_tilemap"):
		engine.bind_tilemap(tilemap)
	await get_tree().process_frame
	_refresh()
	if engine and engine.has_method("bench_medium"):
		var s: String = engine.bench_medium()
		print("[demo] bench_medium => ", s)
	_refresh()

func _process(_delta: float) -> void:
	if engine and label and engine.has_method("get_ticks"):
		var ticks: int = int(engine.get_ticks())
		if ticks % 45 == 0:
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
