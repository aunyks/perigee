use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug, Default)]
pub struct Shared<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Deref for Shared<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: PartialEq> PartialEq<RefCell<T>> for Shared<T> {
    fn eq(&self, other: &RefCell<T>) -> bool {
        &*self.inner == other
    }
}
