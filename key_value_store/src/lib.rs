use smallvec::{smallvec, SmallVec};

#[derive(Debug, Default)]
pub struct KeyValueStore<K, V> {
    data: SmallVec<[(K, V); 10]>,
}

impl<K, V> KeyValueStore<K, V>
where
    K: PartialEq + Clone + Default,
    V: Clone + Default,
{
    pub fn new() -> Self {
        KeyValueStore {
            data: smallvec![(Default::default(), Default::default()); 10],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        KeyValueStore {
            data: smallvec![(Default::default(), Default::default()); capacity],
        }
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        self.data.push((key, value));
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data
            .iter()
            .find(|pair| &pair.0 == key)
            .map(|pair| &pair.1)
    }

    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut index = None;
        for (i, pair) in self.data.iter().enumerate() {
            if &pair.0 == key {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => Some(self.data.remove(i).1),
            None => None,
        }
    }

    #[inline]
    pub fn items(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter().map(|pair| (&pair.0, &pair.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut kv = KeyValueStore::new();
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        assert_eq!(kv.get(&"key1"), Some(&1));
        assert_eq!(kv.get(&"key2"), Some(&2));
        assert_eq!(kv.get(&"key3"), Some(&3));
    }

    #[test]
    fn test_remove() {
        let mut kv = KeyValueStore::new();
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        assert_eq!(kv.remove(&"key2"), Some(2));
        assert_eq!(kv.get(&"key2"), None);
    }

    #[test]
    fn test_items() {
        let mut kv = KeyValueStore::new();
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        for (key, value) in kv.items() {
            assert_eq!(kv.get(key), Some(value));
        }
    }
}
