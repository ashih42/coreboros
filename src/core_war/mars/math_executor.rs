use crate::core_war::core_number::CoreNumber;

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
/// where all values must be "wrapped" to a value within the range `[0, core_size - 1]` as a `CoreNumber`.
pub struct MathExecutor {
    core_size: usize,
}

impl MathExecutor {
    pub const fn new(core_size: usize) -> Self {
        Self { core_size }
    }

    /// Convert the i32 `number` to a `CoreNumber`.
    const fn wrap(&self, number: i32) -> CoreNumber {
        CoreNumber::from_i32(number, self.core_size)
    }

    /// Perform the specified arithmetic operation with the resulting value wrapped.
    pub const fn do_arithmetic(
        &self,
        arithmetic: ArithmeticOperation,
        a: CoreNumber,
        b: CoreNumber,
    ) -> Option<CoreNumber> {
        use ArithmeticOperation as AO;

        let a = a.as_i32();
        let b = b.as_i32();

        match arithmetic {
            AO::Addition => Some(self.add(a, b)),
            AO::Subtraction => Some(self.subtract(a, b)),
            AO::Multiplication => Some(self.multiply(a, b)),
            AO::Division => self.divide(a, b),
            AO::Modulo => self.modulo(a, b),
        }
    }

    /// Add `a` and `b`.
    /// Note: `a` and `b` are wrapped in range `[0, core_size - 1]`.
    pub const fn add(&self, a: i32, b: i32) -> CoreNumber {
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "Because `a` and `b` are at most (core_size - 1), this expression cannot cause overflow/underflow."
        )]
        self.wrap(b + a)
    }

    /// Subtract `a` from `b`, i.e. `b - a`.
    /// Note: `a` and `b` are wrapped in range `[0, core_size - 1]`.
    pub const fn subtract(&self, a: i32, b: i32) -> CoreNumber {
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "Because `a` and `b` are at most (core_size - 1), this expression cannot cause overflow/underflow."
        )]
        self.wrap(b - a)
    }

    /// Multiply `a` and `b`.
    /// Note: `a` and `b` are wrapped in range `[0, core_size - 1]`.
    const fn multiply(&self, a: i32, b: i32) -> CoreNumber {
        #[allow(
            clippy::arithmetic_side_effects,
            reason = "Because `a` and `b` are at most (core_size - 1), this expression cannot cause overflow/underflow."
        )]
        self.wrap(b * a)
    }

    /// Divide `b` by `a`, i.e. `b / a`.
    /// Note: `a` and `b` are wrapped in range `[0, core_size - 1]`.
    const fn divide(&self, a: i32, b: i32) -> Option<CoreNumber> {
        if a == 0 {
            return None;
        }

        #[allow(
            clippy::arithmetic_side_effects,
            reason = "Because `a` and `b` are at most (core_size - 1), this expression cannot cause overflow/underflow."
        )]
        Some(self.wrap(b / a))
    }

    /// Get remainder of dividing `b` by `a`, i.e. `b % a`.
    /// Note: `a` and `b` are wrapped in range `[0, core_size - 1]`.
    const fn modulo(&self, a: i32, b: i32) -> Option<CoreNumber> {
        if a == 0 {
            return None;
        }

        #[allow(
            clippy::arithmetic_side_effects,
            reason = "Because `a` and `b` are at most (core_size - 1), this expression cause overflow/underflow."
        )]
        Some(self.wrap(b % a))
    }

    pub const fn increment(&self, number: CoreNumber) -> CoreNumber {
        self.add(number.as_i32(), 1)
    }

    pub const fn decrement(&self, number: CoreNumber) -> CoreNumber {
        self.add(number.as_i32(), -1)
    }
}
