#!/usr/bin/env bash

terser "$TRUNK_STAGING_DIR"/rustdoku-game.js -c -m --module --output "$TRUNK_STAGING_DIR"/rustdoku-game.js

gzip --best --keep "$TRUNK_STAGING_DIR"/*{.js,.wasm}
brotli --best --keep "$TRUNK_STAGING_DIR"/*{.js,.wasm}
