use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

use std::{env, sync::Arc};
use leptos::LeptosOptions;
use axum::extract::FromRef;
use cloud_storage::Client;
use futures::lock::Mutex;
use sqlx::{MySql, MySqlPool, Pool};

#[derive(Clone, FromRef)]
pub struct ApiState {
    pub databse: Database,
    pub bucket: Bucket,
    pub graph_db: GraphDB,
    pub leptos_options: LeptosOptions
}

#[derive(Clone, Debug)]
pub struct Database(pub Arc<Pool<MySql>>);

#[derive(Clone, Debug)]
pub struct Bucket(pub Arc<Client>);

#[derive(Clone)]
pub struct GraphDB(pub Arc<Mutex<indradb_proto::Client>>);

impl ApiState {
    pub async fn new(leptos_options: LeptosOptions) -> Self {
        Self {
            databse: Database::new().await,
            bucket: Bucket::new(),
            graph_db: GraphDB::new().await,
            leptos_options
        }
    }
}

impl Database {
    pub async fn new() -> Self {
        Self(Arc::new(
            MySqlPool::connect(&env::var("DATABASE_URL").expect("Could not find DATABASE_URL env"))
                .await
                .expect("Failed to connect to DB"),
        ))
    }
}

impl Bucket {
    pub fn new() -> Self {
        Self(Arc::new(Client::default()))
    }
}

impl GraphDB {
    pub async fn new() -> Self {
        Self(Arc::new(Mutex::new(
            indradb_proto::Client::new(
                env::var("GRAPH_DB_URL").unwrap().try_into().unwrap()
            ).await.unwrap()
        )))
    }
}

}}
