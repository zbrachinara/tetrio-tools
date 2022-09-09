pub struct Rng {
    state: u64,
}

impl Rng {
    fn seeded(seed: u64) -> Self {
        Rng { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = 16807 * self.state % 2147483647;
        self.state
    }

    fn next_float(&mut self) -> f64 {
        (self.next() - 1) as f64 / 2147483646.0
    }

    fn shuffle_array<T>(&mut self, arr: &mut [T]) {
        (0..arr.len())
            .rev()
            .map(|i| {
                let swap_with = (self.next_float() * (i + 1) as f64).floor() as usize;
                (i, swap_with)
            })
            .for_each(|(a, b)| {
                arr.swap(a, b);
            });
    }
}
