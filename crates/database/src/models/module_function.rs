use crate::storage::WriteDatabase;
use crate::schema;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::module_function)]
pub struct ModuleFunction {
    pub id: Option<i32>,
    pub name: String,
    pub visibility: String,
    pub is_entry: bool,
    pub generic_type_params: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub return_types: Option<serde_json::Value>,
}

impl WriteDatabase for ModuleFunction {}
