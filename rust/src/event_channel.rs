use crossbeam::channel::{Receiver, Sender, TryRecvError};

pub struct EventChannel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Default for EventChannel<T> {
    fn default() -> Self {
        Self::with_capacity(1)
    }
}

impl<T> EventChannel<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let (sender, receiver) = crossbeam::channel::bounded(cap);

        Self { sender, receiver }
    }

    pub fn send(&self, message: T) {
        if let Err(err) = self.sender.send(message) {
            log::warn!("Could not send previous message: {:?}", err);
        }
    }

    pub fn clone_sender(&self) -> Sender<T> {
        self.sender.clone()
    }

    pub fn get_message(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn eviscerate(&self) -> Result<(), TryRecvError> {
        while !self.receiver.is_empty() {
            match self.get_message() {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(())
    }
}
