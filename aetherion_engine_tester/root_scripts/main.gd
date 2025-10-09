extends Node2D

@onready var clock_timer: Timer = $/root/aetheriontester/main/tilemap/clocktimer
@onready var engine_monitor: Node = $/root/aetheriontester/main/AetherionEngine/EngineMonitor

func _ready() -> void:
	clock_timer.timeout.connect(_on_clock_tick)
	clock_timer.start()

func enter_idle_state() -> void:
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	var oracle := get_node("AetherionOracle")
	var engine := get_node("AetherionEngine")
	var signals := get_node("AetherionSignals")

	if oracle and engine:
		print("🔗 Linking Oracle to Engine...")
		oracle.call("set_engine", engine)

		print("📡 Linking EngineMonitor to Engine...")
		if engine_monitor:
			engine_monitor.call("set_engine", engine)

			if signals:
				print("📶 Connecting Engine to AetherionSignals...")
				engine.call("set_signals_node", signals)
				signals.connect("map_building_status", Callable(engine_monitor, "_on_map_building_status"))
				print("✅ EngineMonitor connected to status signal.")
			else:
				push_warning("⚠️ AetherionSignals node not found. Signal connection skipped.")

		print("🔮 Oracle linked. Delivering first pulse...")
		oracle.call("tick")

		print("⚙️ Engine confirmed idle.")
	else:
		push_error("❌ Failed to link Oracle and Engine. Invocation aborted.")

func _on_clock_tick() -> void:
	print("🕰️ Clock tick.")

	var oracle := get_node("AetherionOracle")
	if oracle:
		oracle.call("tick")

	if engine_monitor:
		engine_monitor.call("update_status")
