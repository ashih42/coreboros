/// Initialize the random number generator's seed with current timestamp.
pub fn init_rng() {
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
}

/// Generate a random value in [`low`, `high`).
pub fn rand_range(low: usize, high: usize) -> usize {
    macroquad::rand::gen_range(low, high)
}
