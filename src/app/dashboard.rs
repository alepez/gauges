use crate::DashboardConfig;
use crate::app::gauge;
use crate::core::Value;
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    value: Value,
    config: DashboardConfig,
}

pub fn dashboard(cx: Scope<DashboardProps>) -> Element {
    let value = cx.props.value.clone();

    cx.render(rsx! {
        div {
            class: "dashboard",
            h1 { "Dashboard" },
            div {
                gauge::gauge {
                    radius: 50.,
                    value: value,
                }
            }
        }
    })
}
