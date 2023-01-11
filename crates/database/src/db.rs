use crate::models::DbError;
use crate::storage::{DatabaseInner, PooledConnection};
use config::DbConfig;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::{Insertable, RunQueryDsl, Table};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Database {
    db: DatabaseInner,
}

impl Debug for Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "database")
    }
}

impl Database {
    pub fn new(config: DbConfig) -> Result<Self, DbError> {
        let inner_db = DatabaseInner::new(config)?;
        Ok(Self { db: inner_db })
    }

    pub fn get_conn(&self) -> Result<PooledConnection, DbError> {
        self.db.conn()
    }
}

pub trait WriteDatabase {
    fn create<T>(self, conn: &mut PooledConnection, table: T) -> Result<usize, DbError>
    where
        T: Table,
        Self: Insertable<T> + Sized,
        InsertStatement<T, Self::Values>: ExecuteDsl<PooledConnection>,
    {
        self.insert_into(table).execute(conn).map_err(|e| e.into())
    }
}
