use smallvec::SmallVec;

#[derive(Debug)]
pub struct KeyValueStore<K, V, const N: usize> {
    data: SmallVec<[(K, V); N]>,
}

impl<K, V, const N: usize> Default for KeyValueStore<K, V, N> {
    fn default() -> Self {
        Self {
            data: SmallVec::new(),
        }
    }
}

impl<K, V, const N: usize> KeyValueStore<K, V, N> {
    pub fn new() -> Self {
        Self {
            data: SmallVec::new(),
        }
    }
}

impl<K, V, const N: usize> KeyValueStore<K, V, N>
where
    K: PartialEq + Clone + Default,
    V: Clone + Default,
{
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
    pub fn get_by_index(&self, index: usize) -> Option<&V> {
        self.data.get(index).map(|pair| &pair.1)
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
        let mut kv = KeyValueStore::<_, _, 0>::new();
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        assert_eq!(kv.get(&"key1"), Some(&1));
        assert_eq!(kv.get(&"key2"), Some(&2));
        assert_eq!(kv.get(&"key3"), Some(&3));
    }

    #[test]
    fn test_remove() {
        let mut kv = KeyValueStore::<_, _, 0>::new();
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        assert_eq!(kv.remove(&"key2"), Some(2));
        assert_eq!(kv.get(&"key2"), None);
    }

    #[test]
    fn test_items() {
        let mut kv = KeyValueStore::<_, _, 0>::new();
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        for (key, value) in kv.items() {
            assert_eq!(kv.get(key), Some(value));
        }
    }

    #[test]
    fn test_get_by_index() {
        let mut kv = KeyValueStore::<_, _, 3>::new();
        assert_eq!(kv.data.capacity(), 3);
        kv.insert("key1", 1);
        kv.insert("key2", 2);
        kv.insert("key3", 3);
        assert_eq!(kv.get_by_index(1), Some(&2));
        assert_eq!(kv.data.capacity(), 3);
    }

    #[test]
    fn test_with_capacity() {
        let kv = KeyValueStore::<&str, i32, 10>::new();
        assert_eq!(kv.data.capacity(), 10);
    }

    #[test]
    fn test_default() {
        let kv = KeyValueStore::<&str, i32, 0>::default();
        assert_eq!(kv.data.capacity(), 0);
    }
}
