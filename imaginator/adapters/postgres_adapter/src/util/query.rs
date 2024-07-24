use std::sync::Arc;

use sqlx::{MySql, Pool, QueryBuilder};

use super::{AdapterFuture, AwaitQueryResponses, DatabaseUtilities};

const BIND_LIMIT: usize = 10000;

pub fn save_many<T>(pool: Arc<Pool<MySql>>, t_data: Vec<T>) -> impl AdapterFuture<Result<(), ()>>
where
    T: DatabaseUtilities + Send + 'static,
{
    let block_length: usize = BIND_LIMIT / T::db_column_names().len();
    async move {
        if t_data.len() == 0 {
            return Ok(());
        }

        let chunks = t_data.into_iter().enumerate().fold(
            vec![],
            |mut acc: Vec<(QueryBuilder<MySql>, Vec<T>)>, (pos, data): (usize, T)| {
                let index = (pos as f32 / block_length as f32).floor() as usize;
                let inner_index = (pos as f32 % block_length as f32) as usize;
                match acc.get_mut(index) {
                    Some(inner_vec) => {
                        inner_vec.1.insert(inner_index, data);
                    }
                    None => {
                        let query_str = format!(
                            "insert into {} ({})",
                            T::db_table_name(),
                            T::db_column_names().join(", ")
                        );
                        acc.insert(index, (QueryBuilder::new(query_str), vec![]));
                        let inner_vec = acc.get_mut(index).unwrap();
                        inner_vec.1.insert(inner_index, data);
                    }
                }
                acc
            },
        );
        let mut futures = vec![];
        for (mut query_builder, chunk) in chunks.into_iter() {
            query_builder.push_values(chunk, T::db_push_touple_fn());

            let execute_pool = pool.clone();
            futures.push(async move {
                let query = query_builder.build();
                query.execute(&*execute_pool).await
            });
        }

        futures.join_await().await
    }
}
