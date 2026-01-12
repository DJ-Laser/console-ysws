extends Button

var game_scene = preload("res://scenes/game/game.tscn")
var latency = 25;

func _ready() -> void:
	pressed.connect(_play)

func _play():
	# ABSOLUTE GOD AWFUL CODE PLS REMOVE LATER ISTG
	var this_scene = get_tree().root.get_children()[0]
	get_tree().root.add_child(game_scene.instantiate())
	var note_manager = get_tree().root.get_node("Node2D/Rhythm Logic/NoteManager")
	note_manager.input_latency_ms =  latency;
	
	get_tree().root.remove_child(this_scene)


func _on_spin_box_value_changed(value: float) -> void:
	latency = value
