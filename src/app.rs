mod dashboard;
mod gauge;

use crate::core::{Signals, Value};
use crate::net::{channel, launch_server};
use dioxus::prelude::*;
use dioxus_desktop::Config as DesktopConfig;
use std::cell::RefCell;

pub fn launch_app() {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    dioxus_desktop::launch_cfg(app, config);
}

fn app(cx: Scope) -> Element {
    let signals = use_state(cx, || RefCell::new(Signals::default()));
    let started = use_state(cx, || false);

    let (sender, mut receiver) = channel();

    if !started {
        started.set(true);
        cx.spawn(async move {
            launch_server(sender.clone()).await;
        });
    }

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![signals];
        async move {
            while let Some(record) = receiver.recv().await {
                {
                    let signals = signals.get();
                    if let Ok(mut signals) = signals.try_borrow_mut() {
                        signals.insert_named_record(record);
                    }
                }
                signals.needs_update();
            }
        }
    });

    // TODO
    let signals = signals.get().borrow();
    let signal = signals.get(&crate::core::Id::Num(1));
    let value = signal
        .and_then(|signal| signal.current_record.as_ref())
        .map(|r| r.value.clone())
        .unwrap_or(Value::None);

    cx.render(rsx! {
        dashboard::dashboard {
            value: value,
        }
    })
}
