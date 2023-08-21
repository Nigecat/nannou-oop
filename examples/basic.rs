use nannou_oop as nannou;

struct App {
    color: nannou::prelude::rgb::Srgb<u8>,
}

impl nannou::Model for App {
    fn view(&self, _app: &nannou::App, frame: nannou::Frame) {
        frame.clear(self.color);
    }
}

fn main() {
    let app = App {
        color: nannou::prelude::CYAN,
    };
    nannou::app(app, |builder| builder);
}
