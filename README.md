# Perigee

A headless realtime 3D engine built with a focus on the web.

For a (growing) list of examples, check out [perigee.aunyks.com](https://perigee.aunyks.com).

## Why this?

I want the core logic of my video games to fit in a single [WebAssembly](https://webassembly.org/) binary so I'm free to handle input and graphics on any platform I want (desktop, web, refrigerator, console, mobile, etc) while the behavior stays the same.

This means that I can compile my game to a single, portable `.wasm` file and I (or other developers / modders) can bring the game to a new platform, change audio or visual assets, or use new input devices without needing to recompile the core game logic.

## Why not this?

If you're not interested in portability, or if you absolutely despise writing WebAssembly glue code between a Perigee binary and your runtime(s) of choice, then other engines may be better for you.

## Installation

**Rust**
Add the following to your `Cargo.toml`

```
[dependencies]
perigee = "0.1.0"
```

**Blender / Web**
Instructions coming soon

## Development Requirements

- Blender Addon (`blender_addon/`)
  - [Blender](https://www.blender.org/)
- Rust Crate (`rust/`)
  - [Rust](https://www.rust-lang.org/) and its Cargo package manager to build the core game simulation
  - [Rustfmt](https://github.com/rust-lang/rustfmt) for formatting
  - [Clippy](https://github.com/rust-lang/rust-clippy) for linting
- Web Artifacts (`web/`)
  - A web browser

## Licensing

Licensed under the MIT license.

Copyright © 2023 Gerald Nash
