extends Control
class_name SliderButton

signal button_up(value : int)

@export
var text : String = "button"
@export
var min : int = 0
@export
var max : int = 100
@export
var default : int = 10


@onready
var button : Button = $Button as Button
@onready
var slider : HSlider = $HSlider as HSlider
@onready
var slider_label : Label = $Label as Label

func _ready() -> void:
	slider.min_value = min
	slider.max_value = max
	slider.value = default
	button.text = text

func _on_h_slider_value_changed(value: float) -> void:
	slider_label.text = str(value)


func _on_button_button_up() -> void:
	button_up.emit(slider.value)
