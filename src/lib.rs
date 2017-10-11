use std::sync::Arc;

pub struct Keyed<K, V> where K: Key<Asset=V> {
    key: K,
    value: Option<Arc<V>>,
}

pub trait Key: Sized {
    type Asset;
}

impl<K, V> Keyed<K, V> where K: Key<Asset=V> {
    pub fn new(key: K) -> Self {
        let value = None;
        Keyed { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn as_ref(&self) -> Option<&V> {
        self.value.as_ref().map(|arc| arc.as_ref())
    }
}
