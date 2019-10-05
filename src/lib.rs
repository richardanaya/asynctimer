use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

struct SleepTracker {
    cur_id: i32,
    handlers: HashMap<i32, Box<dyn Fn() -> () + Send + 'static>>,
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
            Box::new(move || {
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            }),
        );

        unsafe { _timeout(id, millis) }

        TimerFuture { shared_state }
    }
}

fn sleep(millis: i32) -> TimerFuture {
    TimerFuture::new(millis)
}

extern "C" {
    fn _say_num(num: i32);
    fn _timeout(id: i32, millis: i32);
}

fn say_num(num: i32) {
    unsafe { _say_num(num) }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<Box<dyn Future<Output = ()>>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = Box::new(future);
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                /*let context = &mut Context::from_waker((&task);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }*/
            }
        }
    }
}

fn run(future: impl Future<Output = ()> + 'static + Send) {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(future);

    drop(spawner);

    executor.run();
}

#[no_mangle]
pub fn main() -> () {
    run(async {
        say_num(1);
        sleep(1000).await;
        say_num(2);
        sleep(1000).await;
        say_num(3);
    });
}

#[no_mangle]
pub fn timeout_response(id: i32) -> () {
    say_num(id);
}
