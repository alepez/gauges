use gauges::app;
use gauges::{GaugeId, GaugeProps, GaugeStyle, Range};

#[tokio::main]
async fn main() {
    let _gauge = GaugeProps {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };

    dioxus_desktop::launch(app);
}
