FLAGS = -d -t # These are just generally useful for development.

help:
	cargo run -- $(FLAGS) help $(ARGS)

run:
	cargo run -- $(FLAGS) run $(ARGS)

config:
	cargo run -- $(FLAGS) config $(ARGS)
