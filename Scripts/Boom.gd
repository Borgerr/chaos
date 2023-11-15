extends GPUParticles2D

@export var possible_audio: Array[AudioStream] = [
	preload("res://Audio/congratulations/fantastic.ogg"),
	preload("res://Audio/congratulations/flawless.ogg"),
	preload("res://Audio/congratulations/objectiveAchieved.ogg"),
	preload("res://Audio/congratulations/success.ogg"),
	preload("res://Audio/congratulations/untouchable.ogg"),
	preload("res://Audio/congratulations/yes.ogg")
]
# Called when the node enters the scene tree for the first time.
func _ready():
	$Audio.stream = possible_audio[randi_range(0,possible_audio.size()-1)]
	$Audio.play()
	emitting = true


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if !emitting and !$Audio.playing:
		self.queue_free()
