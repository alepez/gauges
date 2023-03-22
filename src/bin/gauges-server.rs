use std::f64::consts::PI;

use gauges::app::launch_app;
use gauges::prelude::*;
use serde::{Deserialize, Serialize};

const CIRCLE_STYLE: GaugeStyle = GaugeStyle::Circle(CircleGaugeStyle { radius: 50.0 });

const ARC_STYLE: GaugeStyle = GaugeStyle::Arc(ArcGaugeStyle {
    radius: 50.0,
    begin_angle: (2.0 * PI) * (2.0 / 8.0),
    full_width: (2.0 * PI) * (4.0 / 8.0),
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
    id: SignalId,
    style: GaugeStyleId,
    range: Range,
    signal: SignalInfo,
}

impl From<GaugeConfig> for GaugeInfo {
    fn from(value: GaugeConfig) -> Self {
        GaugeInfo {
            id: value.id,
            style: match value.style {
                GaugeStyleId::Arc => ARC_STYLE,
                GaugeStyleId::Circle => CIRCLE_STYLE,
                GaugeStyleId::Protractor => PROTRACTOR_STYLE,
            },
            range: value.range,
            signal: value.signal,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    gauges: Vec<GaugeConfig>,
}

impl From<ServerConfig> for DashboardConfig {
    fn from(value: ServerConfig) -> Self {
        DashboardConfig {
            items: value.gauges.into_iter().map(|x| x.into()).collect()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        gauges: vec![
            GaugeConfig {
                id: SignalId::Num(0),
                style: GaugeStyleId::Arc,
                range: Range {
                    min: -20.0,
                    max: 40.0,
                },
                signal: SignalInfo {
                    name: Some("Temperature (C°)".to_owned()),
                },
            },
            GaugeConfig {
                id: SignalId::Num(1),
                style: GaugeStyleId::Circle,
                range: Range {
                    min: 0.0,
                    max: 100.0,
                },
                signal: SignalInfo {
                    name: Some("Speed (knots)".to_owned()),
                },
            },
            GaugeConfig {
                id: SignalId::Num(2),
                style: GaugeStyleId::Protractor,
                range: Range {
                    min: 0.0,
                    max: 360.0,
                },
                signal: SignalInfo {
                    name: Some("Angle (deg)".to_owned()),
                },
            },
        ],
    };

    launch_app(config.into());

    Ok(())
}
