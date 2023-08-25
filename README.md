This crate acts as a drop-in replacement for the [nannou](https://github.com/nannou-org/nannou) crate that provides an additional `Model` trait to allow persisting state easier.

In summary, the following code (with the original [nannou](https://github.com/nannou-org/nannou) crate):
```rust
fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view);
}
```
Is equivalent to (with this crate):
```rust
use nannou::prelude::*;

struct App {
    // state
}

impl nannou::Model for App {
    fn init(&mut self, app: &App) {
        // model init function
    }

    fn update(&mut self, app: &App, update: Update) {
        // update function
    }

    fn view(&self, app: &App, frame: Frame) {
        // view function
    }
}

fn main() {
    nannou::run(App { /* initial state */ }, |builder| builder);
}
```
where the original init/update/view methods can be inlined to allow access to `self`.  
See [examples/basic.rs](examples/basic.rs) for a runnable demo. 

## Installation
```toml
nannou = { package = "nannou_oop", git = "https://github.com/Nigecat/nannou-oop" }
```


