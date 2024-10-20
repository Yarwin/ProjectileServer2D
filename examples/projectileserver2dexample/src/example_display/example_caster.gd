extends CharacterBody2D


###############################################################################
# Properties                                                                 #
###############################################################################

# Signals

# Enums

# Constants

# Export Variables
### Straight shoot example with rect.
@export var projectile_config_rect: ProjectileConfig
### Curved shoot example with texture
@export var projectile_config_texture: ProjectileConfig
### Sinusoid shoot example.
@export var projectile_config_sinusoid: ProjectileConfig
### Sinusoid shoot example.
@export var projectile_config_cosinoid: ProjectileConfig

# Private Variables
var is_projectile_perpendicular = false

# Onready Variables
@onready var rotation_node := %RotationNode
@onready var muzzle := %Muzzle
@onready var curved_muzzle := %CurvedMuzzle
@onready var current_muzzle := self.muzzle
@onready var current_projectile_config: ProjectileConfig = self.projectile_config_rect

###############################################################################
# Builtin functions                                                           #
###############################################################################

func _physics_process(_delta: float) -> void:
	rotation_node.look_at(get_global_mouse_position())

func _unhandled_input(event: InputEvent) -> void:
	if (event is InputEventMouseButton 
	and event.is_pressed() 
	and not event.is_echo() 
	and (event as InputEventMouseButton).button_index == MOUSE_BUTTON_LEFT):
		self.spawn_new_projectile()


###############################################################################
# Public functions                                                            #
###############################################################################

func spawn_new_projectile():
	var canvas: RID = self.get_world_2d().canvas
	var space: RID = self.get_world_2d().space
	var projectile_transform: Transform2D = self.current_muzzle.get_global_transform()
	if is_projectile_perpendicular:
		projectile_transform = projectile_transform.rotated_local(PI / 2.)
	var exclude: Array[RID] = [self.get_rid()]

	ProjectileServer2D.spawn_new_projectile(
		current_projectile_config,
		canvas,
		space,
		projectile_transform,
		exclude,
		self
	)
	if current_projectile_config == self.projectile_config_sinusoid:
		ProjectileServer2D.spawn_new_projectile(
		self.projectile_config_cosinoid,
		canvas,
		space,
		projectile_transform,
		exclude,
		self
	)

###############################################################################
# Private functions                                                           #
###############################################################################


###############################################################################
# Connections                                                                 #
###############################################################################


func _on_display_rect_button_pressed() -> void:
	current_projectile_config = self.projectile_config_rect
	current_muzzle = self.muzzle
	is_projectile_perpendicular = false


func _on_display_curved_button_pressed() -> void:
	current_projectile_config = self.projectile_config_texture
	current_muzzle = self.curved_muzzle
	is_projectile_perpendicular = true

func _on_display_sine_button_pressed() -> void:
	current_projectile_config = self.projectile_config_sinusoid
	current_muzzle = self.muzzle
	is_projectile_perpendicular = false
