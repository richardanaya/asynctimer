use crate::waker::noop_waker_ref;
use std::{
    future::Future,
    pin::Pin,
    sync::{mpsc::SyncSender, Arc, Mutex},
    task::Waker,
};

pub struct Task {
    pub future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
    pub task_sender: SyncSender<Arc<Task>>,
}

impl Task {
    pub fn get_waker(&self) -> &'static Waker {
        noop_waker_ref()
    }
}
