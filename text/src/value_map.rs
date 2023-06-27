use crate::Text;
use common::hashbrown::HashMap;
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
    pub fn new<const N: usize>(keys: [T; N], values: [&str; N], text: &Text) -> Self {
        let mut vs = HashMap::new();
        let mut lengths = Vec::new();
        for (k, v) in keys.iter().zip(values) {
            let value = text.get(v);
            let length = value.chars().count() as u32;
            vs.insert(*k, value);
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
