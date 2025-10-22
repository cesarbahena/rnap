pub async fn connect(database_url: &str) -> sqlx::Result<sqlx::PgPool> {
    sqlx::PgPool::connect(database_url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_connect_to_postgres() {
        let database_url = dotenvy::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://rnap:rnap@localhost:5432/rnap".to_string());
        let pool = connect(&database_url).await;
        assert!(pool.is_ok(), "Failed to connect to Postgres: {:?}", pool.err());
        pool.unwrap().close().await;
    }
}