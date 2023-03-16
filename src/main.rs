use gauges::{GaugeProps, GaugeId, GaugeStyle, Range};

#[tokio::main]
async fn main() {
    println!("Hello, gauges!");

    let gauge = GaugeProps {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };
}
