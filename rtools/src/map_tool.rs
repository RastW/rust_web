use std::{collections::HashMap, hash::Hash};

// pub struct HashMapTool;

// impl HashMapTool {}
pub fn of<K: Eq + Hash, V>(k: K, v: V) -> HashMap<K, V> {
    let mut result = HashMap::new();
    result.insert(k, v);
    result
}

pub fn of2<K: Eq + Hash, V>(k1: K, v1: V, k2: K, v2: V) -> HashMap<K, V> {
    let mut result = HashMap::new();
    result.insert(k1, v1);
    result.insert(k2, v2);
    result
}

pub fn of3<K: Eq + Hash, V>(k1: K, v1: V, k2: K, v2: V, k3: K, v3: V) -> HashMap<K, V> {
    let mut result = HashMap::new();
    result.insert(k1, v1);
    result.insert(k2, v2);
    result.insert(k3, v3);
    result
}
