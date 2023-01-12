use crossbeam_channel::{Receiver, Sender};
use database::db::{Database, WriteDatabase};
use database::models::module_function::NewModuleFunction;
use database::models::move_module::{ModuleContext, MoveModule};
use database::models::DbError;
use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, instrument};

#[instrument(name = "fetch_module_task")]
pub async fn spawn_fetch_modules_task(
    database: RefCell<Database>,
    modules_sender: Sender<Vec<MoveModule>>,
    context: Arc<Mutex<ModuleContext>>,
    query_interval: u64,
) -> Result<(), DbError> {
    let mut conn = database.borrow_mut().get_conn()?;
    loop {
        let mut context = context.lock().await;
        info!("fetching data from index database");
        let res = MoveModule::get_latest_modules(&mut conn, &context)?;

        if let Some(last_module) = res.last() {
            info!("new module found! checkpoint: {:?}", context);
            context.transaction_version = last_module.transaction_version;
            context.write_set_change_index = last_module.write_set_change_index;

            modules_sender
                .send(res)
                .map_err(|_| DbError::DieselError)
                .ok();
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(query_interval)).await;
    }
}

#[instrument(name = "parser_task")]
pub async fn spawn_function_parser_task(
    modules_receiver: Receiver<Vec<MoveModule>>,
    move_functions_sender: Sender<Vec<NewModuleFunction>>,
) -> Result<(), DbError> {
    loop {
        crossbeam_channel::select! {
            recv(modules_receiver) -> unchecked_modules => {
                if let Ok(modules) = unchecked_modules {
                    modules
                    .into_iter()
                    .for_each(|module| {
                        if let Some(function_collections) = module.extract_functions() {
                            info!("parsing {} functions", function_collections.len());
                            let module_functions = function_collections
                                .into_iter()
                                .map(|function| function.to_module_function(&module.address, &module.name, module.transaction_version, module.write_set_change_index))
                                .collect::<Vec<NewModuleFunction>>();
                            move_functions_sender.send(module_functions).ok();
                        }
                    })
                }
            }
        }
    }
}

#[instrument(name = "function_indexing_task")]
pub async fn spawn_function_indexer_task(
    move_functions_receiver: Receiver<Vec<NewModuleFunction>>,
    database: RefCell<Database>,
) -> Result<(), DbError> {
    let mut conn = database.borrow_mut().get_conn()?;

    loop {
        crossbeam_channel::select! {
            recv(move_functions_receiver) -> unchecked_functions => {
                if let Ok(functions) = unchecked_functions {
                    info!("indexing {} functions", functions.len());
                    functions.into_iter().for_each(|function| {
                        function
                        .create(&mut conn, database::schema::module_function::table)
                        .ok();
                    })
                }
            }
        }
    }
}
