# Async-Await in Web Assembly

This project shows a toy async-await executor written from scratch for web assembly. The goal is to make this real and see output:

```rust
pub fn main() -> () {
    Executor::spawn(async {
        say_num(1);
        sleep(1000).await;
        say_num(2);
        sleep(1000).await;
        say_num(3);
    });
}
```
See it working [here](https://richardanaya.github.io/asynctimer/)

This library only uses `once_cell` for global state and [woke](https://github.com/richardanaya/woke/) for waker creation (because the pointer magic for that is crazy).

Warning: this project builds only with Rust 1.39 and above

Limitations: this executor can only run a single top level future (i.e. no multiple spawns)
