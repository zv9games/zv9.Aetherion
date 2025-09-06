extends Node

var engine_ref
var status := "Unknown"

func set_engine(engine):
	engine_ref = engine

func update_status():
	if engine_ref:
		status = engine_ref.call("get_status") # Assuming engine has this method
		print("📡 EngineMonitor: Status - %s" % status)
	else:
		push_error("🚨 EngineMonitor: No engine linked.")
