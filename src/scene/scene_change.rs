use crate::warrior::Warrior;

/// `SceneChange` is a message to change the current scene, produced by a dyn `Scene` and consumed by `Game`.
pub enum SceneChange {
    ToArena { warriors: Box<[Warrior]> },
    ToEditor { warriors: Box<[Warrior]> },
}

impl SceneChange {
    #[must_use]
    pub const fn to_arena(warriors: Box<[Warrior]>) -> Self {
        Self::ToArena { warriors }
    }

    #[must_use]
    pub const fn to_editor(warriors: Box<[Warrior]>) -> Self {
        Self::ToEditor { warriors }
    }
}
