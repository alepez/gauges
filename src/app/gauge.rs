use std::f64::consts::PI;

use dioxus::prelude::*;

use crate::net::Value;

#[derive(PartialEq, Props)]
pub struct GaugeProps {
    value: Value,
    radius: f64,
}

fn arc_commands(x: f64, y: f64, radius: f64, begin_angle: f64, end_angle: f64) -> String {
    let (begin_x, begin_y) = polar_to_cartesian(x, y, radius, end_angle);
    let (end_x, end_y) = polar_to_cartesian(x, y, radius, begin_angle);
    let rx = radius;
    let ry = radius;
    let angle = 0;
    let large_arc_flag = if end_angle - begin_angle <= PI { 0 } else { 1 };
    let sweep_flag = 0;

    format!(
        "M {begin_x} {begin_y} A {rx} {ry} {angle} {large_arc_flag} {sweep_flag} {end_x} {end_y}"
    )
}

pub fn gauge(cx: Scope<GaugeProps>) -> Element {
    match cx.props.value {
        Value::None => gauge_none(cx),
        Value::Float(_) => gauge_arc(cx),
    }
}

fn gauge_arc(cx: Scope<GaugeProps>) -> Element {
    let radius = cx.props.radius;
    let width = radius * 3.;
    let center_x = width / 2.;
    let center_y = width / 2.;
    let text = cx.props.value.to_string();
    let commands = arc_commands(center_x, center_y, radius, 0., 4.71);

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

fn gauge_none(cx: Scope<GaugeProps>) -> Element {
    let text = cx.props.value.to_string();
    cx.render(rsx! {
        div {
            div { "{text}" }
        }
    })
}

fn polar_to_cartesian(center_x: f64, center_y: f64, radius: f64, angle_rad: f64) -> (f64, f64) {
    (
        center_x + radius * angle_rad.cos(),
        center_y + radius * angle_rad.sin(),
    )
}
