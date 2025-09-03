extends Node2D

func enter_idle_state():
	print("\n🎬 Main: Entering idle state. Systems standing by...")

	var oracle := get_node("AetherionOracle")
	var engine := get_node("AetherionEngine")

	if oracle and engine:
		print("🔗 Main: Linking Oracle to Engine...")
		oracle.call("set_engine", engine)

		print("🔮 Main: Oracle linked. Delivering first pulse...")
		oracle.call("tick")

		print("⚙️ Main: Engine confirmed idle.")
	else:
		push_error("❌ Main: Failed to link Oracle and Engine. Invocation aborted.")
