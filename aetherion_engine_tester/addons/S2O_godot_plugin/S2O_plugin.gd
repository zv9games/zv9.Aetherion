@tool
extends EditorPlugin

var aetherion_instance: Node

func _enter_tree() -> void:
    print("🌀 Aetherion plugin booting...")
    ProjectSettings.set_setting("aetherion/plugin_enabled", true)

    var script_path := "res://addons/S2O_godot_plugin/Aetherion.gd"
    if ResourceLoader.exists(script_path):
        var script_res = load(script_path) # ✅ dynamic load
        if script_res:
            aetherion_instance = script_res.new()
            get_tree().root.add_child(aetherion_instance)
            if aetherion_instance.has_method("enter_idle"):
                aetherion_instance.enter_idle()
        else:
            push_error("Failed to load Aetherion script at: %s" % script_path)
    else:
        push_error("Aetherion script not found at: %s" % script_path)

func _exit_tree() -> void:
    print("🧹 Aetherion plugin dismissed.")
    ProjectSettings.set_setting("aetherion/plugin_enabled", false)
    if is_instance_valid(aetherion_instance):
        aetherion_instance.queue_free()
        aetherion_instance = null
