use crate::data_structures::BiMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// A [BiMap](crate::data_structures::BiMap)
/// for naming [Rapier](https://rapier.rs) RigidBodyHandles
/// and ColliderHandles.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NamedHandleMap<T>
where
    T: Eq + Hash,
{
    inner: BiMap<String, T>,
}

impl<T> NamedHandleMap<T>
where
    T: Eq + Hash,
{
    pub fn handle_with_name(&self, name: &String) -> Option<&T> {
        self.inner.get(name)
    }

    pub fn name_of_handle(&self, handle: &T) -> Option<&String> {
        self.inner.get_reverse(handle)
    }

    pub fn insert(&mut self, name: String, handle: T) {
        self.inner.insert(name, handle);
    }
}
