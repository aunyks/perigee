use single_player::level_0::Sim;

#[test]
fn create_sim() {
    let mut game = Sim::new();
    game.initialize().expect("Could not initialize game!");
    for _ in 0..200 {
        game.step(1.0 / 60.0);
        let player_global_translation = game.player.body_isometry() * game.player.head_isometry();
        println!("{:?}", player_global_translation);
    }
}
