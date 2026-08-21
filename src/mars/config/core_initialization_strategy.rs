#[derive(Clone, Copy)]
pub enum CoreInitializationStrategy {
    FillDat00,
    Leftover,
    Random,
}
