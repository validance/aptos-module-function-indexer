use crate::models::module_function::ModuleFunction;
use crate::models::DbError;
use crate::schema::move_modules::dsl::*;
use crate::storage::PooledConnection;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use tokio::sync::MutexGuard;

#[derive(Debug, PartialEq, Eq, Queryable)]
pub struct MoveModule {
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub transaction_block_height: i64,
    pub name: String,
    pub address: String,
    pub bytecode: Option<Vec<u8>>,
    pub exposed_functions: Option<serde_json::Value>,
    pub friends: Option<serde_json::Value>,
    pub structs: Option<serde_json::Value>,
    pub is_deleted: bool,
    pub inserted_at: NaiveDateTime,
}

impl MoveModule {
    pub fn get_latest_modules(
        conn: &mut PooledConnection,
        context: &MutexGuard<ModuleContext>,
    ) -> Result<Vec<MoveModule>, DbError> {
        Ok(move_modules
            .filter(transaction_version.ge(context.transaction_version))
            .filter(write_set_change_index.gt(context.write_set_change_index))
            .offset(context.offset)
            .limit(context.stride)
            .load::<MoveModule>(conn)?)
    }

    pub fn extract_functions(&self) -> Option<Vec<ModuleFunction>> {
        match self.exposed_functions.clone() {
            Some(functions) => serde_json::from_value::<Vec<ModuleFunction>>(functions).ok(),
            None => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct ModuleContext {
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub offset: i64,
    pub stride: i64,
}
