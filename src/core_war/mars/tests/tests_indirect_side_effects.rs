use indoc::indoc;

use crate::{
    core_war::{config::Config, core_number::CoreNumber, mars::Mars},
    warrior::{Warrior, warrior_id::WarriorId},
};

#[test]
fn test_a_indirect_pre_decrement() {
    let mut mars = Mars::new(
        &[Warrior::from_text(indoc! {"
                nop {10  ; AIndirectPreDecrement
            "})
        .unwrap()],
        &Config::default(),
    );
    let warrior_id = WarriorId::new(0);

    // Before: Expect core[10].A == 0
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .a
            .number
            .as_index(),
        0
    );

    // Execute 1 instruction.
    mars.step(warrior_id);

    // After: Expect core[10].A == core_size - 1
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .a
            .number
            .as_index(),
        mars.core.get_size() - 1
    );
}

#[test]
fn test_a_indirect_post_increment() {
    let mut mars = Mars::new(
        &[Warrior::from_text(indoc! {"
                nop }10  ; AIndirectPostIncrement
            "})
        .unwrap()],
        &Config::default(),
    );
    let warrior_id = WarriorId::new(0);

    // Before: Expect core[10].A == 0
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .a
            .number
            .as_index(),
        0
    );

    // Execute 1 instruction.
    mars.step(warrior_id);

    // After: Expect core[10].A == 1
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .a
            .number
            .as_index(),
        1
    );
}

#[test]
fn test_b_indirect_pre_decrement() {
    let mut mars = Mars::new(
        &[Warrior::from_text(indoc! {"
                nop <10  ; BIndirectPreDecrement
            "})
        .unwrap()],
        &Config::default(),
    );
    let warrior_id = WarriorId::new(0);

    // Before: Expect core[10].B == 0
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .b
            .number
            .as_index(),
        0
    );

    // Execute 1 instruction.
    mars.step(warrior_id);

    // After: Expect core[10].B == core_size - 1
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .b
            .number
            .as_index(),
        mars.core.get_size() - 1
    );
}

#[test]
fn test_b_indirect_post_increment() {
    let mut mars = Mars::new(
        &[Warrior::from_text(indoc! {"
                nop >10  ; BIndirectPostIncrement
            "})
        .unwrap()],
        &Config::default(),
    );
    let warrior_id = WarriorId::new(0);

    // Before: Expect core[10].B == 0
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .b
            .number
            .as_index(),
        0
    );

    // Execute 1 instruction.
    mars.step(warrior_id);

    // After: Expect core[10].B == 1
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .b
            .number
            .as_index(),
        1
    );
}

#[test]
fn test_side_effects_work_even_if_task_dies() {
    let mut mars = Mars::new(
        &[Warrior::from_text(indoc! {"
                dat {10  ; AIndirectPreDecrement
            "})
        .unwrap()],
        &Config::default(),
    );
    let warrior_id = WarriorId::new(0);

    // Before: Expect core[10].A == 0
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .a
            .number
            .as_index(),
        0
    );

    // Before: Expect 1 task.
    assert_eq!(mars.get_task_queue(warrior_id).len(), 1);

    // Execute 1 instruction.
    mars.step(warrior_id);

    // After: Expect core[10].A == core_size - 1
    assert_eq!(
        mars.core
            .get_cell(CoreNumber::from_usize_unchecked(10))
            .instruction
            .a
            .number
            .as_index(),
        mars.core.get_size() - 1
    );

    // After: Expect 0 task.
    assert!(mars.get_task_queue(warrior_id).is_empty());
}
