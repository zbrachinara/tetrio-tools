pub struct Rng {
    state: u64,
}

impl Rng {
    fn seeded(seed: u64) -> Self {
        Rng { state: seed % 2147483647 }
    }

    fn next(&mut self) -> u64 {
        self.state = 16807 * self.state % 2147483647;
        self.state
    }

    fn next_float(&mut self) -> f64 {
        (self.next() - 1) as f64 / 2147483646.0
    }

    fn shuffle_slice<T>(&mut self, slice: &mut [T]) {
        (0..slice.len())
            .rev()
            .map(|i| {
                let swap_with = (self.next_float() * (i + 1) as f64).floor() as usize;
                (i, swap_with)
            })
            .for_each(|(a, b)| {
                slice.swap(a, b);
            });
    }

    fn shuffle_array<T, const N: usize>(&mut self, mut arr: [T; N]) -> [T; N] {
        self.shuffle_slice(&mut arr);
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    #[test]
    fn based() {
        let mut base = ['z', 'l', 'o', 's', 'i', 'j', 't'];

        // game 1 of the file included with this source
        let mut rng = Rng::seeded(1742887628);

        assert_eq!(
            rng.shuffle_array(base.clone()),
            ['z', 'l', 'i', 'o', 'j', 't', 's']
        );
        assert_eq!(
            rng.shuffle_array(base.clone()),
            ['i', 't', 'j', 'l', 'o', 'z', 's']
        );
    }
}
