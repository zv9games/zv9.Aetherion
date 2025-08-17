# 🌀 Instantiate APIBot (Node with Rust-backed ChangeOver)
var apibot = APIBot.new()
add_child(apibot)

# ⚙️ Configuration
apibot.set_grid_width(width: int)
apibot.set_grid_height(height: int)
apibot.set_flip_rate(rate: float)
apibot.set_tilemap(tilemap: TileMap)
apibot.set_layer_index(layer_index: int)
apibot.set_flipper_active(active: bool)
apibot.set_can_dissolve(enabled: bool)

# 🧭 Finalize Setup (emits "api_bot_ready" once tilemap + bot are assigned)
apibot.finalize_setup()

# 🎬 Ceremony Start
# Triggered externally when "api_bot_ready" is received
apibot.mode_start()  # Fills black, builds matrix, activates flipper

# 🫧 Ceremony Completion
# Triggered externally when ready to dissolve
apibot.mode_loaded()  # Dissolves matrix, emits "transition_finished"

# 🔁 Frame-by-frame Flip Pulse (auto-called if flipper_active)
# Internally calls:
#   bot.should_flip(delta)
#   bot.flip_random_tiles(30)
#   apibot.apply_updates(updates)

# 🧪 Manual Update Application (optional)
apibot.apply_updates(updates: Array[Dictionary])
