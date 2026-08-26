use crate::instruction::Instruction;

#[derive(Clone, Copy)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

pub struct MathExecutor {
    core_size: i32,
    half_size: i32,
}

impl MathExecutor {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::as_conversions,
        reason = "The conversion is safe 👌"
    )]
    pub const fn new(core_size: usize) -> Self {
        Self {
            core_size: core_size as i32,
            half_size: (core_size as i32) / 2,
        }
    }

    pub const fn wrap_instruction(&self, instruction: &Instruction) -> Instruction {
        let mut wrapped_instruction = *instruction;

        wrapped_instruction.a.number = self.wrap(wrapped_instruction.a.number);
        wrapped_instruction.b.number = self.wrap(wrapped_instruction.b.number);

        wrapped_instruction
    }

    // Normalize `number` to a small value within range of `core_size` centered around 0.
    /// Example:
    /// If `coresize`=8000, this operation maps `number` to some value in [-4000, 3999]
    #[allow(
        clippy::arithmetic_side_effects,
        reason = "The subtraction operation is safe 👌"
    )]
    const fn wrap(&self, number: i32) -> i32 {
        let value = number.rem_euclid(self.core_size);

        // Return a small non-negative number as is.
        if value < self.half_size {
            return value;
        }

        // Return a a large positive number as a small negative number.
        value - self.core_size
    }

    pub const fn increment(&self, number: i32) -> i32 {
        self.add(number, 1)
    }

    pub const fn decrement(&self, number: i32) -> i32 {
        self.subtract(number, 1)
    }

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
}
