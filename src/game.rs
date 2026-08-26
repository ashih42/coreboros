use crate::{
    config_manager::ConfigManager,
    renderer::Renderer,
    scene::{Scene, arena::Arena, editor::Editor, scene_change::SceneChange},
    warrior_queue::WarriorQueue,
    warrior_vault::WarriorVault,
};

pub struct GameContext {
    pub config_manager: ConfigManager,
    pub renderer: Renderer,
    pub warrior_vault: WarriorVault,
}

pub struct Game {
    scene: Box<dyn Scene>,
    context: GameContext,
}

impl Default for Game {
    fn default() -> Self {
        let scene = Editor::new(WarriorQueue::default());

        Self {
            scene: Box::new(scene),
            context: GameContext {
                config_manager: ConfigManager::default(),
                renderer: Renderer::default(),
                warrior_vault: WarriorVault::default(),
            },
        }
    }
}

impl Game {
    #[allow(
        clippy::future_not_send,
        reason = "This game will run as a single-threaded WASM app."
    )]
    pub async fn run(&mut self) {
        loop {
            match self.scene.update(&mut self.context) {
                Some(SceneChange::ToArena { warrior_queue }) => {
                    let config = self.context.config_manager.get_config();
                    let arena = Arena::new(warrior_queue, config);
                    self.scene = Box::new(arena);
                }
                Some(SceneChange::ToEditor { warrior_queue }) => {
                    let editor = Editor::new(warrior_queue);
                    self.scene = Box::new(editor);
                }
                None => (),
            }

            macroquad::window::next_frame().await;
        }
    }
}
