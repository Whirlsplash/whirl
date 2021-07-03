#!/usr/bin/env bash

# https://github.com/ogham/exa/blob/dc5c42a0f240a52f5b102cf965b98ecc2bc328c2/Vagrantfile#L55

echo ">>> Installing Rust"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal --component rustc,rust-std,cargo,clippy -y > /dev/null
source $HOME/.cargo/env
