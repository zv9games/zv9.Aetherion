# Main.gd
# This script orchestrates the GDExtension test.

extends Node2D

@onready var aetherion_engine: AetherionEngine = $"/root/main/AetherionEngine"
@onready var aetherion_signals: AetherionSignals = $"/root/main/AetherionSignals"
@onready var expansive_tilemap: TileMap = $"/root/main/Expansive_TileMap"

func _ready() -> void:
	await get_tree().process_frame
	
	print("🟢 Main.gd is ready. Testing Aetherion GDExtension...")

	aetherion_engine.set_signals_node(aetherion_signals)

	aetherion_signals.connect("build_map_start", Callable(self, "_on_build_map_start"))
	aetherion_signals.connect("map_building_status", Callable(self, "_on_map_building_status"))
	aetherion_signals.connect("generation_complete", Callable(self, "_on_generation_complete"))

	print("🟢 Signals connected. The engine is ready.")

	var black := Vector2i(14, 13)
	var blue := Vector2i(15, 13)
	aetherion_engine.bulk_place_tiles(expansive_tilemap, black, blue)



# Signal handler for when the Rust pipeline begins.
func _on_build_map_start() -> void:
	print("🧬 GDExtension pipeline has started. Loading and processing map data...")

# Signal handler for progress updates.
func _on_map_building_status(status_message: String) -> void:
	print("📢 GDExtension status: ", status_message)

# Signal handler for when the Rust pipeline finishes.
func _on_generation_complete(results: Dictionary) -> void:
	print("✅ GDExtension pipeline has finished placing all tiles.")
	print("Results: ", results)
