extends Control
class_name Path

@export
var line : Line2D = null

var _disappearing : bool = false
var _evaporation_rate : float = 0.0
var eval : float = INF

func create_path(points : PackedVector2Array, path : PackedInt64Array) -> void:
	if points.is_empty():
		return
	for index in path:
		line.add_point(points[index])

func _process(delta: float) -> void:
	if not _disappearing:
		return
	self.modulate.a -= delta*_evaporation_rate
	if self.modulate.a <= 0.0: 
		queue_free()
	pass

func destroy_path(evaporation: float) -> void:
	_disappearing = true
	_evaporation_rate = evaporation
	pass
