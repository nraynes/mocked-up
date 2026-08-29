use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use derive_new::new;

#[derive(PartialEq, Eq, Debug, new)]
pub struct Map<K, V>
where
    K: Hash + Eq,
{
    values: HashMap<K, V>,
}

impl<K, V> Hash for Map<K, V>
where
    K: Hash + Eq,
    V: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut entry_hashes = Vec::with_capacity(self.values.len());

        for (key, value) in &self.values {
            let mut entry_hasher = DefaultHasher::new();

            key.hash(&mut entry_hasher);
            value.hash(&mut entry_hasher);

            entry_hashes.push(entry_hasher.finish());
        }

        // Make the result independent of HashMap iteration order.
        entry_hashes.sort_unstable();

        // Include the number of entries to distinguish map sizes.
        state.write_usize(entry_hashes.len());

        for hash in entry_hashes {
            state.write_u64(hash);
        }
    }
}

impl<K, V> From<HashMap<K, V>> for Map<K, V>
where
    K: Hash + Eq,
    V: Hash,
{
    fn from(value: HashMap<K, V>) -> Self {
        Self { values: value }
    }
}

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
    V: Hash,
{
    pub fn values(&self) -> &HashMap<K, V> {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.values
    }
}
