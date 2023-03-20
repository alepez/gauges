use crate::app::gauge;
use crate::core::Value;
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct DashboardProps {
    value: Value,
}

pub fn dashboard(cx: Scope<DashboardProps>) -> Element {
    let value = cx.props.value;

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
