use dioxus::prelude::*;

use crate::net::Value;

#[derive(PartialEq, Props)]
pub struct GaugeProps {
    value: Value,
}

fn arc_commands() -> String {
    let radius = 100;

    let begin_x = 50;
    let begin_y = 150;
    let rx = radius;
    let ry = radius;
    let angle = 0;
    let larg_arc_flag = 1;
    let sweep_flag = 0;
    let end_x = 150;
    let end_y = 50;

    format!(
        "M {begin_x} {begin_y} A {rx} {ry} {angle} {larg_arc_flag} {sweep_flag} {end_x} {end_y}"
    )
}

pub fn gauge(cx: Scope<GaugeProps>) -> Element {
    let commands = arc_commands();
    let text = cx.props.value.to_string();

    cx.render(rsx! {
        div {
            div { "{text}" }
            div {
                svg {
                    width: "300px",
                    height: "300px",
                    path {
                        fill: "none",
                        stroke: "#000000",
                        stroke_width: "20",
                        d: "{commands}"
                    }
                }
            }
        }
    })
}
