use crate::data_structures::BiMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// A [BiMap](crate::data_structures::BiMap)
/// for naming [Rapier](https://rapier.rs) RigidBodyHandles
/// and ColliderHandles.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NamedHandleMap<T>
where
    T: Eq + Hash + Copy,
{
    inner: BiMap<String, T>,
}

impl<T> NamedHandleMap<T>
where
    T: Eq + Hash + Copy,
{
    pub fn handle_with_name(&self, name: impl Into<String>) -> Option<&T> {
        self.inner.get(&name.into())
    }

    pub fn name_of_handle(&self, handle: &T) -> Option<&String> {
        self.inner.get_reverse(handle)
    }

    pub fn insert(&mut self, name: impl Into<String>, handle: T) {
        self.inner.insert(name.into(), handle);
    }

    pub fn remove_by_name(&mut self, name: impl Into<String>) -> bool {
        self.inner.remove(&name.into())
    }

    pub fn remove_by_handle(&mut self, handle: &T) -> bool {
        self.inner.remove_reverse(handle)
    }

    pub fn rename_handle(
        &mut self,
        old_name: impl Into<String>,
        new_name: impl Into<String>,
    ) -> bool {
        let old_name = old_name.into();
        let handle = match self.handle_with_name(&old_name) {
            Some(handle) => *handle,
            None => return false,
        };
        self.remove_by_name(old_name);
        self.insert(new_name, handle);
        return true;
    }

    pub fn swap_named_handle(&mut self, old_handle: &T, new_handle: T) -> bool {
        let name = match self.name_of_handle(old_handle) {
            Some(name) => name.clone(),
            None => return false,
        };
        self.remove_by_handle(old_handle);
        self.insert(name, new_handle);
        return true;
    }
}
