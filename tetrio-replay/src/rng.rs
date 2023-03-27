use std::collections::VecDeque;

use ttrm::GameType;
use viewtris::tetromino::MinoVariant;

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

    #[allow(dead_code)]
    fn shuffle_array<T, const N: usize>(&mut self, mut arr: [T; N]) -> [T; N] {
        self.shuffle_slice(&mut arr);
        arr
    }

    fn shuffle_boxed_slice<T>(&mut self, mut slice: Box<[T]>) -> Box<[T]> {
        self.shuffle_slice(&mut slice);
        slice
    }
}

/// A piece queue which uses the tetrio RNG strategy to generate new pieces.
pub struct PieceQueue {
    window: VecDeque<MinoVariant>,
    base: Box<[MinoVariant]>,
    rng: Rng,
}

impl PieceQueue {
    /// Creates a `PieceQueue` which returns garbage. Meant for usage in tests.
    #[cfg(test)] // is meant for tests
    pub fn meaningless() -> Self {
        Self::standard(1)
    }

    fn seeded_with_base(seed: u64, base: Box<[MinoVariant]>) -> Self {
        let rng = Rng::seeded(seed);
        let window = VecDeque::new();
        Self { rng, base, window }
    }

    pub fn from_game(game: GameType, seed: u64) -> Self {
        match game {
            GameType::FortyLine => Self::fortyline(seed),
            GameType::League | GameType::Custom => Self::standard(seed),
            _ => unimplemented!("It is not yet known what queue satisfies this game type"),
        }
    }

    /// Creates a piece queue with the given seed, equivalent to the ones used in multi games and
    /// default custom games.
    pub fn standard(seed: u64) -> Self {
        use MinoVariant::*;
        let base = Box::new([Z, L, O, S, I, J, T]);
        Self::seeded_with_base(seed, base)
    }

    pub fn fortyline(seed: u64) -> Self {
        use MinoVariant::*;
        let base = Box::new([I, O, T, Z, J, L, S]);
        Self::seeded_with_base(seed, base)
    }

    /// Return the next piece held in the queue and generate more pieces if necessary
    pub fn pop(&mut self) -> MinoVariant {
        if self.window.is_empty() {
            self.generate();
        }
        let ret = self.window.pop_front();
        ret.unwrap()
    }

    pub fn generate(&mut self) {
        self.window
            .extend(self.rng.shuffle_boxed_slice(self.base.clone()).iter())
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
