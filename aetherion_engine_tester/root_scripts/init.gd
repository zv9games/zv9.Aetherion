extends Node

func initialize(scenemap: Dictionary, main: Node):
	#print("🧠 logicinitializer starting...")
	
	for node_name in scenemap.keys():
		var node_path = scenemap[node_name]
		var node = get_node(node_path)
		if node:
			#print("✅ node ready: %s at %s" % [node_name, node_path])
			_autowire(node_name, node, main)
		else:
			print("⚠️ node missing: %s" % node_name)

	print("🚀 all nodes online. aetherion system is ready.")


func _autowire(name: String, node: Node, main: Node):
	match name:
		"ignition":
			if node.has_signal("pressed"):
				node.pressed.connect(main._on_ignition_pressed)
		"seedlineedit":
			if node.has_method("set_text"):
				node.set_text("ready")
		"clocktimer":
			node.start()
		_:
			# Optional: log or ignore
			pass
