extends Control
class_name PointManager

var point_scene : PackedScene = load("res://point.tscn")

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		event = event as InputEventMouseButton
		if event.is_released() and event.button_index == MOUSE_BUTTON_LEFT:
			_generate_point(event.position)

func _generate_point(position: Vector2) -> void:
	var point : Point = point_scene.instantiate() as Point
	add_child(point)
	point.position = position
	point.add_to_group("points")

func generate_random_points(quantity: int) -> void:
	var max_coord : Vector2 = size*.95
	var min_coord : Vector2 = size*.05
	var rng : RandomNumberGenerator = RandomNumberGenerator.new()
	for _i in range(quantity):
		var position = Vector2(rng.randi_range(min_coord[0], max_coord[0]), rng.randi_range(min_coord[1], max_coord[1]))
		_generate_point(position)
		
func get_point_coordinates() -> PackedVector2Array:
	var packedVector2Array = PackedVector2Array()
	for point in get_children():
		packedVector2Array.append((point as Point).global_position)
	return packedVector2Array

func clear_points() -> void:
	get_tree().call_group("points", "queue_free")
