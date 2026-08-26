use crate::{
    instruction::{
        Instruction, addressing_mode::AddressingMode, modifier::Modifier, operand::Operand,
    },
    mars::{
        address::Address, core::Core, math_executor::ArithmeticOperation, task_outcome::TaskOutcome,
    },
    warrior::warrior_id::WarriorId,
};

// NOTE: All `exec_` functions assume pre-decrements have been done before entry, and post-increments will be done after this function.
// Thus, these functions can handle all 5 variants of indirect addressing modes in the same way simply as indirect addressing mode.

/// `DAT` - Data.
/// Executing this instruction kills the process.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#dat-data>
pub const fn exec_dat(
    _instruction: &Instruction,
    _current_address: Address,
    _core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    die()
}

/// `MOV` - Move.
/// Copy data from source defined in A field to destination defined in B field.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#mov-move>
pub fn exec_mov(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    let (src_instruction, src_a, src_b) =
        core.resolve_instruction_a_b(current_address, instruction.a);

    let dest_address = core.resolve_operand_address(instruction.b, current_address);
    let dest_cell = core.get_cell_mut(dest_address);

    match instruction.operation.modifier {
        Modifier::A => {
            dest_cell.set_a_number(src_a, warrior_id);
        }
        Modifier::B => {
            dest_cell.set_b_number(src_b, warrior_id);
        }
        Modifier::AB => {
            dest_cell.set_b_number(src_a, warrior_id);
        }
        Modifier::BA => {
            dest_cell.set_a_number(src_b, warrior_id);
        }
        Modifier::F => {
            dest_cell.set_a_number(src_a, warrior_id);
            dest_cell.set_b_number(src_b, warrior_id);
        }
        Modifier::X => {
            dest_cell.set_a_number(src_b, warrior_id);
            dest_cell.set_b_number(src_a, warrior_id);
        }
        Modifier::I => {
            dest_cell.set_instruction(src_instruction, warrior_id);
        }
    }

    live(current_address, 1, core)
}

/// `ADD` - Add.
/// This operation may fail as NOP if the destination is not writable.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#add-add>
pub fn exec_add(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    do_arithmetic(
        instruction,
        current_address,
        core,
        warrior_id,
        ArithmeticOperation::Addition,
    )
}

/// `Sub` - Subtract.
/// This operation may fail as NOP if the destination is not writable.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#sub-subtract>
pub fn exec_sub(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    do_arithmetic(
        instruction,
        current_address,
        core,
        warrior_id,
        ArithmeticOperation::Subtraction,
    )
}

/// `MUL` - Multiply.
/// This operation may fail as NOP if the destination is not writable.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#mul-multiply>
pub fn exec_mul(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    do_arithmetic(
        instruction,
        current_address,
        core,
        warrior_id,
        ArithmeticOperation::Multiplication,
    )
}

/// `DIV` - Divide.
/// This operation may fail as NOP if the destination is not writable.
/// This operation may kill the process if the divisor is zero.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#div-divide>
pub fn exec_div(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    do_arithmetic(
        instruction,
        current_address,
        core,
        warrior_id,
        ArithmeticOperation::Division,
    )
}

/// `MOD` - Modulo.
/// This operation may fail as NOP if the destination is not writable.
/// This operation may kill the process if the divisor is zero.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#mod-modulo>
pub fn exec_mod(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    do_arithmetic(
        instruction,
        current_address,
        core,
        warrior_id,
        ArithmeticOperation::Modulo,
    )
}

/// `JMP` - Jump.
/// Jump to the destination in this instruction's A field.
/// This instruction completely ignores its modifier and its B field.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#jmp-jump>
pub fn exec_jmp(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    do_jump(instruction.a, current_address, core)
}

/// `JMZ` - Jump If Zero.
/// Jump to destination in A field, if ALL relevant items for data from instruction's B field are equal to zero.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#jmz-jump-if-zero>
pub fn exec_jmz(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    use Modifier::{A, AB, B, BA, F, I, X};

    let (_, cond_a, cond_b) = core.resolve_instruction_a_b(current_address, instruction.b);

    let should_jump = match instruction.operation.modifier {
        A | BA => cond_a == 0,
        B | AB => cond_b == 0,
        F | X | I => cond_a == 0 && cond_b == 0,
    };

    if should_jump {
        return do_jump(instruction.a, current_address, core);
    }

    live(current_address, 1, core)
}

/// `JMN` - Jump If Not Zero.
/// Jump to destination in A field, if ALL relevant items for data from instruction's B field are NOT equal to zero.
/// Note: This is different from negation of `JMZ` in the truth table for modifiers F, X, I with compound boolean conditions.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#jmn-jump-if-not-zero>
pub fn exec_jmn(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    use Modifier::{A, AB, B, BA, F, I, X};

    let (_, cond_a, cond_b) = core.resolve_instruction_a_b(current_address, instruction.b);

    let should_jump = match instruction.operation.modifier {
        A | BA => cond_a != 0,
        B | AB => cond_b != 0,
        F | X | I => cond_a != 0 && cond_b != 0,
    };

    if should_jump {
        return do_jump(instruction.a, current_address, core);
    }

    live(current_address, 1, core)
}

/// `DJN` - Decrement and Jump If Not Zero.
/// Jump to destination in A field, if the relevant items for data from instruction's B field are first decremented,
/// and then evaluated to check if ANY item is NOT equal to zero.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#djn-decrement-and-jump-if-not-zero>
pub fn exec_djn(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
) -> TaskOutcome {
    use Modifier::{A, AB, B, BA, F, I, X};

    let cond_address = core.resolve_operand_address(instruction.b, current_address);

    let (_, mut cond_a, mut cond_b) = core.resolve_instruction_a_b(current_address, instruction.b);

    // Decrement the copied numbers.
    match instruction.operation.modifier {
        A | AB => {
            cond_a = core.math_executor.decrement(cond_a);
        }
        B | BA => {
            cond_b = core.math_executor.decrement(cond_b);
        }
        F | X | I => {
            cond_a = core.math_executor.decrement(cond_a);
            cond_b = core.math_executor.decrement(cond_b);
        }
    }

    // Decrement the actual locations in the core.
    if instruction.b.mode == AddressingMode::Immediate {
        match instruction.operation.modifier {
            A | AB | F | X | I => {
                // The decremented A field came from the B field in `cond_address`!
                core.decrement_b_number(cond_address, warrior_id);
            }
            B | BA => (),
        }
    } else {
        match instruction.operation.modifier {
            A | AB => {
                core.decrement_a_number(cond_address, warrior_id);
            }
            B | BA => {
                core.decrement_b_number(cond_address, warrior_id);
            }
            F | X | I => {
                core.decrement_a_number(cond_address, warrior_id);
                core.decrement_b_number(cond_address, warrior_id);
            }
        }
    }

    // Decide using the decremented copied numbers.
    let should_jump = match instruction.operation.modifier {
        A | BA => cond_a != 0,
        B | AB => cond_b != 0,
        F | X | I => cond_a != 0 || cond_b != 0,
    };

    if should_jump {
        // Cannot use the cached `instruction.a`, because its core value might have been modified from the decrement earlier!
        let target_operand = core.get_cell(current_address).instruction.a;
        return do_jump(target_operand, current_address, core);
    }

    live(current_address, 1, core)
}

/// `SPL` - Split.
/// Spawn a new task at address in A field.
/// This instruction completely ignores its modifier and its B field.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#spl-split>
pub fn exec_spl(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    let target_address = core.resolve_operand_address(instruction.a, current_address);
    let next_address = core.resolve_address(current_address, 1);

    TaskOutcome::Spawned {
        current_task: next_address,
        new_task: target_address,
    }
}

/// `SEQ` - Skip If Equal.
/// Skip the next instruction if targets in A and B are equal.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#seq-skip-if-equal>
pub fn exec_seq(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    use Modifier::{A, AB, B, BA, F, I, X};

    let (src_instruction, src_a, src_b) =
        core.resolve_instruction_a_b(current_address, instruction.a);

    let (dest_instruction, dest_a, dest_b) =
        core.resolve_instruction_a_b(current_address, instruction.b);

    let should_skip = match instruction.operation.modifier {
        A => src_a == dest_a,
        B => src_b == dest_b,
        AB => src_a == dest_b,
        BA => src_b == dest_a,
        F => (src_a == dest_a) && (src_b == dest_b),
        X => (src_a == dest_b) && (src_b == dest_a),
        I => src_instruction == dest_instruction,
    };

    let next_address_offset = if should_skip { 2 } else { 1 };
    live(current_address, next_address_offset, core)
}

/// `SNE` - Skip If Not Equal.
/// Skip the next instruction if targets in A and B are NOT equal.
/// This is not simply the exact opposite conditional of `SEQ`.  There are edge cases where both `SEQ` and `SNE` would evaluate to the same condition.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#sne-skip-if-not-equal>
pub fn exec_sne(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    use Modifier::{A, AB, B, BA, F, I, X};

    let (src_instruction, src_a, src_b) =
        core.resolve_instruction_a_b(current_address, instruction.a);

    let (dest_instruction, dest_a, dest_b) =
        core.resolve_instruction_a_b(current_address, instruction.b);

    let should_skip = match instruction.operation.modifier {
        A => src_a != dest_a,
        B => src_b != dest_b,
        AB => src_a != dest_b,
        BA => src_b != dest_a,
        F => (src_a != dest_a) && (src_b != dest_b),
        X => (src_a != dest_b) && (src_b != dest_a),
        I => src_instruction != dest_instruction,
    };

    let next_address_offset = if should_skip { 2 } else { 1 };
    live(current_address, next_address_offset, core)
}

/// `SLE` - Skip If Less Than.
/// Skip the next instruction if target in A < target in B.
/// This is also slightly different from `SEQ` and `SLE` in that .I is equivalent to .F,
/// since an instruction cannot be compared to be less than another instruction.
/// Reference: <https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/#slt-skip-if-less-than>
pub fn exec_slt(
    instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    use Modifier::{A, AB, B, BA, F, I, X};

    let (_, src_a, src_b) = core.resolve_instruction_a_b(current_address, instruction.a);

    let (_, dest_a, dest_b) = core.resolve_instruction_a_b(current_address, instruction.b);

    let should_skip = match instruction.operation.modifier {
        A => src_a < dest_a,
        B => src_b < dest_b,
        AB => src_a < dest_b,
        BA => src_b < dest_a,
        F | I => (src_a < dest_a) && (src_b < dest_b),
        X => (src_a < dest_b) && (src_b < dest_a),
    };

    let next_address_offset = if should_skip { 2 } else { 1 };
    live(current_address, next_address_offset, core)
}

/// `NOP` - No Operation.
/// This operation does nothing.
pub const fn exec_nop(
    _instruction: &Instruction,
    current_address: Address,
    core: &Core,
    _warrior_id: WarriorId,
) -> TaskOutcome {
    live(current_address, 1, core)
}

/// Try to perform the arithmetic operation.  If the operation fails (e.g. division or modulo by 0), the process dies.
fn do_arithmetic(
    instruction: &Instruction,
    current_address: Address,
    core: &mut Core,
    warrior_id: WarriorId,
    arithmetic: ArithmeticOperation,
) -> TaskOutcome {
    let (_, src_a, src_b) = core.resolve_instruction_a_b(current_address, instruction.a);

    let (_, dest_a, dest_b) = core.resolve_instruction_a_b(current_address, instruction.b);

    let dest_address = core.resolve_operand_address(instruction.b, current_address);

    match instruction.operation.modifier {
        Modifier::A => {
            if let Some(result_a) = core.math_executor.do_arithmetic(arithmetic, src_a, dest_a) {
                let dest_cell = core.get_cell_mut(dest_address);
                dest_cell.set_a_number(result_a, warrior_id);
                return live(current_address, 1, core);
            }
        }
        Modifier::B => {
            if let Some(result_b) = core.math_executor.do_arithmetic(arithmetic, src_b, dest_b) {
                let dest_cell = core.get_cell_mut(dest_address);
                dest_cell.set_b_number(result_b, warrior_id);
                return live(current_address, 1, core);
            }
        }
        Modifier::AB => {
            if let Some(result_b) = core.math_executor.do_arithmetic(arithmetic, src_a, dest_b) {
                let dest_cell = core.get_cell_mut(dest_address);
                dest_cell.set_b_number(result_b, warrior_id);
                return live(current_address, 1, core);
            }
        }
        Modifier::BA => {
            if let Some(result_a) = core.math_executor.do_arithmetic(arithmetic, src_b, dest_a) {
                let dest_cell = core.get_cell_mut(dest_address);
                dest_cell.set_a_number(result_a, warrior_id);
                return live(current_address, 1, core);
            }
        }
        Modifier::F | Modifier::I => {
            if let Some(result_a) = core.math_executor.do_arithmetic(arithmetic, src_a, dest_a)
                && let Some(result_b) = core.math_executor.do_arithmetic(arithmetic, src_b, dest_b)
            {
                let dest_cell = core.get_cell_mut(dest_address);
                dest_cell.set_a_number(result_a, warrior_id);
                dest_cell.set_b_number(result_b, warrior_id);
                return live(current_address, 1, core);
            }
        }
        Modifier::X => {
            if let Some(result_a) = core.math_executor.do_arithmetic(arithmetic, src_b, dest_a)
                && let Some(result_b) = core.math_executor.do_arithmetic(arithmetic, src_a, dest_b)
            {
                let dest_cell = core.get_cell_mut(dest_address);
                dest_cell.set_a_number(result_a, warrior_id);
                dest_cell.set_b_number(result_b, warrior_id);
                return live(current_address, 1, core);
            }
        }
    }

    die()
}

const fn die() -> TaskOutcome {
    TaskOutcome::Died
}

const fn live(current_address: Address, next_address_offset: i32, core: &Core) -> TaskOutcome {
    let next_address = core.resolve_address(current_address, next_address_offset);

    TaskOutcome::Lived {
        current_task: next_address,
    }
}

fn do_jump(target_operand: Operand, current_address: Address, core: &Core) -> TaskOutcome {
    let target_address = core.resolve_operand_address(target_operand, current_address);

    TaskOutcome::Lived {
        current_task: target_address,
    }
}
