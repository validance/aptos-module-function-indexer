use crossbeam_channel::{Receiver, Sender};
use database::db::Database;
use database::models::move_module::{ModuleContext, MoveModule};
use database::models::DbError;
use std::sync::Arc;
use tokio::sync::Mutex;
use database::models::module_function::ModuleFunction;

pub async fn spawn_fetch_modules_task(
    database: Database,
    modules_sender: Sender<Vec<MoveModule>>,
    move_functions_sender: Sender<Vec<ModuleFunction>>,
    context: Arc<Mutex<ModuleContext>>,
) -> Result<(), DbError> {
    let mut conn = database.get_conn()?;

    loop {
        let mut context = context.lock().await;
        let res = MoveModule::get_latest_modules(&mut conn, &context)?;
        if let Some(last_module) = res.last() {
            context.write_set_change_index = last_module.write_set_change_index;
            context.transaction_version = last_module.transaction_version;
            context.offset += context.stride;
        }
        modules_sender.send(res).map_err(|_| DbError::DieselError).ok();
    }
}

pub async fn spawn_function_parser_task(
    modules_receiver: Receiver<MoveModule>
) {
    loop {
        crossbeam_channel::select! {
            recv(modules_receiver) -> unchecked_modules => {
                if let Ok(modules) = unchecked_modules {

                }
            }
        }
    }
}