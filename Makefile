# -------------
# | Variables |
# -------------
FLAGS = -d -t # These are just generally useful for development.

# ------------
# | Wrappers |
# ------------
fmt:
	cargo fmt

check: fmt
	cargo check

# -------------
# | Executors |
# -------------
help:
	cargo run -- $(FLAGS) help $(ARGS)

run: check
	cargo run -- $(FLAGS) run $(ARGS)

# Subject to change depending on different PCs, this is just mine. ~Fuwn
start_client:
	gsudo "C:\Program Files (x86)\Worlds Inc\WorldsPlayer - Win7\WorldsPlayer.exe"

runc: start_client run
