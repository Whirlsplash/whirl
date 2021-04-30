# -------------
# | Variables |
# -------------
FLAGS = -d -t # These are just generally useful for development.

# BOTH of these variables will vary depending on your system's configuration, ideally, you should
# know the correct values for your own system.
#
# If you DON'T know the correct values, DO NOT file an issue on GitHub, ask someone within the
# Whirlsplash Discord server, https://discord.com/invite/8hn6padWF6.
WORLDS_PATH = "C:\Program Files (x86)\Worlds Inc\WorldsPlayer - Win7\WorldsPlayer.exe"
WORLDS_RUN = gsudo $(WORLDS_PATH)

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
	$(WORLDS_RUN)

runc: start_client run
