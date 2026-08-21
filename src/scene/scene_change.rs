use crate::warrior_queue::WarriorQueue;

pub enum SceneChange {
    ToArena { warrior_queue: WarriorQueue },
    ToEditor { warrior_queue: WarriorQueue },
}

impl SceneChange {
    pub fn to_arena(warrior_queue: WarriorQueue) -> Self {
        Self::ToArena { warrior_queue }
    }

    pub fn to_editor(warrior_queue: WarriorQueue) -> Self {
        Self::ToEditor { warrior_queue }
    }
}
