use std::f64::consts::PI;

use angle::Angle;
use angle::Rad;
use dioxus::prelude::*;

use crate::core::OnOffGaugeStyle;
use crate::core::{
    Age, ArcGaugeStyle, CircleGaugeStyle, GaugeStyle, GaugeTextFormat, ProtractorGaugeStyle, Range,
    SignalInfo, Value,
};

#[derive(PartialEq, Props)]
pub struct GaugeProps {
    value: Value,
    style: GaugeStyle,
    range: Range,
    signal: SignalInfo,
    format: GaugeTextFormat,
    age: Age,
}

fn circle_stroke(width: Rad<f64>, offset: Rad<f64>) -> (String, String) {
    let offset = -offset + Rad::half_pi();
    let a = width;
    let b = Rad::two_pi() - width;

    let offset = offset.value();
    let a = a.value();
    let b = b.value();

    (format!("{a},{b}"), format!("{offset}"))
}

fn format(value: &Value, options: &GaugeTextFormat) -> String {
    match value {
        Value::Float(x) => format!("{0:.1$}", x, options.precision),
        Value::Percent(x) => format!("{0:.1$}%", x, options.precision),
        Value::None => "N/A".to_string(),
        Value::OnOff(false) => "Off".to_string(),
        Value::OnOff(true) => "On".to_string(),
    }
}

fn class_from_age(age: &Age) -> &'static str {
    match age {
        Age::New => "age-new",
        Age::Valid => "age-valid",
        Age::Expired => "age-expired",
        Age::Unknown => "age-unknown",
    }
}

#[allow(non_snake_case)]
pub fn Gauge(cx: Scope<GaugeProps>) -> Element {
    let inner_style: ExtArcGaugeStyle = match cx.props.style {
        GaugeStyle::Arc(style) => style.into(),
        GaugeStyle::Circle(style) => style.into(),
        GaugeStyle::Protractor(style) => style.into(),
        GaugeStyle::OnOff(style) => style.into(),
    };

    let inner = ExtArcGauge(cx, inner_style);

    let info = cx.props.signal.name.as_deref().unwrap_or("-");
    let text = format(&cx.props.value, &cx.props.format);

    let inner_width = inner_style.width;
    let inner_height = inner_style.height;

    let info_width = inner_width;
    let info_height = 30.0;

    let full_width = inner_width;
    let full_height = inner_height + info_height;

    let indicator_age_class = class_from_age(&cx.props.age);

    cx.render(rsx! {
        div {
            class: "gauge",
            margin: "10px",
            width: "{full_width}px",
            height: "{full_height}px",
            div {
                class: "gauge-info-wrapper",
                padding: "5px",
                width: "{info_width}px",
                height: "{info_height}px",
                div {
                    class: "gauge-info",
                    "{info}"
                }
            }
            div {
                class: "gauge-value-text-wrapper",
                width: "{inner_width}px",
                height: "{inner_height}px",
                div {
                    class: "gauge-value-text",
                    "{text}"
                }
            }
            div {
                class: "gauge-inner-wrapper",
                inner
            }
            div {
                class: "gauge-update-indicator {indicator_age_class}",
            }
        }
    })
}

#[allow(non_snake_case)]
fn ExtArcGauge(cx: Scope<GaugeProps>, style: ExtArcGaugeStyle) -> Element {
    let value = match cx.props.value {
        Value::None => None,
        Value::Float(x) => Some(x),
        Value::Percent(x) => Some(x),
        Value::OnOff(x) => Some(if x { 1.0 } else { 0.0 }),
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
        width,
        height,
    } = style;

    let value: f64 = value?;

    let min_value = cx.props.range.min;
    let max_value = cx.props.range.max;

    let range_width = max_value - min_value;

    let norm_value = match normalize_policy {
        NormalizePolicy::Clamp => {
            let clamped = value.clamp(min_value, max_value);
            (clamped - min_value) / range_width
        }
        NormalizePolicy::Mod => {
            let z = value.rem_euclid(range_width);
            (z - min_value) / range_width
        }
        NormalizePolicy::Bool => value,
    };

    let center_x = width / 2.;
    let center_y = width / 2.;

    let real_width = full_width * norm_value;

    let arrow_width = Rad(0.05);
    let arrow_angle = begin_angle + real_width - (arrow_width / 2.0);

    let show_arrow = arrow == ArrowType::OnlyArrow;
    let show_real = arrow == ArrowType::NoArrow;

    cx.render(rsx! {
        svg {
            class: "gauge-inner",
            width: width,
            height: height,
            Arc {
                class: "arc-background",
                center_x: center_x,
                center_y: center_y,
                radius: radius,
                begin_angle: begin_angle,
                width: full_width,
                stroke_width: 20.0,
            }
            if show_real {
                rsx!(Arc {
                    class: "arc-foreground",
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
                    class: "arc-arrow",
                    center_x: center_x,
                    center_y: center_y,
                    radius: radius,
                    begin_angle: arrow_angle,
                    width: arrow_width,
                    stroke_width: 30.0,
                })
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
    begin_angle: Rad<f64>,
    width: Rad<f64>,
    radius: f64,
    class: &'static str,
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
        class,
        stroke_width,
    } = *cx.props;

    let (dash_array, dash_offset) = circle_stroke(width, begin_angle);

    cx.render(rsx! {
        circle {
            fill: "none",
            class: class,
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
    Bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ArrowType {
    NoArrow,
    OnlyArrow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ExtArcGaugeStyle {
    radius: f64,
    width: f64,
    height: f64,
    begin_angle: Rad<f64>,
    full_width: Rad<f64>,
    arrow: ArrowType,
    normalize_policy: NormalizePolicy,
}

impl From<ArcGaugeStyle> for ExtArcGaugeStyle {
    fn from(val: ArcGaugeStyle) -> Self {
        ExtArcGaugeStyle {
            radius: val.radius,
            begin_angle: val.begin_angle,
            full_width: val.full_width,
            arrow: ArrowType::NoArrow,
            normalize_policy: NormalizePolicy::Clamp,
            width: val.radius * 3.0,
            height: val.radius * 3.0,
        }
    }
}

impl From<CircleGaugeStyle> for ExtArcGaugeStyle {
    fn from(val: CircleGaugeStyle) -> Self {
        ExtArcGaugeStyle {
            radius: val.radius,
            begin_angle: Rad(0.0),
            full_width: Rad::two_pi(),
            arrow: ArrowType::NoArrow,
            normalize_policy: NormalizePolicy::Clamp,
            width: val.radius * 3.0,
            height: val.radius * 3.0,
        }
    }
}

impl From<ProtractorGaugeStyle> for ExtArcGaugeStyle {
    fn from(val: ProtractorGaugeStyle) -> Self {
        ExtArcGaugeStyle {
            radius: val.radius,
            begin_angle: Rad(0.0),
            full_width: Rad::two_pi(),
            arrow: ArrowType::OnlyArrow,
            normalize_policy: NormalizePolicy::Mod,
            width: val.radius * 3.0,
            height: val.radius * 3.0,
        }
    }
}

impl From<OnOffGaugeStyle> for ExtArcGaugeStyle {
    fn from(val: OnOffGaugeStyle) -> Self {
        ExtArcGaugeStyle {
            radius: val.radius,
            begin_angle: Rad(0.0),
            full_width: Rad::two_pi(),
            arrow: ArrowType::NoArrow,
            normalize_policy: NormalizePolicy::Bool,
            width: val.radius * 3.0,
            height: val.radius * 3.0,
        }
    }
}
