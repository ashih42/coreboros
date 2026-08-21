use macroquad::window::Conf;

use coreboros::game::Game;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::default();

    game.run().await;
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Coreboros".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}
