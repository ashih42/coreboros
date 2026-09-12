use crate::game_context::{
    config_manager::ConfigManager, renderer::Renderer, warrior_vault::WarriorVault,
};

pub mod config_manager;
pub mod renderer;
pub mod warrior_vault;

/// `GameContext` contains persistent game components that are:
/// - used in multiple scenes.
/// - saved between scene changes.
#[derive(Default)]
pub struct GameContext {
    pub config_manager: ConfigManager,
    pub renderer: Renderer,
    pub warrior_vault: WarriorVault,
}
