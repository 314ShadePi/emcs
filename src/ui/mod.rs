pub mod components;

use c314_utils::prelude::ToStaticStr;
use dioxus::{desktop::use_window, prelude::*};

use crate::cli::Cli;

pub struct UiProps {
    pub cli: Cli,
    pub css: String,
}

pub fn ui(cx: Scope<UiProps>) -> Element {
    let desktop = use_window(&cx);
    desktop.set_title("EMCS");
    let cli = format!("{:#?}", &cx.props.cli);
    let css = cx.props.css.clone();
    let css = css.to_static_str();
    cx.render(rsx! {
        div {
            class: "test red",
            style { [css] }
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
