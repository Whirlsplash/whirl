flags = -d -v

help:
	cargo run -- $(flags) help

run:
	cargo run -- $(flags) run

config:
	cargo run -- $(flags) config
