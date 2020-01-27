pub struct Random {
    pub seed: i64,
}

impl Random {
    pub fn next(&mut self) -> i64 {
        self.seed = (1103515245 * self.seed + 12345) % 2147483648;
        return self.seed;
    }
}
