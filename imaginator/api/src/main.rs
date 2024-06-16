mod db;
mod routes;
mod state;
mod util;



use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use std::{process::Command, ffi::OsStr, error::Error};
    use std::process::Child;
    use cloud_storage::Client;
    use dotenv::dotenv;
    use routes::*;

    use axum::{
        routing::{get, post},
        Router,
    };
    use leptos::{*, LeptosOptions};
    use imager_api::app::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use imager_api::fileserv::file_and_error_handler;

    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt,
        util::SubscriberInitExt,
        Layer,
    };

    use sqlx::MySqlPool;
    use std::{sync::Arc, env};
    use tower::ServiceBuilder;
    use tower_http::{trace::TraceLayer, cors::{CorsLayer, Any}};

    pub struct Server {
        child: Child
    } 

    impl Server {
        pub fn start(
            database_path: &OsStr
        ) -> Result<Self, Box<dyn Error>> {
            let child = Command::new("indradb-server")
                .args([OsStr::new("rocksdb"), database_path])
                .env("RUST_BACKTRACE", "1")
                .spawn()?;
            Ok(Server { child })
        }
    }

    impl Drop for Server {
        fn drop(&mut self) {
            unsafe {
                libc::kill(self.child.id() as i32, libc::SIGTERM);
            }
            self.child.wait().unwrap();
        }
    }



    #[tokio::main]
    async fn main() {



        dotenv().expect("Could not initialize dotenv crate");

        init_tracing();

        let _server = Server::start(OsStr::new("../persist/indradb/indradb.rdb"));

        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(Any);

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(App);

        let state = state::ApiState::new(leptos_options).await;

        let app = Router::new()
            /*.route("/clear", post(clear))
            .route("/media/:id", get(get_one))
            .route("/media", get(get_many))
            .route("/medias", get(get_all))
            .route("/images", get(get_all_images))
            .route("/images/paginated", get(get_images_paginated))
            .route("/images", post(save_one))
            .route("/count-query", get(count_query))
            .route("/create_graph", get(create_graph))
            .route("/properties", get(properties))
            .route("/delete", get(delete_all))
            .route("/images/read", get(read_images))
            .route("/images/read/save", get(read_and_save_images))
            .route("/images/bytes", get(read_image_stream))*/
            .layer(ServiceBuilder::new()
                   .layer(cors)
                   //.layer(http_tracing)
            )
            .leptos_routes_with_context(&state, routes, App)
            .fallback(file_and_error_handler)
            .with_state(state);


        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }

    fn init_tracing() {
        let log_filter = tracing_subscriber::filter::Targets::new()
            .with_default(tracing::Level::DEBUG)
            .with_target("tokio", tracing::Level::INFO)
            .with_target("runtime", tracing::Level::INFO);

        let fmt_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .with_thread_names(false)
            .with_thread_ids(false);

        let fmt_layer_filtered = fmt_layer.with_filter(log_filter);

        tracing_subscriber::Registry::default()
            .with(fmt_layer_filtered)
            .init();
    }

}}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
