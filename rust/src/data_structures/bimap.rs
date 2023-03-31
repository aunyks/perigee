use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

/// A bidirectional HashMap.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BiMap<A, B>
where
    A: Eq + Hash + ?Sized,
    B: Eq + Hash + ?Sized,
{
    left_to_right: HashMap<Rc<A>, Rc<B>>,
    right_to_left: HashMap<Rc<B>, Rc<A>>,
}

impl<A, B> BiMap<A, B>
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    pub fn new() -> Self {
        BiMap {
            left_to_right: HashMap::new(),
            right_to_left: HashMap::new(),
        }
    }

    pub fn insert(&mut self, a: A, b: B) {
        let a = Rc::new(a);
        let b = Rc::new(b);
        self.left_to_right.insert(a.clone(), b.clone());
        self.right_to_left.insert(b, a);
    }

    pub fn get(&self, a: &A) -> Option<&B> {
        self.left_to_right.get(a).map(Deref::deref)
    }

    pub fn get_reverse(&self, b: &B) -> Option<&A> {
        self.right_to_left.get(b).map(Deref::deref)
    }

    pub fn remove(&mut self, a: &A) -> bool {
        self.left_to_right
            .remove(a)
            .and_then(|right| self.right_to_left.remove(&*right))
            .is_some()
    }

    pub fn remove_reverse(&mut self, b: &B) -> bool {
        self.right_to_left
            .remove(b)
            .and_then(|left| self.left_to_right.remove(&*left))
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_and_retrieval() {
        let mut map = BiMap::new();
        map.insert("hi", 2);
        assert_eq!(map.get(&"hi"), Some(&2));
        assert_eq!(map.get_reverse(&2), Some(&"hi"));
    }

    #[test]
    fn insertion_and_removal() {
        let mut map = BiMap::new();

        map.insert("hi", 2);
        assert_eq!(map.get(&"hi"), Some(&2));
        assert_eq!(map.remove(&"hi"), true);
        assert_eq!(map.get(&"hi"), None);

        map.insert("bye", 3);
        assert_eq!(map.get_reverse(&3), Some(&"bye"));
        assert_eq!(map.remove_reverse(&3), true);
        assert_eq!(map.get_reverse(&3), None);
    }
}
