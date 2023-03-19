use dioxus_desktop::Config as DesktopConfig;
use std::cell::Cell;

use dioxus::prelude::*;

use crate::net::{Receiver, Sender, Value};

pub fn launch_app(props: AppProps) {
    let window = dioxus_desktop::WindowBuilder::new().with_title("Gauges");
    let config = DesktopConfig::new().with_window(window);
    dioxus_desktop::launch_with_props(app, props, config);
}

pub struct AppProps {
    pub sender: Cell<Option<Sender>>,
    pub receiver: Cell<Option<Receiver>>,
}

fn arc_commands() -> String {
    let radius = 100;

    let begin_x = 50;
    let begin_y = 150;
    let rx = radius;
    let ry = radius;
    let angle = 0;
    let larg_arc_flag = 1;
    let sweep_flag = 0;
    let end_x = 150;
    let end_y = 50;

    format!(
        "M {begin_x} {begin_y} A {rx} {ry} {angle} {larg_arc_flag} {sweep_flag} {end_x} {end_y}"
    )
}

fn app(cx: Scope<AppProps>) -> Element {
    let value = use_state(&cx, || 0.0);
    let receiver = cx.props.receiver.take();

    let _ = use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![value];
        async move {
            if let Some(mut receiver) = receiver {
                while let Some(msg) = receiver.recv().await {
                    match msg {
                        Value::Float(x) => {
                            value.set(x);
                            println!("update value {:?}", msg);
                        }
                    }
                }
            }
        }
    });

    let commands = arc_commands();

    cx.render(rsx! {
        div {
            div { "{value}" }
            div {
                svg {
                    width: "300px",
                    height: "300px",
                    path {
                        fill: "none",
                        stroke: "#000000",
                        stroke_width: "20",
                        d: "{commands}"
                    }
                }
            }
        }
    })
}
