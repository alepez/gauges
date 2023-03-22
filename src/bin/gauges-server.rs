use std::f64::consts::PI;

use gauges::app::launch_app;
use gauges::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CIRCLE_STYLE: GaugeStyle = GaugeStyle::Circle(CircleGaugeStyle { radius: 50.0 });

    const ARC_STYLE: GaugeStyle = GaugeStyle::Arc(ArcGaugeStyle {
        radius: 50.0,
        begin_angle: (2.0 * PI) * (2.0 / 8.0),
        full_width: (2.0 * PI) * (4.0 / 8.0),
    });

    const PROTRACTOR_STYLE: GaugeStyle =
        GaugeStyle::Protractor(ProtractorGaugeStyle { radius: 50.0 });

    let dashboard = DashboardConfig::new(vec![
        GaugeInfo {
            id: SignalId::Num(0),
            style: ARC_STYLE,
            range: Range {
                min: -20.0,
                max: 40.0,
            },
            signal: SignalInfo {
                name: Some("Temperature (C°)".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(1),
            style: CIRCLE_STYLE,
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Speed (knots)".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(2),
            style: PROTRACTOR_STYLE,
            range: Range {
                min: 0.0,
                max: 360.0,
            },
            signal: SignalInfo {
                name: Some("Angle (deg)".to_owned()),
            },
        },
    ]);

    launch_app(dashboard);

    Ok(())
}
