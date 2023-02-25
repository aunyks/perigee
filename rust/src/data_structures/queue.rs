use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Queue<T> {
    vec: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { vec: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Queue {
            vec: Vec::with_capacity(cap),
        }
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn enqueue(&mut self, item: T) {
        self.vec.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.vec.remove(0)
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.vec.first()
    }
}

impl<T: PartialEq> PartialEq for Queue<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.vec, &other.vec)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for Queue<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        PartialEq::eq(&self.vec, other)
    }
}

impl<T: Eq> Eq for Queue<T> {}
