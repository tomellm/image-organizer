use std::sync::Arc;

use data_communicator::buffered::{
    change::{ChangeError, ChangeResult},
    container::DataContainer,
    query::{Predicate, QueryError, QueryResponse},
    storage::{Future, InitFuture, Storage},
};
use futures::FutureExt;
use imaginator_types::media::Media;
use postgres_adapter::{delete_media, get_all_media, save_new_media};
use sqlx::{MySql, Pool};
use uuid::Uuid;

pub struct DB {
    #[allow(dead_code)]
    pool: Arc<Pool<MySql>>,
    pub media: DataContainer<Uuid, Media, MySqlWriter>,
}

impl DB {
    pub async fn init(pool: Pool<MySql>) -> Self {
        let pool = Arc::new(pool);
        Self {
            media: DataContainer::new(pool.clone()).await,
            pool,
        }
    }
    pub fn state_update(&mut self) {
        self.media.state_update();
    }
}

pub struct MySqlWriter {
    pool: Arc<Pool<MySql>>,
}

impl Storage<Uuid, Media> for MySqlWriter {
    type InitArgs = Arc<Pool<MySql>>;
    fn init(args: Self::InitArgs) -> impl InitFuture<Self> {
        async { Self { pool: args } }
    }
    fn update(&mut self, value: &Media) -> impl Future<ChangeResult> {
        save_new_media(self.pool.clone(), vec![value.to_owned()]).into_change_result()
    }
    fn update_many(&mut self, values: &[Media]) -> impl Future<ChangeResult> {
        save_new_media(self.pool.clone(), values.to_vec()).into_change_result()
    }
    fn delete(&mut self, key: &Uuid) -> impl Future<ChangeResult> {
        delete_media(self.pool.clone(), vec![*key]).into_change_result()
    }
    fn delete_many(&mut self, keys: &[Uuid]) -> impl Future<ChangeResult> {
        delete_media(self.pool.clone(), keys.to_vec()).into_change_result()
    }
    fn get_by_id(&mut self, key: Uuid) -> impl Future<QueryResponse<Uuid, Media>> {
        self.get_by_predicate(Box::new(move |media: &Media| key.eq(&media.uuid)))
    }
    fn get_by_ids(&mut self, keys: Vec<Uuid>) -> impl Future<QueryResponse<Uuid, Media>> {
        self.get_by_predicate(Box::new(move |media: &Media| keys.contains(&media.uuid)))
    }
    fn get_by_predicate(
        &mut self,
        predicate: Predicate<Media>,
    ) -> impl Future<QueryResponse<Uuid, Media>> {
        get_all_media(self.pool.clone()).then(|medias_res| async move {
            medias_res
                .map(|medias| {
                    let filtered_media = medias.into_iter().filter(predicate).collect::<Vec<_>>();
                    QueryResponse::Ok(filtered_media.into())
                })
                .unwrap_or(QueryResponse::Err(QueryError::Default))
        })
    }
}

trait ConvertAdapterFuture {
    fn into_change_result(self) -> impl Future<ChangeResult>;
}

impl<T> ConvertAdapterFuture for T
where
    T: std::future::Future<Output = Result<(), ()>> + Send + 'static,
{
    fn into_change_result(self) -> impl Future<ChangeResult> {
        async move {
            match self.await {
                Ok(()) => ChangeResult::Success,
                Err(()) => ChangeResult::Error(ChangeError::DefaultError),
            }
        }
    }
}
