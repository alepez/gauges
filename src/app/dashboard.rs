use crate::app::gauge;
use crate::core::{SignalId, Signals, Value};
use crate::DashboardConfig;
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct DashboardProps<'a> {
    signals: &'a Signals,
    config: &'a DashboardConfig,
}

fn extract_value(signals: &Signals, id: &SignalId) -> Value {
    let signal = signals.get(id);

    signal
        .and_then(|signal| signal.current_record.as_ref())
        .map(|r| r.value.clone())
        .unwrap_or(Value::None)
}

pub fn dashboard<'a>(cx: Scope<'a, DashboardProps<'a>>) -> Element {
    let signals = &cx.props.signals;
    let items = &cx.props.config.items;

    cx.render(rsx! {
        div {
            class: "dashboard",
            h1 { "Dashboard" },
            for item in items.iter() {
                gauge::gauge {
                    radius: 50.,
                    value: extract_value(&signals, &item.id),
                    info: item.signal.clone(),
                }
            }
        }
    })
}
