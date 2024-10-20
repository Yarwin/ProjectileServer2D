extends Area2D

class_name ExampleTarget

###############################################################################
# Properties                                                                 #
###############################################################################

# Signals

# Enums

# Constants

# Export Variables
@export var font_color := Color.DARK_RED
@export var font_offset: Vector2 = Vector2(1, -4)
# Private Variables
var font = preload("res://assets/fonts/Dimension X Common.ttf")

# Callables
var change_label_transform = func (new_pos, c_item):
		var new_font_t = Transform2D.IDENTITY
		new_font_t.origin = new_pos
		RenderingServer.canvas_item_set_transform(c_item, new_font_t)

var modulate_label = func (color, c_item):
	RenderingServer.canvas_item_set_modulate(c_item, color)

var remove_label = func(c_item):
	RenderingServer.canvas_item_clear(c_item)
	RenderingServer.free_rid(c_item)

# Onready Variables



###############################################################################
# Builtin functions                                                           #
###############################################################################


###############################################################################
# Public functions                                                            #
###############################################################################

func draw_damage_label(amount: float):
	var canvas = self.get_canvas()
	var canvas_item = RenderingServer.canvas_item_create()
	var pos = self.global_position + self.font_offset
	var font_t = Transform2D.IDENTITY
	font_t.origin = pos
	RenderingServer.canvas_item_reset_physics_interpolation(canvas_item)
	RenderingServer.canvas_item_set_parent(canvas_item, canvas)
	RenderingServer.canvas_item_transform_physics_interpolation(canvas_item, font_t)
	font.draw_string(
	canvas_item, 
		Vector2.ZERO, 
		str(amount), 
		0,
		-1,
		6,
		self.font_color
		)
		
	RenderingServer.canvas_item_set_z_index(canvas_item, 6)
	RenderingServer.canvas_item_set_visible(canvas_item, true)

	var tween = create_tween().parallel()
	tween.tween_method(change_label_transform.bindv([canvas_item]), pos, pos + Vector2.UP * 7 + Vector2.RIGHT * 2, 0.5).set_trans(Tween.TRANS_EXPO)
	tween.tween_method(modulate_label.bindv([canvas_item]), Color.WHITE, Color(Color.GRAY, 0.), 1.25).set_trans(Tween.TRANS_EXPO).set_delay(0.1)
	tween.chain().tween_callback(remove_label.bindv([canvas_item]))

###############################################################################
# Private functions                                                           #
###############################################################################


###############################################################################
# Connections                                                                 #
###############################################################################
