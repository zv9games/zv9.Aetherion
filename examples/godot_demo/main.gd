extends Node

@onready var engine: Node = $AetherionEngine
@onready var label: Label = $UI/Status

func _ready() -> void:
	await get_tree().process_frame
	_refresh()
	# Second pass after auto-smoke
	if engine.has_method("bench_medium"):
		var s: String = engine.bench_medium()
		print("[demo] bench_medium => ", s)
	_refresh()

func _process(_delta: float) -> void:
	if engine and label and engine.has_method("get_ticks"):
		var ticks: int = int(engine.get_ticks())
		if ticks % 30 == 0:
			_refresh()

func _refresh() -> void:
	if engine == null or label == null:
		return
	var ver := ""
	var summary := ""
	if engine.has_method("get_version"):
		ver = str(engine.get_version())
	if engine.has_method("get_last_summary"):
		summary = str(engine.get_last_summary())
	label.text = "%s\n%s\nticks=%s" % [ver, summary, engine.get_ticks() if engine.has_method("get_ticks") else "?"]
