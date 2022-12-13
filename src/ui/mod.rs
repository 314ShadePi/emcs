pub mod components;

use dioxus::{desktop::use_window, prelude::*};

pub fn ui(cx: Scope) -> Element {
    let desktop = use_window(&cx);
    desktop.set_title("EMCS");
    cx.render(rsx! {
        div {
            class: "test",
            "test"
        }
    })
}
