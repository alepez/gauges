use crate::app::gauge;
use crate::core::{Signal, Signals, Value};
use crate::{DashboardConfig};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    signals: Signals,
    config: DashboardConfig,
}

fn extract_value(signal: &Signal) -> Value {
    signal
        .current_record
        .as_ref()
        .map(|r| r.value.clone())
        .unwrap_or(Value::None)
}

pub fn dashboard(cx: Scope<DashboardProps>) -> Element {
    let signals = cx.props.signals.clone();
    let values = signals.iter().map(extract_value);

    cx.render(rsx! {
        div {
            class: "dashboard",
            h1 { "Dashboard" },
            for value in values {
                gauge::gauge {
                    radius: 50.,
                    value: value,
                }
            }
        }
    })
}
