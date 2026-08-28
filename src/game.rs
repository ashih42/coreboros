use crate::{
    game_context::GameContext,
    scene::{Scene, arena::Arena, editor::Editor, scene_change::SceneChange},
};

/// `Game` holds a current `scene` and persistent data in `context`.
pub struct Game {
    scene: Box<dyn Scene>,
    context: GameContext,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            scene: Box::new(Editor::default()),
            context: GameContext::default(),
        }
    }
}

impl Game {
    /// Run the game's main loop, calling `scene.update` and listening for `SceneChange` message.
    #[allow(
        clippy::future_not_send,
        reason = "This game will run as a single-threaded WASM app."
    )]
    pub async fn run(&mut self) {
        loop {
            match self.scene.update(&mut self.context) {
                Some(SceneChange::ToArena { warrior_queue }) => {
                    self.scene = Box::new(Arena::new(
                        warrior_queue,
                        self.context.config_manager.get_config(),
                    ));
                }
                Some(SceneChange::ToEditor { warrior_queue }) => {
                    self.scene = Box::new(Editor::new(warrior_queue));
                }
                None => (),
            }

            macroquad::window::next_frame().await;
        }
    }
}
