extends Node



func ready():
	pass	

func switch_to_camera(index):
	var cam1 = $/root/aetheriontester/cameras/camera1
	var cam2 = $/root/aetheriontester/cameras/camera2
	cam1.current = index == 1
	cam2.current = index == 2

func _toggle_camera() -> void:
	var camera1 := $camera1
	var camera2 := $camera2

	if camera1 == null or camera2 == null:
		print("⚠️ Cameras not found.")
		return

	camera1.enabled = !camera1.enabled
	camera2.enabled = !camera2.enabled

	if camera1.enabled and camera1.is_inside_tree():
		camera1.make_current()
	elif camera2.enabled and camera2.is_inside_tree():
		camera2.make_current()
	else:
		print("⚠️ No camera is enabled and inside the tree.")
