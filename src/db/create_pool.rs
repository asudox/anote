use crate::utils::get_config;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

/// This function is used to create an async connection pool to the database.
pub async fn create_pool() -> Pool<AsyncPgConnection> {
    let config = get_config();
    let database_url = format!(
        "postgresql://{}:{}@{}/{}",
        config.database.DB_USERNAME,
        config.database.DB_PASSWORD,
        config.database.DB_HOST,
        config.database.DB_NAME
    );
    let pool_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(pool_config).build();

    pool.unwrap()
}
