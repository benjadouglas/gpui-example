use gpui::*;

struct Counter {
    count: usize,
}

struct Change {
    increment: usize,
}

impl EventEmitter<Change> for Counter {}

impl Render for Counter {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let counter: Model<Counter> = cx.new_model(|_cx| Counter { count: 0 });
        let subscriber = cx.new_model(|cx: &mut ModelContext<Counter>| {
            cx.subscribe(&counter, |subscriber, _emitter, event, _cx| {
                subscriber.count += event.increment;
            })
            .detach();

            Counter {
                count: counter.read(cx).count,
            }
        });
        let inside_div = div()
            .flex()
            .w_48()
            .h_32()
            .justify_start()
            .bg(rgb(0x000000))
            .rounded_md()
            .text_color(rgb(0xF6F6F7))
            .items_center()
            .justify_center()
            .hover(|s| s.bg(rgb(0x1B1B1B)).cursor_pointer())
            .child(format!("{}", subscriber.read(cx).count))
            .on_mouse_down(MouseButton::Left, move |_, cx| {
                counter.update(cx, |counter, cx| {
                    counter.count += 1;
                    cx.notify();
                    cx.emit(Change { increment: 1 });
                    println!("{}", format!("{}", subscriber.read(cx).count))
                })
            });

        div()
            .flex()
            .size_full()
            .p_2()
            .bg(rgb(0x367EA1))
            .items_start()
            .child(inside_div)
    }
}

pub fn run_app(app: gpui::App) {
    app.run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Counter { count: 0 })
        });
    });
}
