use crate::warrior_queue::WarriorQueue;

/// `SceneChange` is a message to change the current scene, produced by a dyn `Scene` and consumed by `Game`.
pub enum SceneChange {
    ToArena { warrior_queue: WarriorQueue },
    ToEditor { warrior_queue: WarriorQueue },
}

impl SceneChange {
    #[must_use]
    pub const fn to_arena(warrior_queue: WarriorQueue) -> Self {
        Self::ToArena { warrior_queue }
    }

    #[must_use]
    pub const fn to_editor(warrior_queue: WarriorQueue) -> Self {
        Self::ToEditor { warrior_queue }
    }
}
