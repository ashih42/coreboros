#![cfg_attr(
    test,
    allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
)]

pub mod game;
pub mod rng;
pub mod scene;
pub mod warrior;

mod color;
mod game_context;
mod instruction;
mod mars;
mod parser;
