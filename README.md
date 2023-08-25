This crate acts as a drop-in replacement for the [nannou](https://github.com/nannou-org/nannou) crate that provides an additional `Model` trait to allow persisting state easier.

In summary, the following code (with the original [nannou](https://github.com/nannou-org/nannou) crate):
```rust
nannou::app(model)
    .update(update)
    .simple_window(view);
```
Is equivalent to (with this crate):
```rust
struct App {
    // state
}

impl nannou::Model for App {
    fn init(&mut self, app: &App) {
        init(app);
    }

    fn update(&mut self, app: &App, update: nannou::prelude::Update) {
        update(app, update);
    }

    fn view(&self, app: &App, frame: Frame) {
        view(app, frame);
    }
}
```
where the original init/update/view methods can be inlined to allow access to `self`.

## Installation
```toml
nannou = { package = "nannou_oop", git = "https://github.com/Nigecat/nannou-oop" }
```


