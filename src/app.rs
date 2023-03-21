mod dashboard;
mod gauge;

use std::rc::Rc;

use crate::core::Signals;
use crate::net::{channel, launch_server};
use crate::DashboardConfig;
use dioxus::prelude::*;
use dioxus_desktop::Config as DesktopConfig;
use dashboard::Dashboard;

struct AppProps {
    dashboard: Rc<DashboardConfig>,
}

pub fn launch_app(dashboard: DashboardConfig) {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    let dashboard = Rc::new(dashboard);
    let props = AppProps { dashboard };
    dioxus_desktop::launch_with_props(app, props, config);
}

fn app(cx: Scope<AppProps>) -> Element {
    let signals = use_ref(cx, || {
        let signals: Signals = cx.props.dashboard.as_ref().clone().into();
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
                signals.write().insert_named_record(record);
                signals.needs_update();
            }
        }
    });

    cx.render(rsx! {
        Dashboard {
            config: cx.props.dashboard.clone(),
            signals: signals.read().clone(),
        }
    })
}
