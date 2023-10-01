#[allow(unused)]

use std::env;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::{*, logging::*};
use leptos_meta::*;
use leptos_router::*;
use reqwest::Url;
use types::database::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/view.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (images, set_images) 
        : (ReadSignal<Vec<ImageData>>, WriteSignal<Vec<ImageData>>) 
        = create_signal(Vec::new());

    view! {
        <button on:click=move |_| {
            spawn_local(async move{
                set_images.set(get_images().await.unwrap());
            });
        }>"Load!"</button>
        <div>
            {move || images.get().into_iter().map(|i| {
                view!{
                    <ImageDataView image_data=i />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn ImageDataView(
    image_data: ImageData
) -> impl IntoView {
    view!{
        <div>
            <div>
                {image_data.group_id}
            </div>
            <div>
                {image_data.file_name}
            </div> 
            <div>
                {image_data.org_name}
            </div> 
            <div>
                {format!("{:?}", image_data.datetime)}
            </div> 
            <div>
                {image_data.file_size}
            </div>
        </div>
    }
}

#[server(Images, "/api")]
pub async fn get_images() -> Result<Vec<ImageData>, ServerFnError> {
    let url = format!("http://{}/", env::var("API_URL").unwrap());

    let url = Url::parse(&*url).expect("creating the url failed");
    let res = reqwest::get(url)
        .await.expect("the request failed")
        .json::<Vec<ImageData>>().await.unwrap();
    Ok(res)
}
