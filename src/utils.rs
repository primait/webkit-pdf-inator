use futures::channel::oneshot;
use std::cell::Cell;
use std::rc::Rc;

pub struct RuntimeOneshotSender<T>(Rc<Cell<Option<oneshot::Sender<T>>>>);

impl<T> Clone for RuntimeOneshotSender<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> RuntimeOneshotSender<T> {
    fn new(s: oneshot::Sender<T>) -> Self {
        Self(Rc::new(Cell::new(Some(s))))
    }

    pub fn send(&self, value: T) -> Result<(), T> {
        if let Some(s) = self.0.take() {
            s.send(value)
        } else {
            tracing::warn!("Runtime oneshot::send called multiple times.");
            Err(value)
        }
    }
}

pub fn runtime_oneshot<T>() -> (RuntimeOneshotSender<T>, oneshot::Receiver<T>) {
    let (s, r) = oneshot::channel();
    let s = RuntimeOneshotSender::new(s);

    (s, r)
}
