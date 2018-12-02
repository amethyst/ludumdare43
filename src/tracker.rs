pub struct GameTracker {
    currency: u32,
    pub score: u32,
}
impl Default for GameTracker {
    fn default() -> Self {
        Self {
            currency: 0,
            score: 0,
        }
    }
}