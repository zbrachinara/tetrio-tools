use gridly::{
    prelude::*,
    vector::{Columns, Rows, Vector},
};

struct BoardStorage<T> {
    columns: usize,
    storage: Vec<Vec<T>>,
}

impl<T> BoardStorage<T> {
    /// Constructs a new grid but does not check for rectangularity
    fn new_from_rows_unchecked(v: Vec<Vec<T>>) -> Self {
        Self {
            columns: v.get(0).map(|v| v.len()).unwrap_or(0),
            storage: v,
        }
    }
}

impl<T> GridBounds for BoardStorage<T> {
    fn dimensions(&self) -> Vector {
        Vector {
            rows: Rows(self.storage.len() as isize),
            columns: Columns(self.columns as isize),
        }
    }

    fn root(&self) -> Location {
        Location::zero()
    }
}

impl<T> Grid for BoardStorage<T> {
    type Item = T;

    unsafe fn get_unchecked(&self, location: gridly::prelude::Location) -> &Self::Item {
        self.storage
            .get(location.row.0 as usize)
            .unwrap()
            .get(location.column.0 as usize)
            .unwrap()
    }
}

impl<T> GridMut for BoardStorage<T> {
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        self.storage
            .get_mut(location.row.0 as usize)
            .unwrap()
            .get_mut(location.column.0 as usize)
            .unwrap()
    }
}

impl<T> GridSetter for BoardStorage<T> {
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        std::mem::replace(self.get_unchecked_mut(location), value)
    }

    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        *self.get_unchecked_mut(location) = value
    }
}
