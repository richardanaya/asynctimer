use crate::js_api::timeout;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

struct SleepTracker {
    cur_id: i32,
    handlers: HashMap<i32, Arc<Mutex<Box<dyn Fn() -> () + Send + 'static>>>>,
}

fn sleep_handlers() -> &'static Mutex<SleepTracker> {
    static INSTANCE: OnceCell<Mutex<SleepTracker>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Mutex::new(SleepTracker {
            cur_id: 0,
            handlers: HashMap::new(),
        })
    })
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(millis: i32) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        let mut h = sleep_handlers().lock().unwrap();
        h.cur_id += 1;
        let id = h.cur_id;
        h.handlers.insert(
            id,
            Arc::new(Mutex::new(Box::new(move || {
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    std::mem::drop(shared_state);
                    waker.wake()
                }
            }))),
        );

        timeout(id, millis);

        TimerFuture { shared_state }
    }
}

pub fn sleep(millis: i32) -> TimerFuture {
    TimerFuture::new(millis)
}

pub fn handle_timeout(id: i32) -> () {
    // find the callback associated with the timeout id
    let h = sleep_handlers().lock().unwrap();
    let handler_ref = h.handlers.get(&id).unwrap().clone();
    std::mem::drop(h);
    // call the callback that will wake the task!
    let handler = handler_ref.lock().unwrap();
    handler()
}
