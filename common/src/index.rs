use serde::{Deserialize, Serialize};

/// An `Index` is an index in a known-length array.
/// The index can be incremented or decremented past the bounds of length, in which case it will loop to the start/end value.
/// The index can never exceed the length.
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Index {
    /// The index in the array.
    index: usize,
    /// The length of the array.
    length: usize,
}

impl Index {
    /// - `index` The index in the array.
    /// - `length` The size of the array.
    pub fn new(index: usize, length: usize) -> Self {
        Self { index, length }
    }

    /// Increment or decrement the index.
    ///
    /// If the incremented index is greater than `self.length`, `self.index` is set to 0.
    /// If the decremented index would be less than 0, `self.index` is set to `self.length - 1`.
    ///
    /// - `up` If true, increment. If false, decrement.
    pub fn increment(&mut self, up: bool) {
        self.index = match up {
            true => match self.index == self.length - 1 {
                true => 0,
                false => self.index + 1,
            },
            false => match self.index == 0 {
                true => self.length - 1,
                false => self.index - 1,
            },
        };
    }

    /// Returns the index.
    pub fn get(&self) -> usize {
        self.index
    }

    /// Set `self.index` to `index`. Panics if `index` is greater than or equal to `self.length`.
    pub fn set(&mut self, index: usize) {
        if index >= self.length {
            panic!("Index {} exceeds length {}!", index, self.length)
        } else {
            self.index = index
        }
    }
}
