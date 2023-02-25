use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

/// A bidirectional HashMap.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BiMap<A, B>
where
    A: Eq + Hash,
    B: Eq + Hash,
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
}
