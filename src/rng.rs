/// Initialize the random number generator's seed with current timestamp.
pub fn init_rng() {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions,
        reason = "The seed value doesn't matter."
    )]
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
}

/// Generate a random value in [`low`, `high`).
#[must_use]
pub fn rand_range(low: usize, high: usize) -> usize {
    macroquad::rand::gen_range(low, high)
}

/// Randomly shuffle `items` by the Fisher–Yates shuffle algorithm.
#[allow(
    clippy::arithmetic_side_effects,
    reason = "The length of `items` is small."
)]
pub fn shuffle<T>(items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let j = macroquad::rand::gen_range(0, i + 1);
        items.swap(i, j);
    }
}
