extern "C" {
    fn _say_num(num: i32);
    fn _timeout(id: i32, millis: i32);
}

// This file represents the javascript functions that given to our module

pub fn say_num(num: i32) {
    unsafe { _say_num(num) }
}

pub fn timeout(id: i32, millis: i32) {
    unsafe { _timeout(id, millis) }
}
