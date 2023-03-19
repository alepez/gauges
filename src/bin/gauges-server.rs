use gauges::app::{launch_app, AppProps};
use gauges::net::{channel, launch_server};
use gauges::{GaugeId, GaugeInfo, GaugeStyle, Range};
use std::cell::Cell;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use dioxus::prelude::dioxus_hot_reload::Config as HotReloadConfig;
    // hot_reload_init!(HotReloadConfig::new().with_rebuild_command("cargo run"));

    let _gauge = GaugeInfo {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };

    let (sender, receiver) = channel();

    launch_server(sender.clone());

    let props = AppProps {
        sender: Cell::new(Some(sender)),
        receiver: Cell::new(Some(receiver)),
    };

    launch_app(props);

    Ok(())
}
