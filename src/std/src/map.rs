//! Standard library: Map<K, V> for Gigli

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Map<K, V> {
    data: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Map<K, V> {
    /// Creates a new empty map.
    pub fn new() -> Self {
        Map { data: HashMap::new() }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    /// Gets a reference to the value for the given key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Gets a mutable reference to the value for the given key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    /// Removes a key from the map, returning the value if it existed.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Returns true if the map contains the given key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Returns the number of key-value pairs in the map.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over the key-value pairs.
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the key-value pairs.
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V> {
        self.data.iter_mut()
    }
}
