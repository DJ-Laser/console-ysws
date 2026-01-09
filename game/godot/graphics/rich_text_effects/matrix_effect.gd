@tool
extends RichTextEffect
class_name RichTextMatrix

# Syntax: [matrix clean=2.0 dirty=1.0 span=50][/matrix]

# Define the tag name.
var bbcode = "matrix"

# Gets TextServer for retrieving font information.
func get_text_server():
	return TextServerManager.get_primary_interface()

func _process_custom_fx(char_fx):
	# Get parameters, or use the provided default value if missing.
	var dirty_time = char_fx.env.get("dirty", 0.15)
	var clear_time = char_fx.env.get("clean", 1.0)
	var text_span = char_fx.env.get("span", 20)
	
	var matrix_time = char_fx.elapsed_time - char_fx.range.x / float(text_span)
	
	var end_clear_time = dirty_time + clear_time;
	var end_dirty_time = end_clear_time + dirty_time;
	
	if Engine.is_editor_hint():
		matrix_time = fmod(matrix_time, end_dirty_time + 1.0)
	
	if matrix_time < 0 || matrix_time > end_dirty_time:
		char_fx.color = Color.TRANSPARENT
	elif matrix_time < dirty_time || matrix_time > end_clear_time:
		var value = char_fx.glyph_index
		value += int(1 * matrix_time * (126 - 65))
		value %= (126 - 65)
		value += 65
		
		char_fx.glyph_index = get_text_server().font_get_glyph_index(char_fx.font, 1, value, 0)
	
	return true
