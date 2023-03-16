use std::cell::Cell;

use dioxus::prelude::*;

pub struct AppProps {
    pub sender: Cell<Option<tokio::sync::mpsc::UnboundedSender<f64>>>,
    pub receiver: Cell<Option<tokio::sync::mpsc::UnboundedReceiver<f64>>>,
}

pub fn app(cx: Scope<AppProps>) -> Element {
    let value = use_state(&cx, || 0.0);
    let receiver = cx.props.receiver.take();

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![value];
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.recv().await {
                    value.set(msg);
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Value: {value}" }
        }
    })
}
