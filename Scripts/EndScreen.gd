extends Control

func _ready():
	var globals = get_node("/root/Globals")
	$Score.text += str(globals.last_score)
	if globals.new_high_score:
		$Hiscore.text = "New High Score!"
		$HighScoreAudio.play()
	else:
		$Hiscore.text += str(globals.high_score)

func _on_restart_button_pressed():
	get_tree().change_scene_to_file("res://Scenes/TitleScreen.tscn")
