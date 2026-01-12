extends Sprite2D

@export var conductor: Conductor

var initial_position = transform.origin

func _process(delta: float) -> void:
	var current_beat = conductor.get_current_beat()
	
	var posOffset = Vector2(cos(current_beat / 5) * 200, sin(current_beat / 7) * 150);
	
	self.transform.origin = initial_position + posOffset;
