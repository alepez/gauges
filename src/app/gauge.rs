use std::f64::consts::PI;

use dioxus::prelude::*;

use crate::core::{
    ArcGaugeStyle, CircleGaugeStyle, GaugeStyle, ProtractorGaugeStyle, Range, SignalInfo, Value,
};

#[derive(PartialEq, Props)]
pub struct GaugeProps {
    value: Value,
    style: GaugeStyle,
    range: Range,
    signal: SignalInfo,
}

fn circle_stroke(width: f64, offset: f64) -> (String, String) {
    let offset = -offset + (PI / 2.0);
    let a = width;
    let b = (2.0 * PI) - width;
    (format!("{a},{b}"), format!("{offset}"))
}

#[allow(non_snake_case)]
pub fn Gauge(cx: Scope<GaugeProps>) -> Element {
    let inner = match cx.props.style {
        GaugeStyle::Arc(style) => ExtArcGauge(cx, style.into()),
        GaugeStyle::Circle(style) => ExtArcGauge(cx, style.into()),
        GaugeStyle::Protractor(style) => ExtArcGauge(cx, style.into()),
    };

    let info = cx.props.signal.name.as_deref().unwrap_or("-");
    let text = cx.props.value.to_string();

    cx.render(rsx! {
        div {
            class: "gauge",
            // width: "150px", // TODO
            // height: "180px", // TODO
            div {
                class: "gauge-info-wrapper",
                // width: "150px", // TODO
                // height: "30px", // TODO
                div {
                    class: "gauge-info",
                    "{info}"
                }
            }
            div {
                class: "gauge-value-text-wrapper",
                div {
                    class: "gauge-value-text",
                    // width: "150px", // TODO
                    // line_height: "150px", // TODO
                    "{text}"
                }
            }
            div {
                class: "gauge-inner-wrapper",
                inner
            }
        }
    })
}

#[allow(non_snake_case)]
fn ExtArcGauge(cx: Scope<GaugeProps>, style: ExtArcGaugeStyle) -> Element {
    let value = match cx.props.value {
        Value::None => None,
        Value::Float(x) => Some(x),
    };

    if value.is_none() {
        return NoneGauge(cx);
    }

    let ExtArcGaugeStyle {
        radius,
        begin_angle,
        full_width,
        arrow,
        normalize_policy,
    } = style;

    let value: f64 = value?;

    let min_value = cx.props.range.min;
    let max_value = cx.props.range.max;

    let norm_value = match normalize_policy {
        NormalizePolicy::Clamp => {
            let clamped = value.clamp(min_value, max_value);
            (clamped / (max_value - min_value)) - min_value
        }
        NormalizePolicy::Mod => {
            let z = value.rem_euclid(max_value - min_value);
            (z - min_value) / (max_value - min_value)
        }
    };

    let width = radius * 3.;
    let height = width;
    let center_x = width / 2.;
    let center_y = width / 2.;

    let real_width = norm_value * full_width;

    let arrow_width = 0.05;
    let arrow_angle = begin_angle + real_width - (arrow_width / 2.0);

    let show_arrow = arrow == ArrowType::OnlyArrow;
    let show_real = arrow == ArrowType::NoArrow;

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
                        stroke_width: 20.0,
                    }
                    if show_real {
                        rsx!(Arc {
                            color: "#00FF00",
                            center_x: center_x,
                            center_y: center_y,
                            radius: radius,
                            begin_angle: begin_angle,
                            width: real_width,
                            stroke_width: 20.0,
                        })
                    }
                    if show_arrow {
                        rsx!(Arc {
                            color: "#FFFFFF",
                            center_x: center_x,
                            center_y: center_y,
                            radius: radius,
                            begin_angle: arrow_angle,
                            width: arrow_width,
                            stroke_width: 30.0,
                        })
                    }
                }
            }
        }
    })
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
    stroke_width: f64,
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
        stroke_width,
    } = *cx.props;

    let (dash_array, dash_offset) = circle_stroke(width, begin_angle);

    cx.render(rsx! {
        circle {
            fill: "none",
            stroke: "{color}",
            stroke_width: stroke_width,
            cx: center_x,
            cy: center_y,
            r: radius,
            path_length: 2.0 * PI,
            stroke_dasharray: "{dash_array}",
            stroke_dashoffset: "{dash_offset}",
        }
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NormalizePolicy {
    Clamp,
    Mod,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ArrowType {
    NoArrow,
    OnlyArrow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ExtArcGaugeStyle {
    radius: f64,
    begin_angle: f64,
    full_width: f64,
    arrow: ArrowType,
    normalize_policy: NormalizePolicy,
}

impl Into<ExtArcGaugeStyle> for ArcGaugeStyle {
    fn into(self) -> ExtArcGaugeStyle {
        ExtArcGaugeStyle {
            radius: self.radius,
            begin_angle: self.begin_angle,
            full_width: self.full_width,
            arrow: ArrowType::NoArrow,
            normalize_policy: NormalizePolicy::Clamp,
        }
    }
}

impl Into<ExtArcGaugeStyle> for CircleGaugeStyle {
    fn into(self) -> ExtArcGaugeStyle {
        ExtArcGaugeStyle {
            radius: self.radius,
            begin_angle: 0.0,
            full_width: 2.0 * PI,
            arrow: ArrowType::NoArrow,
            normalize_policy: NormalizePolicy::Clamp,
        }
    }
}

impl Into<ExtArcGaugeStyle> for ProtractorGaugeStyle {
    fn into(self) -> ExtArcGaugeStyle {
        ExtArcGaugeStyle {
            radius: self.radius,
            begin_angle: 0.0,
            full_width: 2.0 * PI,
            arrow: ArrowType::OnlyArrow,
            normalize_policy: NormalizePolicy::Mod,
        }
    }
}
