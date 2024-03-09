run:
	cargo run --features bevy/dynamic_linking
watch:
	cargo watch -q -c -x 'run --features bevy/dynamic_linking'