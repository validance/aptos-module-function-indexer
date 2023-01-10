mod error;

use config::DbConfig;
use database::db::Database;
use database::models::module_function::ModuleFunction;
use database::models::move_module::{ModuleContext, MoveModule};
use indexer::{spawn_fetch_modules_task, spawn_function_indexer_task, spawn_function_parser_task};
use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_config = DbConfig::from_env();
    let context = Arc::new(Mutex::new(ModuleContext::default()));

    let database = RefCell::new(Database::new(db_config).unwrap());

    let (modules_sender, modules_receiver) = crossbeam_channel::unbounded::<Vec<MoveModule>>();
    let (move_functions_sender, move_functions_receiver) =
        crossbeam_channel::unbounded::<Vec<ModuleFunction>>();

    let module_task_handle = tokio::task::spawn(spawn_fetch_modules_task(
        database.clone(),
        modules_sender,
        context,
    ));

    let parser_task_handle = tokio::task::spawn(spawn_function_parser_task(
        modules_receiver,
        move_functions_sender,
    ));

    let function_indexer_task_handle = tokio::task::spawn(spawn_function_indexer_task(
        move_functions_receiver,
        database,
    ));

    futures::future::join_all([
        module_task_handle,
        parser_task_handle,
        function_indexer_task_handle,
    ])
    .await;
}
