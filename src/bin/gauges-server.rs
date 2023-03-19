use dioxus_desktop::Config as DesktopConfig;
use gauges::app::{app, AppProps};
use gauges::net::launch_server;
use gauges::{GaugeId, GaugeProps, GaugeStyle, Range};
use std::cell::Cell;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use dioxus::prelude::dioxus_hot_reload::Config as HotReloadConfig;
    // hot_reload_init!(HotReloadConfig::new().with_rebuild_command("cargo run"));

    let _gauge = GaugeProps {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };

    let (sender, receiver) = unbounded_channel::<f64>();

    launch_server(sender.clone());

    let props = AppProps {
        sender: Cell::new(Some(sender)),
        receiver: Cell::new(Some(receiver)),
    };

    launch_app(props);

    Ok(())
}

fn launch_app(props: AppProps) {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    dioxus_desktop::launch_with_props(app, props, config);
}
