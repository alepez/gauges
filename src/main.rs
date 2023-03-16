use std::cell::Cell;
use std::time::Duration;

use dioxus::prelude::dioxus_hot_reload::Config as HotReloadConfig;
use dioxus::prelude::hot_reload_init;
use dioxus_desktop::Config as DesktopConfig;
use gauges::{app, AppProps};
use gauges::{GaugeId, GaugeProps, GaugeStyle, Range};
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() {
    hot_reload_init!(HotReloadConfig::new().with_rebuild_command("cargo run"));

    let _gauge = GaugeProps {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };

    let (sender, receiver) = unbounded_channel::<f64>();
    let other = sender.clone();

    tokio::spawn(async move {
        let mut count = 0.0;
        loop {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            count += 1.0;
            other.send(count).unwrap();
        }
    });

    let props = AppProps {
        sender: Cell::new(Some(sender)),
        receiver: Cell::new(Some(receiver)),
    };

    dioxus_desktop::launch_with_props(app, props, DesktopConfig::default());
}
