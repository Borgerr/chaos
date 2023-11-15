extends Area2D

@onready var rotation_speed: float = randf_range(-2,2)
@onready var starting_nectar: float = randf_range(0.75,2)

@onready var nectar = starting_nectar

var got_nectar = false

var nice_boom = preload("res://Prefabs/NiceBoom.tscn")

# Called when the node enters the scene tree for the first time.
func _ready():
	$Sprite.frame = randi_range(0,53)

func get_nectar(delta) -> float:
	got_nectar = true
	var nectar_returned = starting_nectar * delta
	nectar -= nectar_returned
	if (nectar < starting_nectar / 10):
		var boom_instance = nice_boom.instantiate()
		get_parent().add_child(boom_instance)
		boom_instance.position = self.position
		self.queue_free()
		return -69
	return nectar_returned


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if got_nectar:
		$NectarParticles.emitting = true
		if $Succ.stream_paused:
			$Succ.stream_paused = false
		if !$Succ.playing:
			$Succ.play()
		got_nectar = false
	else:
		$NectarParticles.emitting = false
		if !$Succ.stream_paused:
			$Succ.stream_paused = true
	rotation += rotation_speed * delta
	scale = Vector2(nectar,nectar)
