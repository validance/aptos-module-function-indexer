use crate::models::module_function::{ExposedFunctions, ModuleFunction};
use crate::models::DbError;
use crate::storage::PooledConnection;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use tokio::sync::MutexGuard;

#[derive(Clone, Debug, PartialEq, Eq, Queryable)]
pub struct MoveModule {
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub transaction_block_height: i64,
    pub name: String,
    pub address: String,
    pub bytecode: Option<Vec<u8>>,
    pub friends: Option<serde_json::Value>,
    pub exposed_functions: Option<serde_json::Value>,
    pub structs: Option<serde_json::Value>,
    pub is_deleted: bool,
    pub inserted_at: NaiveDateTime,
}

impl MoveModule {
    pub fn get_latest_modules(
        conn: &mut PooledConnection,
        context: &MutexGuard<ModuleContext>,
    ) -> Result<Vec<MoveModule>, DbError> {
        use crate::schema::move_modules::dsl::*;

        Ok(move_modules
            .filter(
                transaction_version
                    .ge(context.transaction_version)
                    .and(write_set_change_index.gt(context.write_set_change_index))
                    .or(transaction_version.gt(context.transaction_version)),
            )
            .limit(context.stride)
            .load::<MoveModule>(conn)?)
    }

    pub fn extract_functions(&self) -> Option<Vec<ExposedFunctions>> {
        match self.exposed_functions.clone() {
            Some(functions) => serde_json::from_value::<Vec<ExposedFunctions>>(functions).ok(),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct ModuleContext {
    pub transaction_version: i64,
    pub write_set_change_index: i64,
    pub stride: i64,
}

impl ModuleContext {
    pub fn load_or_new(conn: &mut PooledConnection) -> Self {
        match ModuleFunction::get_latest_function(conn) {
            Ok(latest_fn) => Self::new(
                latest_fn.move_modules_transaction_version,
                latest_fn.move_modules_write_set_change_index,
                30,
            ),
            Err(_) => Self::new(0, 0, 30),
        }
    }

    fn new(transaction_version: i64, write_set_change_index: i64, stride: i64) -> Self {
        ModuleContext {
            transaction_version,
            write_set_change_index,
            stride,
        }
    }
}
