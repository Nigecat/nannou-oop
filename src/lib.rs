pub use nannou::*;
use std::cell::RefCell;

pub trait Model {
    fn init(&mut self, _app: &App) {}

    fn update(&mut self, app: &App, update: nannou::prelude::Update);

    fn view(&self, app: &App, frame: Frame);
}

// Shadow [`nannou::App`]
/// Begin building the `App`.
pub fn app<T, F>(model: T, setup: Option<F>)
where
    T: Model,
    F: Fn(app::Builder<(), Event>) -> app::Builder<(), Event>,
{
    let model: Box<dyn Model> = Box::new(model);
    let model: &'static mut dyn Model = unsafe { std::mem::transmute(Box::leak(model)) };

    thread_local! {
        static DATA: RefCell<Option<&'static mut dyn Model>> = RefCell::new(None);
    }

    DATA.with(move |data| {
        *data.borrow_mut() = Some(model);
    });

    fn init(app: &App) {
        DATA.with(|data| {
            data.borrow_mut().as_mut().unwrap().init(app);
        });
    }

    fn update(app: &App, _: &mut (), update: nannou::prelude::Update) {
        DATA.with(|data| {
            data.borrow_mut().as_mut().unwrap().update(app, update);
        });
    }

    fn view(app: &App, _: &(), frame: Frame) {
        DATA.with(|data| {
            data.borrow().as_ref().unwrap().view(app, frame);
        });
    }

    let mut app = ::nannou::app(init).update(update).simple_window(view);
    if let Some(setup) = setup {
        app = setup(app);
    }
    app.run();

    // Ensure we don't trigger a memory leak
    DATA.with(|data| {
        data.replace(None);
    });
}
