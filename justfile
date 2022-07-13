alias c := clippy
alias b := build
alias bw := build-wasm
alias pw := prepare-web
alias sw := serve-web
alias spw := serve-prepare-web
alias t := test
alias ta := test-all
alias tw := test-wasm

default:
  just --list

# Run clippy on a specific crate (e.g. `just clippy core`)
clippy CRATE:
  cargo clippy -p {{CRATE}}

# Build a specific crate (e.g. `just build core`)
build CRATE:
  cargo build -p {{CRATE}}

# Test a specific crate (e.g. `just test core`)
test CRATE:
  cargo test -p {{CRATE}}

# Test everything
test-all: test-wasm
  cargo test

# Start the game using the Bevy interface in debug mode
bevy:
  # Copy assets from the file 
  node scripts/copy-assets-to-bevy-interface.js
  cargo run -p bevy_interface

# Build single player sims to WASM. Set the RELEASE envar to anything to build in release mode
build-wasm:
  node scripts/build-wasm-levels.js

# Test the JavaScript wrappers around the WASM sims
test-wasm: build-wasm
  npm i --prefix ./single_player_ffi/wasm_js_wrappers && npm test --prefix ./single_player_ffi/wasm_js_wrappers

# Prepare all needed assets for immediate use by the web interface
prepare-web: build-wasm
  node scripts/prepare-web-artifacts.js

# Serve the web interface
serve-web:
  http-server ./web_interface -p 3000

# Build and serve the web interface
serve-prepare-web: prepare-web serve-web