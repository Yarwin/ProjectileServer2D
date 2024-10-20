@tool
extends EditorPlugin


func _enter_tree():
	add_autoload_singleton("ProjectileServer2D", "res://addons/projectile_server2d/ProjectileServer2D.tscn")

func _exit_tree():
	pass
