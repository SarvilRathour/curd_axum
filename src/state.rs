use sqlx::PgPool;
use crate::repositories::UserRepository;
use axum::extract::FromRef;
#[derive(Clone,FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub user_repository:UserRepository,
}
impl AppState {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(url).await?;
        sqlx::migrate!("./migrations").run(&db).await?;
        let user_repository=UserRepository::new(db.clone());
        
        Ok(Self { db,user_repository })
    }
}
