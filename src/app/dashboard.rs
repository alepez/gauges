use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;

use crate::app::gauge::Gauge;
use crate::core::{Age, DashboardConfig, GaugeInfo, Signal};
use crate::core::{SignalId, Signals, Value};

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    signals: Signals,
    config: Rc<DashboardConfig>,
}

fn extract_value(signals: &Signals, id: &SignalId) -> Option<(Value, Duration)> {
    let signal = signals.get(id)?;
    let (record, age) = (signal.signal().current_record.as_ref()?, signal.age());
    Some((record.value.clone(), age))
}

fn foo<'a>(signals: &Signals, info: &'a GaugeInfo) -> (&'a GaugeInfo, Value, Age) {
    let x = extract_value(signals, &info.id);
    let age = x.as_ref().map(|x| x.1).unwrap_or(Duration::MAX);
    let value = x.as_ref().map(|x| x.0.clone()).unwrap_or(Value::None);

    let age = if age < Duration::from_millis(100) {
        Age::New
    } else if age < Duration::from_secs(10) {
        Age::Valid
    } else {
        Age::Expired
    };

    (info, value, age)
}

#[allow(non_snake_case)]
pub fn Dashboard(cx: Scope<DashboardProps>) -> Element {
    let signals = &cx.props.signals;
    let items = &cx.props.config.items;

    cx.render(rsx! {
        div {
            class: "dashboard",
            for (item, value, age) in items.iter().map(|x| foo(signals, x)) {
                Gauge {
                    value: value
                    signal: item.signal.clone(),
                    style: item.style,
                    range: item.range,
                    format: item.format,
                    age: age,
                }
            }
        }
    })
}
