extends Node2D

func ready():
	pass
	
func enter_idle_state():
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	var oracle := get_node("AetherionOracle")
	var engine := get_node("AetherionEngine")

	if oracle and engine:
		print("🔗 Main: Linking Oracle to Engine...")
		oracle.call("set_engine", engine)

		print("📡 Main: Linking EngineMonitor to Engine...")
		if engine_monitor:
			engine_monitor.call("set_engine", engine)

		print("🔮 Main: Oracle linked. Delivering first pulse...")
		oracle.call("tick")

		print("⚙️ Main: Engine confirmed idle.")
	else:
		push_error("❌ Main: Failed to link Oracle and Engine. Invocation aborted.")


@onready var clock_timer := $/root/aetheriontester/main/tilemap/clocktimer
@onready var engine_monitor := $/root/aetheriontester/main/AetherionEngine/EngineMonitor

func _ready():
	clock_timer.connect("timeout", Callable(self, "_on_clock_tick"))
	clock_timer.start()

func _on_clock_tick():
	print("🕰️ Clock tick.")
	
	var oracle := get_node("AetherionOracle")
	if oracle:
		oracle.call("tick")

	if engine_monitor:
		engine_monitor.call("update_status")
