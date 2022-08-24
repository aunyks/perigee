# single_player

Simulation structures for single player gameplay levels. These structures should be used by interface layers as the core gameplay logic: interface layers should simply provide these structures with input coming from the player, step the structures, and then extract simulation data from them to be rendered to the player. These depend on structures from `core`.

Run crate-specific commands with `cargo <cmd> -p single_player`. For example, run `cargo test -p single_player` to run the test suite.
