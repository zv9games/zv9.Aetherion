@tool
extends EditorPlugin

var aetherion_instance

func _enter_tree():
    # 🔓 Plugin enabled — draw the glyph
    print("🌀 Aetherion plugin booting...")

    # 🪶 Set plugin flag for runtime gate
    ProjectSettings.set_setting("aetherion/plugin_enabled", true)

    # 🧩 Mount the engine node
    aetherion_instance = preload("res://addons/aetherion_plugin/Aetherion.gd").new()
    get_tree().get_root().add_child(aetherion_instance)
    aetherion_instance.enter_idle()

func _exit_tree():
    # 🔒 Plugin disabled — seal the temple
    print("🧹 Aetherion plugin dismissed.")

    # 🪶 Clear plugin flag
    ProjectSettings.set_setting("aetherion/plugin_enabled", false)

    # 🧼 Remove engine node
    if aetherion_instance:
        aetherion_instance.queue_free()
