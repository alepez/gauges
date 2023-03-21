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

fn circle_stroke(radius: f64, angle: f64) -> (String, String) {
    let circumference = 2.0 * PI * radius;
    let offset = circumference / 4.0;
    let a = angle * radius;
    let b = circumference - a;
    (format!("{a},{b}"), format!("{offset}"))
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

    let value: f64 = value?;

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
    let angle = norm_value * 2.0 * PI;
    let (dash_array, dash_offset) = circle_stroke(radius, angle);

    cx.render(rsx! {
        div {
            div { "{text}" }
            div {
                svg {
                    width: width,
                    height: height,
                    circle {
                        fill: "none",
                        stroke: "#000000",
                        stroke_width: "20",
                        cx: center_x,
                        cy: center_y,
                        r: radius,
                        stroke_dasharray: "{dash_array}",
                        stroke_dashoffset: "{dash_offset}",
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
