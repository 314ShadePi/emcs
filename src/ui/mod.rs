pub mod components;

use dioxus::{desktop::DesktopContext, prelude::*};

pub fn ui(cx: Scope) -> Element {
    let desktop = cx.consume_context::<DesktopContext>().unwrap();
    desktop.set_title("EMCS");
    cx.render(rsx! {
        div {
            class: "test",
            "test"
        }
    })
}
