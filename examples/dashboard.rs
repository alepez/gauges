use std::f64::consts::PI;

use gauges::app::launch_app_with_server;
use gauges::core::{SignalId, SignalInfo};
use gauges::net::Sender;
use gauges::prelude::*;

async fn fake_server(sender: Sender) {
    use gauges::core::*;

    let records = [
        (101, Value::None),
        (102, Value::Float(0.0)),
        (103, Value::Float(42.0)),
        (104, Value::Float(100.0)),
        (201, Value::None),
        (202, Value::Float(0.0)),
        (203, Value::Float(42.0)),
        (204, Value::Float(100.0)),
    ]
    .map(|(id, value)| NamedRecord {
        record: Record { value },
        id: SignalId::Num(id),
    });

    for record in records {
        sender.send(record).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle_style = CircleGaugeStyle { radius: 50.0 };
    let arc_style = ArcGaugeStyle {
        radius: 50.0,
        begin_angle: (2.0 * PI) * (3.0 / 8.0),
        full_width: (2.0 * PI) * (6.0 / 8.0),
    };

    let dashboard = DashboardConfig::new(vec![
        GaugeInfo {
            id: SignalId::Num(101),
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
            id: SignalId::Num(102),
            style: GaugeStyle::Arc(arc_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Two".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(103),
            style: GaugeStyle::Arc(arc_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Three".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(104),
            style: GaugeStyle::Arc(arc_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("Four".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(201),
            style: GaugeStyle::Circle(circle_style),
            range: Range {
                min: 0.0,
                max: 100.0,
            },
            signal: SignalInfo {
                name: Some("One".to_owned()),
            },
        },
        GaugeInfo {
            id: SignalId::Num(202),
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
            id: SignalId::Num(203),
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
            id: SignalId::Num(204),
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

    launch_app_with_server(dashboard, &fake_server);

    Ok(())
}
