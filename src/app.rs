mod dashboard;
mod gauge;

use crate::core::Value;
use crate::net::{channel, launch_server};
use dioxus::prelude::*;
use dioxus_desktop::Config as DesktopConfig;

pub fn launch_app() {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    dioxus_desktop::launch_cfg(app, config);
}

fn app(cx: Scope) -> Element {
    let value = use_state(cx, || Value::None);
    let started = use_state(cx, || false);

    let (sender, mut receiver) = channel();

    if !started {
        started.set(true);
        cx.spawn(async move {
            launch_server(sender.clone()).await;
        });
    }

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![value];
        async move {
            while let Some(x) = receiver.recv().await {
                value.set(x);
            }
        }
    });

    cx.render(rsx! {
        dashboard::dashboard {
            value: value.get().clone()
        }
    })
}
