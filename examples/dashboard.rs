use std::f64::consts::PI;

use angle::Rad;

use gauges::app::launch_app_with_server;
use gauges::core::{SignalId, SignalInfo};
use gauges::net::Sender;
use gauges::prelude::*;

struct ExampleGauge {
    value: Value,
    style: GaugeStyle,
    min: f64,
    max: f64,
    name: &'static str,
    precision: usize,
}

const CIRCLE_STYLE: GaugeStyle = GaugeStyle::Circle(CircleGaugeStyle { radius: 50.0 });

const ARC_STYLE: GaugeStyle = GaugeStyle::Arc(ArcGaugeStyle {
    radius: 80.0,
    begin_angle: Rad(225.0 / 180.0 * PI),
    full_width: Rad(270.0 / 180.0 * PI),
});

const PROTRACTOR_STYLE: GaugeStyle = GaugeStyle::Protractor(ProtractorGaugeStyle { radius: 50.0 });

const EXAMPLES: [ExampleGauge; 12] = [
    ExampleGauge {
        value: Value::None,
        style: ARC_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Arctognathus murryi",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(0.0),
        style: ARC_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Brachylophosaurus canadensis",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(50.0),
        style: ARC_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Coelophysis bauri",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(100.0),
        style: ARC_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Diplodocus carnegii",
        precision: 3,
    },
    ExampleGauge {
        value: Value::None,
        style: CIRCLE_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Edmontosaurus annectens",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(0.0),
        style: CIRCLE_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Fukuiraptor kitadaniensis",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(50.0),
        style: CIRCLE_STYLE,
        min: 0.0,
        max: 100.0,
        name: "Guanlong wucaii",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(90.0),
        style: PROTRACTOR_STYLE,
        min: 0.0,
        max: 360.0,
        name: "Hesperornis regalis",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(180.0),
        style: PROTRACTOR_STYLE,
        min: 0.0,
        max: 360.0,
        name: "Hesperornis regalis",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(0.0),
        style: PROTRACTOR_STYLE,
        min: 0.0,
        max: 360.0,
        name: "Hesperornis regalis",
        precision: 3,
    },
    ExampleGauge {
        value: Value::Float(-90.0),
        style: PROTRACTOR_STYLE,
        min: 0.0,
        max: 360.0,
        name: "Hesperornis regalis",
        precision: 0,
    },
    ExampleGauge {
        value: Value::Float(0.0),
        style: ARC_STYLE,
        min: -100.0,
        max: 100.0,
        name: "Arctognathus murryi",
        precision: 1,
    },
];

async fn fake_server(sender: Sender) {
    use gauges::core::*;

    let records = EXAMPLES
        .iter()
        .enumerate()
        .map(|(id, example)| NamedRecord {
            record: Record {
                value: example.value.clone(),
            },
            id: SignalId::Num(id as u32),
        });

    for record in records {
        sender.send(record).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dashboard_items = EXAMPLES.iter().enumerate().map(|(id, example)| GaugeInfo {
        id: SignalId::Num(id as u32),
        style: example.style,
        range: Range {
            min: example.min,
            max: example.max,
        },
        signal: SignalInfo {
            name: Some(example.name.to_owned()),
        },
        format: GaugeTextFormat {
            precision: example.precision,
        },
    });

    let dashboard = DashboardConfig::new(dashboard_items.collect());

    launch_app_with_server(dashboard, &fake_server);

    Ok(())
}
