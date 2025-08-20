extends Node2D

@onready var echo := EchoApi.new()

func _ready():
	var path = "C:\\zv9\\zv9.aetherionengine\\rust\\target\\release\\debugger.exe"
	print("🧿 Attempting to launch debugger from GDScript: ", path)

	var result = OS.execute(path, [], [], false)
	print("🧿 OS.execute result: ", result)




	print("🌱 Main scene ready. Binding the thread...")
	print("🔍 EchoApi instance: ", echo)
	echo.init_runtime()  # ✅ This is the exposed method
	print("🔮 EchoApi runtime initialized.")

	var debug_output = echo.get_debug_output()
	print("🪶 Debug Output:\n", debug_output)
