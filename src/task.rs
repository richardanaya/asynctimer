use crate::waker::{noop_waker_ref, ArcWake};
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

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

pub fn make_waker<'a>(a: &'a Arc<Task>) -> &'a Waker {
    noop_waker_ref()
}
