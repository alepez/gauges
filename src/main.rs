use dioxus::prelude::dioxus_hot_reload::Config;
use dioxus::prelude::hot_reload_init;
use gauges::app;
use gauges::{GaugeId, GaugeProps, GaugeStyle, Range};

fn main() {
    hot_reload_init!(Config::new().with_rebuild_command("cargo run"));

    let _gauge = GaugeProps {
        id: GaugeId::from("foo".to_owned()),
        range: Range { min: 0, max: 100 },
        style: GaugeStyle::Bar,
    };

    dioxus_desktop::launch(app);
}
