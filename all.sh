#!/bin/bash

echo Running all days

find ./day* -maxdepth 0 -type d \( ! -name . \) -exec bash -c "cd '{}' && RUSTFLAGS=\"$RUSTFLAGS -A warnings\" cargo run --release -q" \;

