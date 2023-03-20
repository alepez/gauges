use gauges::app::launch_app;

use gauges::{DashboardConfig, GaugeInfo, GaugeStyle, Range, SignalId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use dioxus::prelude::dioxus_hot_reload::Config as HotReloadConfig;
    // hot_reload_init!(HotReloadConfig::new().with_rebuild_command("cargo run"));

    let dashboard = DashboardConfig::new(vec![
        GaugeInfo {
            id: SignalId::Num(1),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 100 },
        },
        GaugeInfo {
            id: SignalId::Num(5),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 100 },
        },
        GaugeInfo {
            id: SignalId::Num(100),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 100 },
        },
        GaugeInfo {
            id: SignalId::Num(99),
            style: GaugeStyle::Circle,
            range: Range { min: 0, max: 100 },
        },
    ]);

    launch_app(dashboard);

    Ok(())
}
