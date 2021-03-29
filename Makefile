flags = -d -v # These are just generally useful for development.

help:
	cargo run -- $(flags) help

run:
	cargo run -- $(flags) run

config:
	cargo run -- $(flags) config
