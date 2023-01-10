use crate::models::DbError;

use config::DbConfig;
use diesel::r2d2::{self, ConnectionManager};

pub type DbCon = diesel::PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbCon>>;
pub(crate) type PooledConnection = r2d2::PooledConnection<ConnectionManager<DbCon>>;

#[derive(Clone)]
pub(crate) struct DatabaseInner {
    pool: DbPool,
}

impl DatabaseInner {
    pub fn new(config: DbConfig) -> Result<Self, DbError> {
        let db_uri = config.to_string();
        let database_pool = r2d2::Pool::builder().build(ConnectionManager::<DbCon>::new(db_uri));

        match database_pool {
            Ok(db_pool) => Ok(Self { pool: db_pool }),
            Err(_) => Err(DbError::InnerDbInitFailed {}),
        }
    }

    pub(crate) fn conn(&self) -> Result<PooledConnection, DbError> {
        self.pool
            .get()
            .map_err(|_| DbError::ClientConnectionFailed {})
    }
}
