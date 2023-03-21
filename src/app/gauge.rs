use std::f64::consts::PI;

use dioxus::prelude::*;

use crate::{
    core::{SignalInfo, Value},
    CircleGaugeStyle, GaugeStyle, Range,
};

#[derive(PartialEq, Props)]
pub struct GaugeProps {
    value: Value,
    style: GaugeStyle,
    range: Range,
    signal: SignalInfo,
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
    let inner = match cx.props.style {
        GaugeStyle::Circle(style) => gauge_circle(cx, style),
    };

    cx.render(rsx! {
        div {
            inner
        }
    })
}

fn gauge_circle(cx: Scope<GaugeProps>, style: CircleGaugeStyle) -> Element {
    let value = match cx.props.value {
        Value::None => None,
        Value::Float(x) => Some(x),
    };

    if value.is_none() {
        return gauge_none(cx);
    }

    let value = value.unwrap();

    let angle_offset_rad = -PI / 2.0;

    let min_value = cx.props.range.min;
    let max_value = cx.props.range.max;

    let clamped = value.clamp(min_value, max_value);
    let range_size = max_value - min_value;
    let norm_value = (clamped / range_size) - min_value;

    let radius = style.radius;
    let width = radius * 3.;
    let height = width;
    let center_x = width / 2.;
    let center_y = width / 2.;
    let text = cx.props.value.to_string();
    let begin_angle = 0.0 + angle_offset_rad;
    let end_angle = (norm_value * 2.0 * PI) + angle_offset_rad;
    let commands = arc_commands(center_x, center_y, radius, begin_angle, end_angle);

    cx.render(rsx! {
        div {
            div { "{text}" }
            div {
                svg {
                    width: width,
                    height: height,
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
