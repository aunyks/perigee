# Codename: Perigee

Perigee is an experimental, highly portable game engine built with a focus on the web.

Interfaces (`*_interface/`) are immediately human-playable versions of the game. They use `single_player` simulations of the game, which in turn use the `core` structures throughout simulation.

Interfaces employ their own input handling to pass to the simulation. They also handle rendering the state of the simulation to the player.

## Requirements

- [Git Large File Storage](https://git-lfs.github.com/)
- [Rust](https://www.rust-lang.org/) and its Cargo package manager to build the core game simulation
- [Clippy](https://github.com/rust-lang/rust-clippy) linter for Rust
- [just](https://github.com/casey/just) task runner to make complex or long commands easier to run and remember
- [Node.js](https://nodejs.org/en/) and its [NPM](https://www.npmjs.com/) package manager for testing JavaScript WASM wrappers and miscellaneous scripting
  - [http-server](https://www.npmjs.com/package/http-server) for a basic HTTP file server. Install it globally with `npm install --global http-server`.

## Get started

Once all of the above requirements are installed, run `just` in this root directory to discover the available tasks.

```
> just
just --list
Available recipes:
    build CRATE # Build a specific crate (e.g. `just build core`)
    build-wasm  # Build single player sims to WASM. Set the RELEASE envar to anything to build in release mode
    test CRATE  # Test a specific crate (e.g. `just test core`)
    test-all    # Test everything
    test-wasm   # Test the JavaScript wrappers around the WASM sims
    prepare-web # Prepare all needed assets for immediate use by the web interface
    serve-web   # Serve the web interface
    bevy        # Start the game using the desktop Bevy interface in debug mode
```

## Licensing

This project is unlicensed. Please contact me if you'd like to use or extend this project!
