use crate::{game_context::GameContext, scene::scene_change::SceneChange};

pub mod arena;
pub mod editor;
pub mod scene_change;

/// `Scene` represents game scene, which may have its own logic and rendering operations.
pub trait Scene {
    /// Perform both the logic update and rendering update.
    /// Return a `SceneChange` message if applicable.
    fn update(&mut self, game_ctx: &mut GameContext) -> Option<SceneChange>;
}
