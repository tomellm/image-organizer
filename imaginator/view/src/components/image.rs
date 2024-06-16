use std::{
    ffi::OsStr,
    hash::{DefaultHasher, Hash},
    ops::Deref,
};

use leptonic::prelude::*;
use leptos::*;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use types::image::Media;
use crate::util::*;

use crate::{
    api::{get_images, get_images_paginated},
    types::MediaPage,
};

#[component]
pub fn Image(image: Media) -> impl IntoView {
    let name = image.get_linkable_name();
    let link = get_actual_image_link(&name);

    let (selected, set_selected) = create_signal(false);
    let (selected_images, set_selected_images) = use_context::<
        (ReadSignal<Vec<Media>>, WriteSignal<Vec<Media>>)
    >().unwrap();

    create_effect(enclose!{(image) move |_| {
        match selected.get() {
            true => set_selected_images.update(enclose!{(image) move |images| images.push(image)}),
            false => set_selected_images.update(|images| {
                images.retain(|i| !i.eq(&image));
            })
        }
    }});


    view! {
        <div class="rounded-md bg-stone-50 w-full p-2 relative">
            <div
                class="h-52 rounded-md bg-cover bg-center bg-no-repeat block"
                style=format!("background-image: url({link})")
            >
            </div>
            <div class="py-2">
                <div>
                    {format!("{}", image.original_name)}
                </div>
                <Show 
                    when=move || image.datetime_created.is_some()
                    fallback=|| view!{<div>"Sorry no date here... ;("</div>}
                >
                    <div>
                        {format!("{:?}", image.datetime_created.unwrap())}
                    </div>
                </Show>
            </div>
            <div 
                class="
                    absolute top-0 left-0 bg-white rounded-md p-1 w-8 h-8 flex
                    justify-center items-center
                "
            >
                <input 
                    type="checkbox"
                    checked=move || selected.get()
                    on:change=move |_| set_selected.set(!selected.get())
                />
            </div>
        </div>
    }
}


fn get_actual_image_link(name: &String) -> String {
    let path = std::path::Path::new(&name);
    let ext = path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_uppercase();

    match ext.as_str() {
        "HEIC" => format!(
            "{}converted/{}.jpg",
            env!("BUCKET_URL"),
            path.file_stem().and_then(OsStr::to_str).unwrap()
        ),
        _ => format!("{}raw/{}", env!("BUCKET_URL"), name),
    }
}
