use std::f64::consts::PI;

use gauges::app::launch_app;
use gauges::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle_style = CircleGaugeStyle { radius: 50.0 };
    let arc_style = ArcGaugeStyle {
        radius: 50.0,
        begin_angle: (2.0 * PI) * (2.0 / 8.0),
        full_width: (2.0 * PI) * (4.0 / 8.0),
    };

    let dashboard = DashboardConfig::new(vec![
        GaugeInfo {
            id: SignalId::Num(1),
            style: GaugeStyle::Arc(arc_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("One".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(5),
            style: GaugeStyle::Circle(circle_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Two".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(100),
            style: GaugeStyle::Circle(circle_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Three".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(99),
            style: GaugeStyle::Circle(circle_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Four".to_owned()),
            },
        },
    ]);

    launch_app(dashboard);

    Ok(())
}
