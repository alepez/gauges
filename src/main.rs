use std::cell::Cell;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use dioxus::prelude::dioxus_hot_reload::Config as HotReloadConfig;
use dioxus::prelude::hot_reload_init;
use dioxus_desktop::Config as DesktopConfig;
use gauges::{app, AppProps};
use gauges::{GaugeId, GaugeProps, GaugeStyle, Range};
use tokio::net::TcpListener;
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hot_reload_init!(HotReloadConfig::new().with_rebuild_command("cargo run"));

    let _gauge = GaugeProps {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };

    let (sender, receiver) = unbounded_channel::<f64>();
    let other = sender.clone();

    tokio::spawn(async move {
        let bind = SocketAddr::from_str("127.0.0.1:9999").unwrap();
        let listener = TcpListener::bind(bind).await.unwrap();

        let (socket, _) = listener.accept().await.unwrap();
        let mut server = Framed::new(socket, LinesCodec::new_with_max_length(1024));
        while let Some(Ok(line)) = server.next().await {
            let parsed: Option<f64> = line.parse().ok();
            if let Some(x) = parsed {
                other.send(x).unwrap();
            }
        }
    });

    let props = AppProps {
        sender: Cell::new(Some(sender)),
        receiver: Cell::new(Some(receiver)),
    };

    dioxus_desktop::launch_with_props(app, props, DesktopConfig::default());

    Ok(())
}
