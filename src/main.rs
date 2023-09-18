#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App::new("Dioxus Desktop", 800, 600));
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, World!"
        }
    })
}