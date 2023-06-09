use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_desktop::Config as DesktopConfig;

use crate::app::dashboard::Dashboard;

use crate::core::DashboardConfig;
use crate::core::Signals;
use crate::net::{channel, Sender};

struct AppProps<F, T>
where
    F: Fn(Sender, String) -> T + 'static,
    T: std::future::Future<Output = ()>,
{
    dashboard: Rc<DashboardConfig>,
    launch_server: &'static F,
}

pub fn launch_app(dashboard: DashboardConfig) {
    launch_app_with_server(dashboard, &crate::net::launch_server)
}

#[cfg(debug_assertions)]
fn custom_head() -> String {
    r#"<link rel="stylesheet" href="assets/style.css" />"#.to_owned()
}

#[cfg(not(debug_assertions))]
fn custom_head() -> String {
    format!(r#"<style>{}</style>"#, include_str!("../assets/style.css"))
}

pub fn launch_app_with_server<F, T>(dashboard: DashboardConfig, launch_server: &'static F)
where
    F: Fn(Sender, String) -> T + 'static,
    T: std::future::Future<Output = ()> + 'static, // TODO Why this needs to be static?
{
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");

    let config = DesktopConfig::new()
        .with_window(window)
        .with_custom_head(custom_head());

    let dashboard = Rc::new(dashboard);
    let props = AppProps {
        dashboard,
        launch_server,
    };
    dioxus_desktop::launch_with_props(app, props, config);
}

fn app<F, T>(cx: Scope<AppProps<F, T>>) -> Element
where
    F: Fn(Sender, String) -> T + 'static,
    T: std::future::Future<Output = ()>,
{
    let signals = use_ref(cx, || {
        let signals: Signals = cx.props.dashboard.as_ref().clone().into();
        signals
    });

    let updates_count = use_state(cx, || 0);

    let started = use_state(cx, || false);

    let (sender, mut receiver) = channel();

    // TODO Why this does not work but the other works?
    // let launch_server = &cx.props.launch_server;
    let launch_server: &'static F = cx.props.launch_server;
    let addr = cx.props.dashboard.addr.clone();

    if !started {
        started.set(true);
        cx.spawn(async move {
            launch_server(sender.clone(), addr).await;
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

    let show_age_indicator = cx.props.dashboard.age_indicator;

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![updates_count];
        async move {
            if show_age_indicator {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    updates_count += 1;
                }
            }
        }
    });

    cx.render(rsx! {
        Dashboard {
            config: cx.props.dashboard.clone(),
            signals: signals.read().clone(),
            updates_count: *updates_count.get(),
            age_indicator: show_age_indicator,
        }
    })
}
