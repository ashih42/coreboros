use crate::warrior::warrior_id::WarriorId;

/// Find the appropriate order to draw the effects of warriors such that the current-turn warrior is drawn last,
/// so his effects are drawn on top of those of other warriors.
///
/// Example: In a 4 player game, if it is currently player 2's turn, then the rendering order is [3, 0, 1, 2].
#[inline]
pub fn generate_warrior_rendering_order(
    num_warriors: usize,
    current_warrior_id: WarriorId,
) -> impl DoubleEndedIterator<Item = WarriorId> {
    #[allow(clippy::arithmetic_side_effects, reason = "This operation is valid 👌")]
    (0..num_warriors).map(move |i| {
        let index = (current_warrior_id.0 + i + 1) % num_warriors;
        WarriorId(index)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_warrior_rendering_order() {
        assert!(generate_warrior_rendering_order(4, WarriorId(0)).eq([1, 2, 3, 0].map(WarriorId)));
        assert!(generate_warrior_rendering_order(4, WarriorId(1)).eq([2, 3, 0, 1].map(WarriorId)));
        assert!(generate_warrior_rendering_order(4, WarriorId(2)).eq([3, 0, 1, 2].map(WarriorId)));
        assert!(generate_warrior_rendering_order(4, WarriorId(3)).eq([0, 1, 2, 3].map(WarriorId)));

        assert!(generate_warrior_rendering_order(2, WarriorId(0)).eq([1, 0].map(WarriorId)));
        assert!(generate_warrior_rendering_order(2, WarriorId(1)).eq([0, 1].map(WarriorId)));

        assert!(generate_warrior_rendering_order(1, WarriorId(0)).eq([0].map(WarriorId)));
    }
}
