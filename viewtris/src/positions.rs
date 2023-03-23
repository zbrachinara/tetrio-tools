use std::ops::Add;

use duplicate::duplicate_item;
use tap::Tap;

/// A list of positions the cells of a mino takes up. Ordinarily, the cells which a mino takes up
/// are expressed in terms of the position of its center, its rotation state, and its type. These
/// are transformed into this direct list of positions using [Positions::tetromino]
#[derive(Debug, Clone)]
pub struct Positions<const N: usize>(pub [(isize, isize); N]);

#[allow(clippy::unnecessary_cast)]
#[duplicate_item(
    ty;
    [i8];
    [i16];
    [isize];
)]
impl<const N: usize> Add<(ty, ty)> for Positions<N> {
    type Output = Positions<N>;

    fn add(self, (rhs_x, rhs_y): (ty, ty)) -> Self::Output {
        self.tap_mut(|arr| {
            arr.0.iter_mut().for_each(|(x, y)| {
                *x += rhs_x as isize;
                *y += rhs_y as isize;
            })
        })
    }
}

impl<const N: usize> Positions<N> {
    pub fn iter(&self) -> impl Iterator<Item = &(isize, isize)> {
        self.0.iter()
    }

    /// Resorts the positions contained by how low they are on the board
    pub fn lowest_first(self) -> Self {
        self.tap_mut(|pos| pos.0.sort_by(|(_, y1), (_, y2)| y1.cmp(y2)))
    }
}
