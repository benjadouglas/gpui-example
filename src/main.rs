mod button;
mod chat;
use gpui::*;

fn main() {
    let app = App::new();
    // chat::run_app(app);
    button::run_app(app)
}
