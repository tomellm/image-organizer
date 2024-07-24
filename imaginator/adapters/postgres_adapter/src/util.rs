pub mod query;

use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use sqlx::{
    error::Error, query_builder::Separated, Database, Encode, MySql, Pool,
    QueryBuilder, Type,
};
use std::{pin::Pin, sync::Arc};

use crate::types::mediatype::DBEnum;

pub async fn get_number_images(pool: Arc<Pool<MySql>>) -> Result<u64, ()> {
    let mut query_builder = add_in_items(
        "select count(*) as count from media_data where media_type in (",
        DBEnum::get_image_types_u8().into_iter(),
        ")",
    );

    #[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
    struct CountQuery {
        count: i64,
    }

    query_builder
        .build_query_as::<CountQuery>()
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: get_count failed to execute query. {}",
                err
            )
        })
        .map(|c| c.count as u64)
}

pub fn add_in_items<'args, I, T>(
    query_front: &str,
    items: I,
    query_back: &str,
) -> QueryBuilder<'args, MySql>
where
    I: IntoIterator<Item = T>,
    T: 'args + Encode<'args, MySql> + Send + Type<MySql>,
{
    let mut query_builder: QueryBuilder<'args, MySql> = QueryBuilder::new(query_front);

    items.into_iter().enumerate().for_each(|(index, id)| {
        if index != 0 {
            query_builder.push(",");
        };
        query_builder.push_bind(id);
    });

    query_builder.push(query_back);

    query_builder
}

pub trait LogMysqlError<T> {
    fn log_err(self, err_text: &str) -> Result<T, ()>;
}

impl<T> LogMysqlError<T> for Result<T, Error> {
    fn log_err(self, err_text: &str) -> Result<T, ()> {
        self.map_err(|err| tracing::event!(tracing::Level::ERROR, "ERROR: {}. {}", err_text, err))
    }
}

pub trait AdapterFuture<FutureOutput>
where
    Self: std::future::Future<Output = FutureOutput> + Send,
    FutureOutput: Send,
{
}

impl<T, FutureOutput> AdapterFuture<FutureOutput> for T
where
    T: std::future::Future<Output = FutureOutput> + Send,
    FutureOutput: Send,
{
}

pub trait AwaitQueryResponses {
    fn join_await(self) -> impl AdapterFuture<Result<(), ()>>;
}

impl<Fut> AwaitQueryResponses for Vec<Fut>
where
    Fut: std::future::Future<Output = Result<<MySql as Database>::QueryResult, Error>>
        + Send
        + 'static,
{
    fn join_await(self) -> impl AdapterFuture<Result<(), ()>> {
        Box::pin(async move {
            try_join_all(self)
                .await
                .log_err("While joining qery responses an error occured")?;
            Ok(())
        }) as Pin<Box<dyn std::future::Future<Output = Result<(), ()>> + Send>>
    }
}

/// When implementing column names and push touples make sure that the two
/// functions name the different columns in the same order, since otherwise it
/// will lead to unexpected results when creating the queries
pub trait DatabaseUtilities {
    fn db_table_name() -> &'static str;
    fn db_column_names() -> &'static [&'static str];
    fn db_push_touple_fn() -> impl FnMut(Separated<'_, '_, MySql, &'static str>, Self);
}
