use crate::Text;
use hashbrown::HashMap;
use std::hash::Hash;

/// A map of keys of type T to value strings, and a map of the same keys to the lengths of the strings.
pub struct ValueMap<T>
where
    T: Eq + Hash + Copy,
{
    /// A map of value strings corresponding with keys of type T.
    values: HashMap<T, String>,
    /// The maximum length of the values strings.
    pub max_length: u32,
}

impl<T> ValueMap<T>
where
    T: Eq + Hash + Copy,
{
    /// - `keys` the lookup keys of the underlying HashMap.
    /// - `values` The string look-up keys that will be used in `text.get(v)`.
    /// - `text` The text.
    pub fn new<const N: usize>(keys: [T; N], values: [&str; N], text: &Text) -> Self {
        const EMPTY_STRING: String = String::new();
        let mut strings: [String; N] = [EMPTY_STRING; N];
        for (i, v) in values.iter().enumerate() {
            strings[i] = text.get(v);
        }
        Self::new_from_strings(keys, strings)
    }

    /// - `keys` the lookup keys of the underlying HashMap.
    /// - `values` The string values.
    pub fn new_from_strings<const N: usize>(keys: [T; N], values: [String; N]) -> Self {
        let mut vs = HashMap::new();
        let mut lengths = Vec::new();
        for (k, v) in keys.iter().zip(values) {
            let length = v.chars().count() as u32;
            vs.insert(*k, v);
            lengths.push(length);
        }
        let max_length = *lengths.iter().max().unwrap();
        Self {
            values: vs,
            max_length,
        }
    }

    pub fn get(&self, key: &T) -> &String {
        &self.values[key]
    }
}
