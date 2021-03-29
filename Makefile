FLAGS = -d -v # These are just generally useful for development.

help:
	cargo run -- $(FLAGS) help

run:
	cargo run -- $(FLAGS) run

config:
	cargo run -- $(FLAGS) config
