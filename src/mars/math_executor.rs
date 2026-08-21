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
}

impl MathExecutor {
    pub fn new(core_size: usize) -> Self {
        Self {
            core_size: core_size as i32,
        }
    }

    pub fn wrap_instruction(&self, instruction: &Instruction) -> Instruction {
        let mut wrapped_instruction = *instruction;

        wrapped_instruction.a.number = self.wrap(wrapped_instruction.a.number);
        wrapped_instruction.b.number = self.wrap(wrapped_instruction.b.number);

        wrapped_instruction
    }

    fn wrap(&self, number: i32) -> i32 {
        number.rem_euclid(self.core_size)
    }

    pub fn increment(&self, number: i32) -> i32 {
        self.add(number, 1)
    }

    pub fn decrement(&self, number: i32) -> i32 {
        self.subtract(number, 1)
    }

    pub fn do_arithmetic(&self, arithmetic: ArithmeticOperation, a: i32, b: i32) -> Option<i32> {
        use ArithmeticOperation as AO;

        match arithmetic {
            AO::Addition => Some(self.add(a, b)),
            AO::Subtraction => Some(self.subtract(a, b)),
            AO::Multiplication => Some(self.multiply(a, b)),
            AO::Division => self.divide(a, b),
            AO::Modulo => self.modulo(a, b),
        }
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        self.wrap(a.wrapping_add(b))
    }

    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        self.wrap(a.wrapping_sub(b))
    }

    fn multiply(&self, a: i32, b: i32) -> i32 {
        self.wrap(a.wrapping_mul(b))
    }

    fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            return None;
        }

        Some(self.wrap(a.div_euclid(b)))
    }

    fn modulo(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            return None;
        }

        Some(self.wrap(a.rem_euclid(b)))
    }
}
