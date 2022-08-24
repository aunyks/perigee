# Codename: Perigee

Perigee is an experimental, highly portable game engine built with a focus on the web.

Interfaces (`*_interface/`) are immediately human-playable versions of the game. They use `single_player` simulations of the game, which in turn use the `core` structures throughout simulation.

Interfaces employ their own input handling to pass to the simulation. They also handle rendering the state of the simulation to the player.

## Requirements

- [Git Large File Storage](https://git-lfs.github.com/)
- [Rust](https://www.rust-lang.org/) and its Cargo package manager to build the core game simulation
- [just](https://github.com/casey/just) task runner to make complex or long commands easier to run and remember
- [Deno](https://deno.land) for testing JavaScript WASM wrappers and miscellaneous scripting
- [Watchexec](https://github.com/watchexec/watchexec) for realtime development of web builds (WASM auto-recompiles on changes to Rust code).

## Get started

Once all of the above requirements are installed, run `just setup` to install some language-specific add-ons (Clippy, static file server, etc). After, run `just` to discover the available tasks.

```
> just
Available recipes:
    build CRATE         # Build a specific crate (e.g. `just build core`)
    build-wasm          # Build single player sims to WASM. Set the RELEASE envar to anything to build in release mode
    build_blender_addon # Generate a .zip file from the Blender addon code to be installed into Blender
    clippy CRATE        # Run clippy on a specific crate (e.g. `just clippy core`)
    dev-web             # Rebuild and re-serve the web interface on changes to Rust code
    format              # Format code
    lint                # Lint code
    prepare-web         # Prepare all needed assets for immediate use by the web interface
    serve-prepare-web   # Build and serve the web interface
    serve-web           # Serve the web interface
    setup               # Install language-specific add-ons
    test CRATE          # Test a specific crate (e.g. `just test core`)
    test-all            # Test everything
    test-wasm           # Test the JavaScript wrappers around the WASM sims
```

## Licensing

This project is unlicensed. Please contact me if you'd like to use or extend this project!
