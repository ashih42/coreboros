pub type WarriorId = usize;

pub trait WarriorIdDisplay {
    fn as_display_id(&self) -> usize;
}

impl WarriorIdDisplay for WarriorId {
    #[allow(clippy::arithmetic_side_effects, reason = "The number is small.")]
    fn as_display_id(&self) -> usize {
        self + 1
    }
}
