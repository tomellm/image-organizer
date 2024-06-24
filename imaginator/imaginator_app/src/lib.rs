use std::sync::Arc;

use imaginator_types::media::Media;
use sqlx::{MySql, Pool};
use postgres_adapter as pg;




pub async fn view_all_media(pool: Arc<Pool<MySql>>) -> Result<Vec<Media>, ()> {
    pg::get_all_media(pool).await
}

pub async fn delete_all_media(pool: Arc<Pool<MySql>>) -> Result<(), ()> {
    pg::delete_all(pool).await
}
