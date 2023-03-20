use std::cell::RefCell;
use std::rc::Rc;

use crate::app::gauge;
use crate::core::{SignalId, Signals, Value};
use crate::DashboardConfig;
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    signals: Rc<RefCell<Signals>>,
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
    let signals = signals.borrow();
    let items = &cx.props.config.items;

    // dbg!(&items);

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
