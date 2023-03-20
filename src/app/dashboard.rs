use crate::app::gauge;
use crate::core::{Signals, Value};
use crate::{DashboardConfig, SignalId};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    signals: Signals,
    config: DashboardConfig,
}

fn extract_value(signals: &Signals, id: &SignalId) -> Value {
    let signal = signals.get(id);
    signal
        .and_then(|signal| signal.current_record.as_ref())
        .map(|r| r.value.clone())
        .unwrap_or(Value::None)
}

pub fn dashboard(cx: Scope<DashboardProps>) -> Element {
    let signals = cx.props.signals.clone();
    let ids = vec![1, 5, 100];
    let values: Vec<_> = ids
        .iter()
        .map(|&id| extract_value(&signals, &SignalId::Num(id)))
        .collect();

    cx.render(rsx! {
        div {
            class: "dashboard",
            h1 { "Dashboard" },
            for value in values.iter() {
                gauge::gauge {
                    radius: 50.,
                    value: value.clone(),
                }
            }
        }
    })
}
