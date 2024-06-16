mod api;
mod images;
mod types;
mod components;
mod util;

use images::*;
use leptonic::prelude::*;
use leptos::*;
use leptos_query::*;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

#[macro_use]
extern crate dotenv_codegen;

#[component]
pub fn App() -> impl IntoView {
    provide_query_client();
    view! {
        <Root default_theme=LeptonicTheme::default()>
            <ImagesPage />
        </Root>
    }
}

fn main() {
    fmt()
        .with_writer(
            // To avoide trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .init();

    mount_to_body(|| view! { <App /> });
}
