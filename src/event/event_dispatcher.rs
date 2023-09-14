use super::{EventHandler, EventHandlerId};
use parking_lot::Mutex;

pub struct EventDispatcher<T> {
    handlers: Mutex<Vec<EventHandler<T>>>,
    added_queue: Mutex<Vec<EventHandler<T>>>,
    removed_queue: Mutex<Vec<EventHandlerId>>,
}

impl<T> EventDispatcher<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new().into(),
            added_queue: Vec::new().into(),
            removed_queue: Vec::new().into(),
        }
    }

    pub fn add_handler(&self, handler: EventHandler<T>) {
        match self.handlers.try_lock() {
            Some(mut handlers) => {
                handlers.push(handler);
            }
            None => {
                self.added_queue.lock().push(handler);
            }
        }
    }

    pub fn remove_handler(&self, handler_id: EventHandlerId) {
        match self.handlers.try_lock() {
            Some(mut handlers) => {
                if let Some(index) = handlers
                    .iter()
                    .position(|handler| handler.id() == handler_id)
                {
                    handlers.swap_remove(index);
                }
            }
            None => {
                self.removed_queue.lock().push(handler_id);
            }
        }
    }

    pub fn dispatch(&self, event: &T) {
        let mut handlers = if let Some(handlers) = self.handlers.try_lock() {
            handlers
        } else {
            return;
        };

        for handler in handlers.iter_mut() {
            handler.call(event);
        }

        for removed in self.removed_queue.lock().drain(..) {
            if let Some(index) = handlers.iter().position(|handler| handler.id() == removed) {
                handlers.swap_remove(index);
            }
        }

        handlers.extend(self.added_queue.lock().drain(..));
    }
}
