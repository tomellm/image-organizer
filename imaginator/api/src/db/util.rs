use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {

use std::sync::Arc;
use sqlx::{QueryBuilder, MySql, Type, Encode, Pool};
use serde::{Deserialize, Serialize};
use types::mediatypes::MediaType;

pub async fn get_number_images(
    pool: Arc<Pool<MySql>>,
) -> Result<u64, ()>{
    let mut query_builder = add_in_items(
        "select count(*) as count from media_data where media_type in (",
        MediaType ::get_image_types_u8().into_iter(),
        ")"
    );

    #[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
    struct CountQuery {
        count: i64
    }


    query_builder.build_query_as::<CountQuery>()
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            tracing::event!(
                tracing::Level::ERROR,
                "ERROR: get_count failed to execute query. {}", err
            )
        }).map(|c|c.count as u64)
}


pub fn add_in_items<'args, I, T>(
    query_front: &str,
    items: I,
    query_back: &str,
) -> QueryBuilder<'args, MySql>
where 
    I: Iterator<Item = T>,
    T: 'args + Encode<'args, MySql> + Send + Type<MySql>
{

    let mut query_builder: QueryBuilder<'args, MySql> = QueryBuilder::new(query_front);

   items.enumerate()
        .for_each(|(index, id)|{
            if index != 0 { query_builder.push(","); };
            query_builder.push_bind(id);
        });

    query_builder.push(query_back);

    query_builder
}

}}
