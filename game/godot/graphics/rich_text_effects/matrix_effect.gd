@tool
extends RichTextEffect
class_name RichTextMatrixInOut

# Syntax: [matrix clean=2.0 dirty=1.0 span=50][/matrix]

# Define the tag name.
var bbcode = "matrix"

# Gets TextServer for retrieving font information.
func get_text_server():
	return TextServerManager.get_primary_interface()

func _process_custom_fx(char_fx: CharFXTransform):
	# Get parameters, or use the provided default value if missing.
	var in_time = char_fx.env.get("in", 0.1)
	var clear_time = char_fx.env.get("clean", 0.5)
	var out_time = char_fx.env.get("out", 0.15)
	var text_delay = char_fx.env.get("text_delay", 0.01)
	
	var end_clear_time = in_time + clear_time;
	var end_time = end_clear_time + out_time;
	
	var effect_time = char_fx.elapsed_time - char_fx.range.x * text_delay
	
	if Engine.is_editor_hint():
		effect_time = fmod(effect_time, end_time + 1.0)
	
	if effect_time < 0:
		char_fx.color = Color.TRANSPARENT
	elif effect_time < in_time:
		_matrix_text(char_fx, effect_time)
	elif effect_time < end_clear_time:
		# Show the original character
		return false
	elif effect_time < end_time:
		_matrix_text(char_fx, effect_time)
		
		var char_alpha = (end_time - effect_time) / out_time
		char_fx.color.a = char_alpha
	else:
		char_fx.color = Color.TRANSPARENT
	
	return true

func _matrix_text(char_fx: CharFXTransform, matrix_time: float):
	var value = char_fx.glyph_index
	value += int(1 * matrix_time * (126 - 65))
	value %= (126 - 65)
	value += 65
	
	char_fx.glyph_index = get_text_server().font_get_glyph_index(char_fx.font, 1, value, 0)
