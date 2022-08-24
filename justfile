alias fmt := format
alias f := format

alias c := clippy
alias l := lint

alias b := build
alias bwasm := build-wasm
alias pw := prepare-web

alias t := test
alias ta := test-all
alias twasm := test-wasm

alias sw := serve-web
alias spw := serve-prepare-web
alias dw := dev-web

internal_js_folders := "scripts single_player/wasm_js_wrappers"

default:
  just --list

# Install language-specific add-ons
setup:
  rustup component add rustfmt
  rustup component add clippy

# Format code
format:
  cargo fmt
  deno fmt {{internal_js_folders}}

# Run clippy on a specific crate (e.g. `just clippy core`)
clippy CRATE:
  cargo clippy -p {{CRATE}}

# Lint code
lint:
  cargo clippy
  deno lint {{internal_js_folders}}

# Build a specific crate (e.g. `just build core`)
build CRATE:
  cargo build -p {{CRATE}}

# Test a specific crate (e.g. `just test core`)
test CRATE:
  cargo test -p {{CRATE}}

# Test everything
test-all: test-wasm
  cargo test

# Generate a .zip file from the Blender addon code to be installed into Blender
build_blender_addon:
  pushd ./assets/blender/addons && zip -r PerigeeEngineAddon.zip ./PerigeeEngineAddon; popd

# Build single player sims to WASM. Set the RELEASE envar to anything to build in release mode
build-wasm:
  deno run \
  --allow-read --allow-env \
  --allow-run --allow-write \
  --unstable ./scripts/build-wasm-levels.js

# Test the JavaScript wrappers around the WASM sims
test-wasm: build-wasm
  deno test --allow-read --allow-env ./single_player/wasm_js_wrappers

# Prepare all needed assets for immediate use by the web interface
prepare-web: build-wasm
  deno run \
  --allow-read --allow-env --allow-write \
  ./scripts/prepare-web-artifacts.js

# Serve the web interface
serve-web:
  deno run --allow-read --allow-net --allow-write  \
  scripts/dev_server.ts ./static_web_interface -p 3000

# Build and serve the web interface
serve-prepare-web: prepare-web serve-web

# Rebuild and re-serve the web interface on changes to Rust code
dev-web:
  watchexec -r -e rs -- just serve-prepare-web