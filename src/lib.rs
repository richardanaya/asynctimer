mod executor;
mod js_api;
mod sleep;

use crate::executor::run;
use crate::js_api::say_num;
use crate::sleep::sleep;

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
