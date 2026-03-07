use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait PropertyMap {
    type Key;
    type Value;

    fn get_property(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn set_property(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}

pub trait PropertySet {
    type Key;

    fn check(&self, key: &Self::Key) -> bool;
    fn mark(&mut self, key: Self::Key, value: bool) -> bool;
}

impl<K: Hash + Eq, V> PropertyMap for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn get_property(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.get(key)
    }

    fn set_property(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }
}

impl<K: Hash + Eq> PropertySet for HashSet<K> {
    type Key = K;

    fn check(&self, key: &Self::Key) -> bool {
        self.contains(key)
    }

    fn mark(&mut self, key: Self::Key, value: bool) -> bool {
        if value {
            self.insert(key)
        } else {
            self.remove(&key)
        }
    }
}
