/// `CoreNumber` is a non-negative integer within the range `[0, core_size - 1]`.
///
/// There are 2 situations where `CoreNumber` exist:
/// - All addresses in the `Core` are `CoreNumber`.
///   - Thus, it follows that "tasks" inside `TaskQueue` are `CoreNumber` as well.
/// - All numeric values stored in the `Core` are `CoreNumber`.
///   - `Instruction` is transformed to `CoreInstruction` containing `CoreNumber`.
///   - All effects from executing instructions must result in `CoreNumber`.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct CoreNumber(u16);

impl CoreNumber {
    /// Construct `CoreNumber` from a i32 `value`, which is transformed to some value within the range `[0, core_size - 1]`.
    pub const fn from_i32(value: i32, core_size: usize) -> Self {
        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            reason = "`core_size` is a small number that can fit within a `i32`."
        )]
        let wrapped_number = value.rem_euclid(core_size as i32);

        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "`wrapped_number` is guaranteed to be a small value that fits in a `u16`."
        )]
        Self(wrapped_number as u16)
    }

    /// Construct `CoreNumber` from a usize `value`, which is transformed to some value within the range `[0, core_size - 1]`.
    pub const fn from_usize(value: usize, core_size: usize) -> Self {
        let wrapped_number = value.rem_euclid(core_size);

        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            reason = "`wrapped_number` is guaranteed to be a small value that fits in a `u16`."
        )]
        Self(wrapped_number as u16)
    }

    /// Construct `CoreNumber` from a usize `value` that is guaranteed to be within the range `[0, core_size - 1]`.
    pub const fn from_usize_unchecked(value: usize) -> Self {
        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            reason = "`value` is guaranteed to be a small value that fits in a `u16`."
        )]
        Self(value as u16)
    }

    /// Convert `CoreNumber` to an index for accessing `CoreCell` in `Core`.
    #[inline]
    pub const fn as_index(self) -> usize {
        #[allow(
            clippy::as_conversions,
            reason = "It is safe to promote a `u16` value to `usize`."
        )]
        {
            self.0 as usize
        }
    }

    /// Convert `CoreNumber` to an `i32` for math operations.
    #[inline]
    pub const fn as_i32(self) -> i32 {
        #[allow(
            clippy::as_conversions,
            reason = "It is safe to promote a `u16` value to `i32`."
        )]
        {
            self.0 as i32
        }
    }
}
