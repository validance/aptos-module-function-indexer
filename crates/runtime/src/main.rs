mod error;

use config::DbConfig;
use database::db::Database;
use database::models::module_function::NewModuleFunction;
use database::models::move_module::{ModuleContext, MoveModule};
use indexer::{spawn_fetch_modules_task, spawn_function_indexer_task, spawn_function_parser_task};
use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    tracing::info!("Initializing");

    let aptos_db_config =
        DbConfig::from_env("FULL_INDEXER_URL").expect("env variable FULL_INDEXER_URL not found");
    let function_indexer_config =
        DbConfig::from_env("DATABASE_URL").expect("env variable DATABASE_URL not found");
    let query_interval = config::get_query_interval_from_env("QUERY_INTERVAL").unwrap_or(1000);

    let aptos_database = RefCell::new(Database::new(aptos_db_config).unwrap());
    let function_indexer_db = RefCell::new(Database::new(function_indexer_config).unwrap());

    let context = Arc::new(Mutex::new(ModuleContext::load_or_new(
        &mut function_indexer_db.borrow_mut().get_conn().unwrap(),
    )));

    let (modules_sender, modules_receiver) = crossbeam_channel::unbounded::<Vec<MoveModule>>();
    let (move_functions_sender, move_functions_receiver) =
        crossbeam_channel::unbounded::<Vec<NewModuleFunction>>();

    let module_task_handle = tokio::task::spawn(spawn_fetch_modules_task(
        aptos_database.clone(),
        modules_sender,
        context,
        query_interval,
    ));

    let parser_task_handle = tokio::task::spawn(spawn_function_parser_task(
        modules_receiver,
        move_functions_sender,
    ));

    let function_indexer_task_handle = tokio::task::spawn(spawn_function_indexer_task(
        move_functions_receiver,
        function_indexer_db,
    ));

    futures::future::join_all([
        module_task_handle,
        parser_task_handle,
        function_indexer_task_handle,
    ])
    .await;
}
