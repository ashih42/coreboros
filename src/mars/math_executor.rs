use crate::instruction::Instruction;

/// `ArithmeticOperation` indicates the 5 kinds of math operations to be performed
/// with the resulting value wrapped within [0, `core_size`).
#[derive(Clone, Copy)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

/// `MathExecutor` is responsible for all math operations while executing instructions in the core,
/// where all values must be "wrapped", or transformed to a value within [0, `core_size`).
pub struct MathExecutor {
    core_size: i32,
}

impl MathExecutor {
    pub const fn new(core_size: usize) -> Self {
        Self {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_possible_wrap,
                clippy::as_conversions,
                reason = "The conversion is safe 👌"
            )]
            core_size: core_size as i32,
        }
    }

    /// Transform the `number` to a value within [0, `core_size`).
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "The subtraction operation is safe 👌"
    )]
    const fn wrap(&self, number: i32) -> i32 {
        number.rem_euclid(self.core_size)
    }

    /// Return a new instruction with its A and B numbers wrapped.
    pub const fn wrap_instruction(&self, instruction: &Instruction) -> Instruction {
        let mut wrapped_instruction = *instruction;

        wrapped_instruction.a.number = self.wrap(instruction.a.number);
        wrapped_instruction.b.number = self.wrap(instruction.b.number);

        wrapped_instruction
    }

    /// Perform the specified arithmetic operation with the resulting value wrapped.
    pub const fn do_arithmetic(
        &self,
        arithmetic: ArithmeticOperation,
        a: i32,
        b: i32,
    ) -> Option<i32> {
        use ArithmeticOperation as AO;

        match arithmetic {
            AO::Addition => Some(self.add(a, b)),
            AO::Subtraction => Some(self.subtract(a, b)),
            AO::Multiplication => Some(self.multiply(a, b)),
            AO::Division => self.divide(a, b),
            AO::Modulo => self.modulo(a, b),
        }
    }

    pub const fn add(&self, a: i32, b: i32) -> i32 {
        self.wrap(a.wrapping_add(b))
    }

    pub const fn subtract(&self, a: i32, b: i32) -> i32 {
        self.wrap(a.wrapping_sub(b))
    }

    const fn multiply(&self, a: i32, b: i32) -> i32 {
        self.wrap(a.wrapping_mul(b))
    }

    const fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            return None;
        }

        Some(self.wrap(a.div_euclid(b)))
    }

    const fn modulo(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            return None;
        }

        Some(self.wrap(a.rem_euclid(b)))
    }

    pub const fn increment(&self, number: i32) -> i32 {
        self.add(number, 1)
    }

    pub const fn decrement(&self, number: i32) -> i32 {
        self.subtract(number, 1)
    }
}
