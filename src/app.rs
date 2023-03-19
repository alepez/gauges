mod gauge;

use dioxus_desktop::Config as DesktopConfig;
use std::cell::Cell;

use dioxus::prelude::*;

use crate::net::{channel, launch_server, Receiver, Sender, Value};

pub fn launch_app() {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    dioxus_desktop::launch_cfg(app, config);
}

pub struct AppProps {
    pub sender: Cell<Option<Sender>>,
    pub receiver: Cell<Option<Receiver>>,
}

fn app(cx: Scope) -> Element {
    let value = use_state(cx, || Value::None);
    let started = use_state(cx, || false);

    let (sender, mut receiver) = channel();

    if !started {
        started.set(true);
        cx.spawn(async move {
            println!("Launch server");
            launch_server(sender.clone()).await;
        });
    }

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![value];
        async move {
            while let Some(x) = receiver.recv().await {
                value.set(x);
                println!("update value {:?}", x);
            }
        }
    });

    cx.render(rsx! {
        div {
            gauge::gauge {
                radius: 50.,
                value: *value.get()
            }
        }
    })
}
