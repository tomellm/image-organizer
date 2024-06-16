use std::{ops::Deref, hash::{Hash, DefaultHasher}, ffi::OsStr};

use leptonic::prelude::*;
use leptos::{logging::log, *, html::Div};
use leptos_query::QueryResult;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use types::image::Media;
use crate::components::image::*;
use leptos_icons::Icon;
use icondata as icon;

use crate::{api::{get_images, get_images_paginated}, types::MediaPage};

const IMAGE_BLOCKS: usize = 6;
const IMAGE_PAGE_SIZE: usize = IMAGE_BLOCKS * 3;

#[component]
pub fn ImagesPage() -> impl IntoView {
    let images = create_resource(
        || (),
        |_| async move { 
            MediaPage::from_res_opt(
                get_images_paginated(0, IMAGE_PAGE_SIZE).await
            )
        }
    );

    view! {
        <div>
            <Suspense fallback=move || view!{<ImagesLoading />}>
                { move || {
                    let val = images.get().unwrap_or(MediaPage::Error("Could not load first page ;(".to_string()));
                    view!{
                        <Images first_page=val />
                    }
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn ImagesLoading() -> impl IntoView {
    view! {
        <Skeleton height=Size::Auto/>
    }
}




#[component]
fn Images(first_page: MediaPage) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (pages, set_pages) = create_signal(vec![first_page]);

    let selected_images = create_signal(vec![] as Vec<Media>);
    provide_context(selected_images);

    let _ = use_infinite_scroll_with_options(
        el,
        move |state| async move {
            let len = pages.with_untracked(|d| d.len());
            let new_page = MediaPage::from_res_opt(
                get_images_paginated(len, IMAGE_PAGE_SIZE).await
            );
            set_pages.update(|mut pages| {
                let mut new_vec = vec![];
                new_vec.append(&mut pages);
                new_vec.push(new_page);
                *pages = new_vec;
            });
        },
        UseInfiniteScrollOptions::default()
            .distance(10.0)
            //.interval(10000.0),
    );

    view! {
        <Box>
            <Box>
                <div node_ref=el class="flex flex-col gap-2 p-4 w-screen h-screen m-auto overflow-y-scroll bg-gray-500/5 rounded">
                    <Grid spacing=Size::Em(0.6)>
                        <For
                            each=move || {pages.get().into_iter().enumerate().collect::<Vec<_>>()}
                            key=|page| page.0
                            let:page
                        >
                            <ImagePage page=page.1 />
                        </For>
                    </Grid>
                </div>
            </Box>
            <ImagesDrawer />
        </Box>
    }
}

#[component]
fn ImagesDrawer() -> impl IntoView {
    let (open_drawer, set_open_drawer) = create_signal(false);
    let (selected_images, set_selected_images) = use_context::<
        (ReadSignal<Vec<Media>>, WriteSignal<Vec<Media>>)
    >().unwrap();
    let not_show = create_memo(move |_| !open_drawer.get());
    let icon = icon::FaAngleLeftSolid;
    view! {
        <Drawer 
            side=DrawerSide::Right
            shown=not_show
            class="p-2 overflow-scroll absolute top-0 right-0 w-24"
        >
            <button on:click=move |_| set_open_drawer.set(true)>
                <Icon icon />
            </button>
        </Drawer>
        <Drawer 
            side=DrawerSide::Right
            shown=open_drawer
            class="p-2 h-auto overflow-scroll absolute top-0 right-0"
        >
            <div>
                <button on:click=move |_| set_open_drawer.set(false)>
                    "close"
                </button>
                <div>
                    <div class="flex font-xs wrap w-full">
                        {
                            move || selected_images.get()
                                .into_iter()
                                .map(|i| view!{<div>{i.original_name}</div>})
                                .collect_view()
                        }
                    </div>
                    {move || selected_images.get().len()}
                </div>
                <button on:click=move |_| set_selected_images.set(vec![])>
                    "clear images"
                </button>
            </div>
        </Drawer>
    }
}


#[component]
fn ImagePage(page: MediaPage) -> impl IntoView {

    match page {
        MediaPage::Final => view!{<FinalPage />},
        MediaPage::Error(err) => view!{<ErrorPage err />},
        MediaPage::Page(page) => view!{<ImageRows page />}
    }
}


#[component]
fn FinalPage() -> impl IntoView {
    view!{
        <Row>
            <Col>
                "There are no more images to show... Sorry! ;("
            </Col>
        </Row>
    }
}

#[component]
fn ErrorPage(err: String) -> impl IntoView {
    view!{
        <Row>
            <Col>
                {format!("There has been an error while loading this page: {}", err)}
            </Col>
        </Row>
    }
}


#[component]
fn ImageRows(page: Vec<Media>) -> impl IntoView {
    let rows = page 
        .chunks(IMAGE_BLOCKS)
        .map(|chunk| chunk.to_vec())
        .enumerate()
        .collect::<Vec<(usize, Vec<_>)>>();

    view! {
        <For
            each=move || rows.clone()
            key=|row| row.0
            let:row
        >
            <ImageRow row=row.1/>
        </For>
    }
}

#[component]
fn ImageRow(row: Vec<Media>) -> impl IntoView {
    view!{
        <Row>
            <For
                each=move || row.clone()
                key=|image| image.uuid 
                let:image
            >
                <Col md=2 sm=4 xs=6>
                    <Image image/>
                </Col>
            </For>
        </Row>
    }
}
