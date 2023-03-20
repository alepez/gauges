use gauges::app::launch_app;

use gauges::core::{SignalId, SignalInfo};
use gauges::{DashboardConfig, GaugeInfo, GaugeStyle, Range};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use dioxus::prelude::dioxus_hot_reload::Config as HotReloadConfig;
    // hot_reload_init!(HotReloadConfig::new().with_rebuild_command("cargo run"));

    let dashboard = DashboardConfig::new(vec![
        GaugeInfo {
            id: SignalId::Num(1),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 100 },
            signal: SignalInfo {
                name: Some("One".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(5),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 360 },
            signal: SignalInfo {
                name: Some("One".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(100),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 1000 },
            signal: SignalInfo {
                name: Some("One".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(99),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 100 },
            signal: SignalInfo {
                name: Some("One".to_owned()),
            },
        },
    ]);

    launch_app(dashboard);

    Ok(())
}
