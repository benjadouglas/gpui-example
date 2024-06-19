use std::{fmt::format, time::Duration};

use gpui::*;

struct Chat {
    text: Option<SharedString>,
    _subscription: Option<Subscription>,
}

struct NewMessage {
    message: String,
}

impl EventEmitter<NewMessage> for Chat {}

impl Chat {
    pub fn build_view(model: &Model<Chat>, cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let subscription = cx.subscribe(model, |this: &mut Chat, _emitter, event, cx| {
                println!("New Message: {}", event.message);
                this.text = Some(event.message.clone().into());
                cx.notify()
            });
            Self {
                text: None,
                _subscription: Some(subscription),
            }
        });
        view
    }

    pub fn build_model(cx: &mut WindowContext) -> Model<Chat> {
        let counter: Model<Chat> = cx.new_model(|_cx| Chat {
            text: None,
            _subscription: None,
        });
        counter
    }
}

impl Render for Chat {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.text.get_or_insert_with(|| "".into()).clone();
        div()
            .flex()
            .bg(rgb(0x333333))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Message: {}", text))
    }
}

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            let model = Chat::build_model(cx);
            let view = Chat::build_view(&model, cx);
            cx.spawn(|mut cx| async move {
                let mut count = 0;
                loop {
                    count += 1;
                    let _ = model.update(&mut cx, |_chat, cx| {
                        let message = format!("{:?}", count);
                        // chat.text = Some(message.into());
                        cx.emit(NewMessage {
                            message: message.clone(),
                        })
                    });
                    cx.background_executor()
                        .timer(Duration::from_millis(1000))
                        .await;
                }
            })
            .detach();
            view
        });
    });
}
