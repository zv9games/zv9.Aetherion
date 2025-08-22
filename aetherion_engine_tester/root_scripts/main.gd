extends Node2D

@onready var echo := EchoApi.new()

func _ready():
	print("🌱 Main scene ready. Awaiting plugin glyph...")
	load_aetherion()

	if is_plugin_enabled():
		summon_aetherion()
	else:
		print("🔒 Plugin disabled. Aetherion sleeps.")

func load_aetherion():
	# Optional: preload assets, bind overlays, or prepare idle perch
	print("📦 Aetherion chipspace secured.")

func summon_aetherion():
	print("🔓 Plugin enabled. Summoning Aetherion...")

	var path = "C:\\zv9\\zv9.aetherionengine\\rust\\target\\release\\debugger.exe"
	print("🧿 Attempting to launch debugger from GDScript: ", path)

	var output := []
	var result := OS.execute(path, [], output, false)
	print("🧿 OS.execute result: ", result)
	print("🪶 Debugger output:\n", output)



	print("🧿 OS.execute result: ", result)

	echo.init_runtime()
	print("🔮 EchoApi runtime initialized.")

	var debug_output = echo.get_debug_output()
	print("🪶 Debug Output:\n", debug_output)

func is_plugin_enabled() -> bool:
	return ProjectSettings.get_setting("aetherion/plugin_enabled", false)

func _process(delta):
	if echo.is_ready():
		echo.advance_tick()  # 🫁 Breath binding — advances Animate + Tick
		echo.generate_tiles()  # 🧩 Optional — triggers Generate phase

		var debug_output = echo.get_debug_output()
		if debug_output != "":
			print("🪶 Live Debug Output:\n", debug_output)
