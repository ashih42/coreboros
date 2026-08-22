use macroquad::window::Conf;

use coreboros::game::Game;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::default();

    game.run().await;
}

/// Create a `Conf` for hardware and platform settings.
/// Note: Setting `sample_count` (Multi-Sample Anti-Aliasing) to a higher value makes diagonal lines look smoother.
fn window_conf() -> Conf {
    Conf {
        window_title: "Coreboros".to_owned(),
        window_width: 1280,
        window_height: 720,
        sample_count: 8,
        ..Default::default()
    }
}
