use crate::Index;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// An `Index` with an array of values of type T.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
pub struct IndexedValues<T, const N: usize>
where
    [T; N]: Serialize + DeserializeOwned,
    T: Copy + Default,
{
    values: [T; N],
    pub index: Index,
}

impl<T, const N: usize> Default for IndexedValues<T, N>
where
    [T; N]: Serialize + DeserializeOwned,
    T: Copy + Default,
{
    fn default() -> Self {
        Self {
            values: [T::default(); N],
            index: Index::default(),
        }
    }
}

impl<T, const N: usize> IndexedValues<T, N>
where
    [T; N]: Serialize + DeserializeOwned,
    T: Copy + Default,
{
    pub fn new(index: usize, values: [T; N]) -> Self {
        let index = Index::new(index, values.len());
        Self { values, index }
    }

    /// Returns the value at the index.
    pub fn get(&self) -> T {
        self.values[self.index.get()]
    }
}
