use std::f64::consts::PI;

use dioxus::prelude::*;

use crate::{
    core::{SignalInfo, Value},
    ArcGaugeStyle, CircleGaugeStyle, GaugeStyle, Range,
};

#[derive(PartialEq, Props)]
pub struct GaugeProps {
    value: Value,
    style: GaugeStyle,
    range: Range,
    signal: SignalInfo,
}

fn circle_stroke(radius: f64, angle: f64, offset: f64) -> (String, String) {
    let circumference = 2.0 * PI * radius;
    let offset = circumference * 0.25 + offset * radius;
    let a = angle * radius;
    let b = circumference - a;
    (format!("{a},{b}"), format!("{offset}"))
}

#[allow(non_snake_case)]
pub fn Gauge(cx: Scope<GaugeProps>) -> Element {
    let inner = match cx.props.style {
        GaugeStyle::Arc(style) => ArcGauge(cx, style),
        GaugeStyle::Circle(style) => CircleGauge(cx, style),
    };

    let text = cx.props.value.to_string();

    cx.render(rsx! {
        div {
            class: "gauge",
            position: "relative",
            width: "150px", // TODO
            height: "150px", // TODO
            display: "inline-block",
            div { 
                class: "gauge-text",
                position: "absolute",
                width: "150px", // TODO
                text_align: "center",
                vertical_align: "middle",
                line_height: "150px", // TODO
                "{text}" 
            }
            div {
                class: "gauge-inner",
                inner
            }
        }
    })
}

#[allow(non_snake_case)]
fn ArcGauge(cx: Scope<GaugeProps>, style: ArcGaugeStyle) -> Element {
    let value = match cx.props.value {
        Value::None => None,
        Value::Float(x) => Some(x),
    };

    if value.is_none() {
        return NoneGauge(cx);
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

    let begin_angle = style.begin_angle;

    let full_width = style.full_width;
    let real_width = norm_value * full_width;

    cx.render(rsx! {
        div {
            div {
                svg {
                    width: width,
                    height: height,
                    Arc {
                        color: "#000000",
                        center_x: center_x,
                        center_y: center_y,
                        radius: radius,
                        begin_angle: begin_angle,
                        width: full_width,
                    }
                    Arc {
                        color: "#00FF00",
                        center_x: center_x,
                        center_y: center_y,
                        radius: radius,
                        begin_angle: begin_angle,
                        width: real_width,
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
fn CircleGauge(cx: Scope<GaugeProps>, style: CircleGaugeStyle) -> Element {
    let style = ArcGaugeStyle {
        radius: style.radius,
        begin_angle: 0.0,
        full_width: 2.0 * PI,
    };

    ArcGauge(cx, style)
}

#[allow(non_snake_case)]
fn NoneGauge(cx: Scope<GaugeProps>) -> Element {
    cx.render(rsx! {
        div {
        }
    })
}

#[derive(PartialEq, Props)]
struct ArcProps {
    center_x: f64,
    center_y: f64,
    begin_angle: f64,
    width: f64,
    radius: f64,
    color: &'static str,
}

#[allow(non_snake_case)]
fn Arc(cx: Scope<ArcProps>) -> Element {
    let ArcProps {
        center_x,
        center_y,
        begin_angle,
        width,
        radius,
        color,
    } = *cx.props;

    let (dash_array, dash_offset) = circle_stroke(radius, width, begin_angle);

    cx.render(rsx! {
        circle {
            fill: "none",
            stroke: "{color}",
            stroke_width: "20",
            cx: center_x,
            cy: center_y,
            r: radius,
            stroke_dasharray: "{dash_array}",
            stroke_dashoffset: "{dash_offset}",
        }
    })
}
