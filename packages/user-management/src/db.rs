use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn setup(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(db_url).await?;
        Ok(Self { pool })
    }
}
