use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;

use crate::app::gauge::Gauge;
use crate::core::{Age, DashboardConfig, GaugeInfo};
use crate::core::{SignalId, Signals, Value};

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    signals: Signals,
    config: Rc<DashboardConfig>,
    updates_count: usize,
    age_indicator: bool,
}

fn extract_value(signals: &Signals, id: &SignalId) -> Option<(Value, Duration)> {
    let signal = signals.get(id)?;
    let (record, age) = (signal.signal().current_record.as_ref()?, signal.age());
    Some((record.value.clone(), age))
}

fn extract_info<'a>(signals: &Signals, info: &'a GaugeInfo) -> (&'a GaugeInfo, Value, Duration) {
    let x = extract_value(signals, &info.id);
    let value = x.as_ref().map(|x| x.0.clone()).unwrap_or(Value::None);
    let age = x.as_ref().map(|x| x.1).unwrap_or(Duration::MAX);
    (info, value, age)
}

fn calculate_age(show_age_indicator: bool, age: Duration) -> Age {
    if !show_age_indicator {
        Age::Unknown
    } else if age < Duration::from_millis(250) {
        Age::New
    } else if age < Duration::from_secs(10) {
        Age::Valid
    } else {
        Age::Expired
    }
}

#[allow(non_snake_case)]
pub fn Dashboard(cx: Scope<DashboardProps>) -> Element {
    let signals = &cx.props.signals;
    let items = &cx.props.config.items;
    let show_age_indicator = cx.props.age_indicator;
    let first = items.first().unwrap();
    let (item, value, age) = extract_info(signals, first);

    cx.render(rsx! {
        div {
            class: "dashboard",
                Gauge {
                    value: value
                    signal: item.signal.clone(),
                    style: item.style,
                    range: item.range,
                    format: item.format,
                    age: calculate_age(show_age_indicator, age),
                }
        }
    })
}
