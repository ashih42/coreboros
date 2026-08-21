use crate::{game::GameContext, scene::scene_change::SceneChange};

pub mod arena;
pub mod editor;
pub mod scene_change;

pub trait Scene {
    fn update(&mut self, game_ctx: &mut GameContext) -> Option<SceneChange>;
}
