extends Control
class_name TravellingSalesmanSolver

signal done

var _ant_colony_interface : EvolutionManager = EvolutionManager.new()
var _path_scene : PackedScene = load("res://path.tscn")

var _best_path : Path = null
var _best_eval : float = INF
var _current_path : Path = null
var _points : PackedVector2Array = []

var _selection : String = "Ant Colony"
var _real_simulation_time : float = 1.0

var _is_simulating : bool = false

@onready
var ant_config : Array = [
	$VBoxContainer/Alpha,
	$VBoxContainer/Beta,
	$VBoxContainer/PheromonePower,
	$VBoxContainer/AntQuantity,
	$VBoxContainer/Evaporation,
]

@onready
var gene_config : Array = [
	$VBoxContainer/Mutation,
	$VBoxContainer/PoolSize
]

@onready
var perm_config : Array = [
	$VBoxContainer/MaxTime
]

@onready
var options : OptionButton = $VBoxContainer/OptionButton
@onready
var replay_time : SliderValue = $VBoxContainer/ReplayTime
@onready
var best_label : Label = $Best
@onready
var best_iter : Label = $Iter
@onready
var best_time : Label = $Time
var iter : int = 0


func _ready() -> void:
	_on_option_button_item_selected(0)

func _on_run_button_button_up() -> void:
	iter = 0
	if _best_path != null:
		_best_path.modulate = Color.INDIAN_RED
	_points = ($PointManager as PointManager).get_point_coordinates()
	
	if len(_points) < 3:
		return
	
	var results : Array[Dictionary] = []
	var time_start = Time.get_ticks_msec()
	match _selection:
		"Ant Colony":
			results = _ant_colony_interface.ant_colony(_points, {
				"max_iter": int(($VBoxContainer/MaxIter as SliderValue).get_value()),
				"alpha": ($VBoxContainer/Alpha as SliderValue).get_value(),
				"beta": ($VBoxContainer/Beta as SliderValue).get_value(),
				"pheromone_power": ($VBoxContainer/PheromonePower as SliderValue).get_value(),
				"ant_quantity": int(($VBoxContainer/AntQuantity as SliderValue).get_value()),
				"evaporation": ($VBoxContainer/Evaporation as SliderValue).get_value(),
			})
		"Genetic":
			results = _ant_colony_interface.genetic(_points, {
				"max_iter": int(($VBoxContainer/MaxIter as SliderValue).get_value()),
				"mutate_chance": ($VBoxContainer/Mutation as SliderValue).get_value(),
				"pool_size": int(($VBoxContainer/PoolSize as SliderValue).get_value()),
			})
		"Permutation":
			results = _ant_colony_interface.permutation(_points, {
				"max_iter": int(($VBoxContainer/MaxIter as SliderValue).get_value()),
				"max_time": int(($VBoxContainer/MaxTime as SliderValue).get_value()*1000),
			})
	_real_simulation_time = (Time.get_ticks_msec()-time_start)/1000.0
	best_time.text = "Time: %.2f" % _real_simulation_time
	
	var simulation_time : float = replay_time.get_value() / (results.size())
	if simulation_time == 0:
		simulation_time = _real_simulation_time / (results.size())
		
	
	if _is_simulating:
		_is_simulating = false
		await self.done
	
	_is_simulating = true
	for result in results:
		if !_is_simulating:
			break
		show_path(result, 1/simulation_time)
		await get_tree().create_timer(simulation_time).timeout
	if _current_path != null:
		_current_path.destroy_path(1/simulation_time)
	_is_simulating = false
	done.emit()
	if _best_path != null:
		_best_path.modulate = Color.LAWN_GREEN
	
func show_path(arr : Dictionary, evap_time : float) -> void:
	iter += 1
	var new_path : Path = _path_scene.instantiate()
	add_child(new_path)
	new_path.create_path(_points, arr["path"])
	if _current_path != null:
		_current_path.destroy_path(evap_time)
	_current_path = new_path
	if arr["eval"] < _best_eval:
		_best_eval = arr["eval"]
		best_label.text = "Best: %.2f" % _best_eval
		
		if _best_path != null:
			_best_path.queue_free()
		_best_path = _path_scene.instantiate() as Path
		add_child(_best_path)
		best_iter.text = "Iter: %d" % iter
		_best_path.create_path(_points, arr["path"])
		_best_path.modulate = Color.INDIAN_RED
		_best_path.modulate.a = 0.5
		_best_path.z_index = 10
		

func _on_generate_button_button_up(value : int) -> void:
	_on_button_clear_button_up()
	($PointManager as PointManager).generate_random_points(value)

func _on_button_clear_button_up() -> void:
	if _best_path != null:
		_best_path.queue_free()
	if _current_path != null:
		_current_path.queue_free()
	_best_eval = INF
	iter = 0
	best_label.text = "Best: %.2f" % INF
	best_time.text = "Time: %.2f" % INF
	best_iter.text = "Iter: %d" % 0
	_points.clear()
	if _is_simulating:
		_is_simulating = false
		await self.done 
	($PointManager as PointManager).clear_points()


func _on_option_button_item_selected(index: int) -> void:
	_selection = options.get_item_text(index)
	var aco : bool = false
	var gen : bool = false
	var perm: bool = false
	match _selection:
		"Ant Colony":
			aco = true
		"Genetic":
			gen = true
		"Permutation":
			perm = true
	for option in ant_config:
		option.visible = aco
	for option in gene_config:
		option.visible = gen
	for option in perm_config:
		option.visible = perm
			
