mod executor;
mod js_api;
mod sleep;

use crate::executor::Executor;
use crate::js_api::say_num;
use crate::sleep::{handle_timeout, sleep};

// This file represents the two entry points into our module

#[no_mangle]
pub fn main() -> () {
    // start an executor and give the first task
    Executor::spawn(async {
        say_num(1);
        sleep(1000).await;
        say_num(2);
        sleep(1000).await;
        say_num(3);
    });
}

#[no_mangle]
pub fn timeout_response(id: i32) -> () {
    // take the id given to us by window.setTimeout
    // and all the closure that wakes the right task
    handle_timeout(id);
}
