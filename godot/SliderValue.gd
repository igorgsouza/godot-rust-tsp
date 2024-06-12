extends Control
class_name SliderValue

signal button_up(value : int)

@export
var text : String = "button"
@export
var min : float = 0
@export
var max : float = 100
@export
var default : float = 10
@export
var step : float = 1.0
var real_time : float = 0.0


@onready
var slider : HSlider = $HSlider as HSlider
@onready
var slider_label : Label = $Label as Label

func _ready() -> void:
	slider.step = step
	slider.min_value = min
	slider.max_value = max
	slider.value = default
	slider_label.text = text + str(default)

func _on_h_slider_value_changed(value: float) -> void:
	slider_label.text = text + str(value)

func get_value() -> float:
	return slider.value
