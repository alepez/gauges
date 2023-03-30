use std::f64::consts::PI;

use angle::Rad;
use serde::{Deserialize, Serialize};

use gauges::app::launch_app;
use gauges::prelude::*;

const CIRCLE_STYLE: GaugeStyle = GaugeStyle::Circle(CircleGaugeStyle { radius: 50.0 });

const ARC_STYLE: GaugeStyle = GaugeStyle::Arc(ArcGaugeStyle {
    radius: 50.0,
    begin_angle: Rad(225.0 / 180.0 * PI),
    full_width: Rad(270.0 / 180.0 * PI),
});

const PROTRACTOR_STYLE: GaugeStyle = GaugeStyle::Protractor(ProtractorGaugeStyle { radius: 50.0 });

#[derive(Serialize, Deserialize)]
enum GaugeStyleId {
    Arc,
    Circle,
    Protractor,
}

#[derive(Serialize, Deserialize)]
struct GaugeConfig {
    id: usize,
    style: GaugeStyleId,
    range: Range,
    signal: SignalInfo,
    #[serde(default)]
    format: GaugeTextFormat,
}

impl From<GaugeConfig> for GaugeInfo {
    fn from(value: GaugeConfig) -> Self {
        GaugeInfo {
            id: SignalId::Num(value.id as u32),
            style: match value.style {
                GaugeStyleId::Arc => ARC_STYLE,
                GaugeStyleId::Circle => CIRCLE_STYLE,
                GaugeStyleId::Protractor => PROTRACTOR_STYLE,
            },
            range: value.range,
            signal: value.signal,
            format: value.format,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NetworkConfig {
    addr: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            addr: "127.0.0.1:9999".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CommonDashboardConfig {
    age_indicator: bool,
}

impl Default for CommonDashboardConfig {
    fn default() -> Self {
        Self {
            age_indicator: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    #[serde(default)]
    network: NetworkConfig,
    gauges: Vec<GaugeConfig>,
    #[serde(default)]
    dashboard: CommonDashboardConfig,
}

impl From<ServerConfig> for DashboardConfig {
    fn from(value: ServerConfig) -> Self {
        DashboardConfig {
            items: value.gauges.into_iter().map(|x| x.into()).collect(),
            addr: value.network.addr,
            age_indicator: value.dashboard.age_indicator,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config();

    launch_app(config.into());

    Ok(())
}

fn read_config() -> ServerConfig {
    let content = std::fs::read_to_string("gauges.toml").expect("Could not read config");
    toml::from_str(&content).expect("Could not parse config")
}

#[allow(dead_code)]
fn write_config(config: &ServerConfig) {
    let content = toml::to_string(config).expect("Could encode config to TOML");
    std::fs::write("gauges.toml", content).expect("Could not write to file!");
}
