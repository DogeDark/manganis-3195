use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const OPTIONS: FolderAssetOptions = FolderAssetOptions::new();
const SOMETHING_FOLDER: Asset = asset!("/assets/something", OPTIONS);

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting app.");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { target: "_blank", href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { target: "_blank", href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
