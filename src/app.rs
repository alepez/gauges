mod dashboard;
mod gauge;

use crate::core::Signals;
use crate::net::{channel, launch_server};
use crate::DashboardConfig;
use dioxus::prelude::*;
use dioxus_desktop::Config as DesktopConfig;

struct AppProps {
    dashboard: DashboardConfig,
}

pub fn launch_app(dashboard: DashboardConfig) {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    let props = AppProps { dashboard };
    dioxus_desktop::launch_with_props(app, props, config);
}

fn app(cx: Scope<AppProps>) -> Element {
    let signals = use_state(cx, || {
        let signals: Signals = cx.props.dashboard.clone().into();
        signals
    });

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
                    signals.modify(|signals| {
                        let mut signals = signals.clone();
                        signals.insert_named_record(record);
                        signals
                    });
                }
                signals.needs_update();
            }
        }
    });

    cx.render(rsx! {
        dashboard::dashboard {
            config: &cx.props.dashboard,
            signals: &signals.get(),
        }
    })
}
