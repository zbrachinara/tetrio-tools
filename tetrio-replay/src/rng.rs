use std::collections::VecDeque;

use bsr_tools::tetromino::MinoVariant;

/// The RNG used by tetrio to generate new pieces.
pub struct Rng {
    state: u64,
}

impl Rng {
    fn seeded(seed: u64) -> Self {
        Rng {
            state: seed % 2147483647,
        }
    }

    fn next(&mut self) -> u64 {
        self.state = 16807 * self.state % 2147483647;
        self.state
    }

    fn next_float(&mut self) -> f64 {
        (self.next() - 1) as f64 / 2147483646.0
    }

    fn shuffle_slice<T>(&mut self, slice: &mut [T]) {
        (1..slice.len())
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

/// A piece queue which uses the tetrio RNG strategy to generate new pieces.
pub struct PieceQueue {
    window: VecDeque<MinoVariant>, //TODO: Determine whether window is necessary
    window_size: usize,
    rng: Rng,
}

impl PieceQueue {
    /// Creates a dummy piecequeue that won't be used (for tests, for example)
    #[cfg(test)] // is meant for tests
    pub fn meaningless() -> Self {
        Self::seeded(1, 1)
    }

    /// Creates a new piece queue with a seed and a preview window size
    pub fn seeded(seed: u64, window_size: usize) -> Self {
        let mut rng = Rng::seeded(seed);
        let mut window = VecDeque::with_capacity(window_size / 7 * 7);

        while window.len() < window_size {
            use MinoVariant::*;
            window.extend(rng.shuffle_array([Z, L, O, S, I, J, T]))
        }

        Self {
            rng,
            window,
            window_size,
        }
    }

    /// Returns a view into the preview window of the queue
    pub fn window(&self) -> impl Iterator<Item = &MinoVariant> {
        self.window.iter().take(self.window_size)
    }

    /// Return the next piece held in the queue and generate more pieces if necessary
    pub fn pop(&mut self) -> MinoVariant {
        let ret = self.window.pop_front();
        self.fill();
        ret.unwrap()
    }

    /// Add as many pieces as necessary to fill up the display window
    pub fn fill(&mut self) {
        if self.window.len() < self.window_size {
            self.generate()
        }
    }

    pub fn generate(&mut self) {
        use MinoVariant::*;
        self.window
            .extend(self.rng.shuffle_array([Z, L, O, S, I, J, T]))
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    #[test]
    fn based() {
        let base = ['z', 'l', 'o', 's', 'i', 'j', 't'];

        // game 1 of the file included with this source
        let mut rng = Rng::seeded(1742887628);

        assert_eq!(rng.shuffle_array(base), ['z', 'l', 'i', 'o', 'j', 't', 's']);
        assert_eq!(rng.shuffle_array(base), ['i', 't', 'j', 'l', 'o', 'z', 's']);
        assert_eq!(rng.shuffle_array(base), ['s', 'j', 't', 'o', 'l', 'i', 'z']);
        assert_eq!(rng.shuffle_array(base), ['l', 'j', 't', 'o', 's', 'i', 'z']);
    }
}
