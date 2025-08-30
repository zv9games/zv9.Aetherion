extends Node

func ready():
	pass	

func switch_to_camera(index):
	var cam1 = $/root/aetheriontester/cameras/camera1
	var cam2 = $/root/aetheriontester/cameras/camera2
	cam1.current = index == 1
	cam2.current = index == 2
