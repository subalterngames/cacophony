use serde::{Deserialize, Serialize};

/// An `Index` is an index in a known-length array.
/// The index can be incremented or decremented past the bounds of length, in which case it will loop to the start/end value.
/// The index can never exceed the length.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
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
        if self.length == 0 {
            return;
        }
        self.index = if up {
            if self.index == self.length - 1 {
                0
            } else {
                self.index + 1
            }
        } else if self.index == 0 {
            self.length - 1
        } else {
            self.index - 1
        };
    }

    /// Increment or decrement the index without looping around the bounds.
    ///
    /// - `up` If true, increment. If false, decrement.
    ///
    /// Returns true if we incremented.
    pub fn increment_no_loop(&mut self, up: bool) -> bool {
        if self.length == 0 {
            false
        } else if up {
            self.index += 1;
            true
        } else if self.index > 0 {
            self.index -= 1;
            true
        } else {
            false
        }
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

    /// Returns the length of the value range.
    pub fn get_length(&self) -> usize {
        self.length
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Index;

    #[test]
    fn index() {
        // Zero.
        let mut i = Index::default();
        assert_eq!(i.index, 0);
        assert_eq!(i.length, 0);
        i.increment(true);
        assert_eq!(i.index, 0);
        assert_eq!(i.length, 0);
        i.increment(false);
        assert_eq!(i.index, 0);
        assert_eq!(i.length, 0);
        i.increment_no_loop(true);
        assert_eq!(i.index, 0);
        assert_eq!(i.length, 0);
        i.increment_no_loop(false);
        assert_eq!(i.index, 0);
        assert_eq!(i.length, 0);
        // Some.
        i = Index::new(1, 9);
        assert_eq!(i.index, 1);
        assert_eq!(i.index, i.get());
        assert_eq!(i.length, 9);
        assert_eq!(i.length, i.get_length());
        i.increment(false);
        assert_eq!(i.get(), 0);
        i.increment(false);
        assert_eq!(i.get(), 8);
        i.increment(true);
        assert_eq!(i.get(), 0);
        i.increment_no_loop(false);
        assert_eq!(i.get(), 0);
    }
}
