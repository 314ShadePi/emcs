pub mod components;

use dioxus::{desktop::use_window, prelude::*};

use crate::cli::Cli;

pub struct UiProps {
    pub cli: Cli,
}

pub fn ui(cx: Scope<UiProps>) -> Element {
    let desktop = use_window(&cx);
    desktop.set_title("EMCS");
    let cli = format!("{:#?}", &cx.props.cli);
    cx.render(rsx! {
        div {
            class: "test",
            "test"
            pre {
                code {
                    class: "language-json",
                    "{cli}"
                }
            }
        }
    })
}
