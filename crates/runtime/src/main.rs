mod error;

use config::DbConfig;
use database::db::Database;
use database::models::module_function::ModuleFunction;
use database::models::move_module::{ModuleContext, MoveModule};
use indexer::spawn_fetch_modules_task;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_config = DbConfig::from_env();
    let context = Arc::new(Mutex::new(ModuleContext::default()));

    let database = Database::new(db_config).unwrap();

    let (modules_sender, modules_receiver) = crossbeam_channel::unbounded::<Vec<MoveModule>>();
    let (move_functions_sender, move_functions_receiver) =
        crossbeam_channel::unbounded::<Vec<ModuleFunction>>();

    tokio::task::spawn(spawn_fetch_modules_task(database, modules_sender, move_functions_sender, context));

    loop {
        crossbeam_channel::select! {
            recv(modules_receiver) -> res => {
                    if let Ok(k) = res {
                        println!("{:?}", k);
                    }
            }
        }
    }
}
