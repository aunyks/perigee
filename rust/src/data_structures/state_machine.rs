use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct StateMachine<T> {
    state: T,
}

impl<T> StateMachine<T> {
    pub fn new(initial_state: T) -> Self {
        Self {
            state: initial_state,
        }
    }

    pub fn transition_to(&mut self, new_state: T) {
        self.state = new_state;
    }

    pub fn current_state(&self) -> &T {
        &self.state
    }
}

impl<T> Deref for StateMachine<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.current_state()
    }
}

impl<T: PartialEq> PartialEq for StateMachine<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.current_state(), other.current_state())
    }
}

impl<T: PartialEq> PartialEq<T> for StateMachine<T> {
    fn eq(&self, other: &T) -> bool {
        PartialEq::eq(self.current_state(), other)
    }
}

impl<T: Eq> Eq for StateMachine<T> {}
