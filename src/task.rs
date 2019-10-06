use crate::waker::{ArcWake,make_waker};
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
        make_waker(self)
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}