use dioxus_desktop::Config as DesktopConfig;
use std::cell::Cell;

use dioxus::prelude::*;

use crate::net::{Receiver, Sender, Value};

pub fn launch_app(props: AppProps) {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    dioxus_desktop::launch_with_props(app, props, config);
}

pub struct AppProps {
    pub sender: Cell<Option<Sender>>,
    pub receiver: Cell<Option<Receiver>>,
}

fn app(cx: Scope<AppProps>) -> Element {
    let value = use_state(&cx, || 0.0);
    let receiver = cx.props.receiver.take();

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![value];
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.recv().await {
                    match msg {
                        Value::Float(x) => {
                            value.set(x);
                            println!("update value {:?}", msg);
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            div { "{value}" }
        }
    })
}
