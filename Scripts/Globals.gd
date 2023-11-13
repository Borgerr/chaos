extends Node

var last_score = 0
var high_score = 0
var new_high_score = false

func scored(score: int):
	new_high_score = false
	last_score = score
	if score > high_score:
		new_high_score = true
		high_score = score
