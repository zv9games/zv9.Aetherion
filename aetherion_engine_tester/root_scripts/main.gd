# In main.gd

extends Node2D

@onready var echo = null

func _ready():
	print("🌱 Main scene ready. Awaiting plugin glyph...")

	echo = EchoApi.new()
	add_child(echo)
	print("✔️ add_child called. Is EchoApi valid? ", is_instance_valid(echo))

	if is_instance_valid(echo):
		echo.debug_info_received.connect(_on_debug_info_received)
		print("📢 Connected to 'debug_info_received' signal.")

	load_aetherion()

	if is_plugin_enabled():
		summon_aetherion()
	else:
		print("🔒 Plugin disabled. Aetherion sleeps.")

func load_aetherion():
	print("📦 Aetherion chipspace secured.")

func summon_aetherion():
	print("🔓 Plugin enabled. Summoning Aetherion...")
	
	echo.init_runtime()
	print("🔮 EchoApi runtime initialized.")
	
	echo.get_debug_output()
	print("📡 Requested initial debug output. Waiting for signal...")

func is_plugin_enabled() -> bool:
	return ProjectSettings.get_setting("aetherion/plugin_enabled", false)

func _on_debug_info_received(info: String):
	print("🪶 Live Debug Output:\n", info)

func _process(delta):
	# Make sure the echo object is valid before trying to use it
	if not is_instance_valid(echo):
		print("❌ EchoApi is no longer a valid instance. Program will exit.")
		return

	print("tick")

	if echo.is_ready():
		echo.advance_tick() 

		if Engine.get_frames_drawn() % 60 == 0:
			echo.get_debug_output()
