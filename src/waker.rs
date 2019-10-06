use core::cell::UnsafeCell;
use std::sync::Arc;
use std::task::{RawWaker, RawWakerVTable, Waker};

unsafe fn noop_clone(_data: *const ()) -> RawWaker {
    noop_raw_waker()
}

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(core::ptr::null(), &NOOP_WAKER_VTABLE)
}

unsafe fn noop(_data: *const ()) {}

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

pub fn noop_waker_ref() -> &'static Waker {
    thread_local! {
        static NOOP_WAKER_INSTANCE: UnsafeCell<Waker> =
            UnsafeCell::new(noop_waker());
    }
    NOOP_WAKER_INSTANCE.with(|l| unsafe { &*l.get() })
}

pub trait ArcWake: Send + Sync {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }
    fn wake_by_ref(arc_self: &Arc<Self>);
}

pub fn make_waker(_: &impl ArcWake) -> &'static Waker {
    noop_waker_ref()
}
