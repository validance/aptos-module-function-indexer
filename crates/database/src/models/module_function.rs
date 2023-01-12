use crate::db::WriteDatabase;
use crate::schema;

use crate::models::DbError;
use crate::storage::PooledConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = schema::module_function)]
pub struct NewModuleFunction {
    pub id: Option<i32>,
    pub module_address: String,
    pub module_name: String,
    pub move_modules_transaction_version: i64,
    pub move_modules_write_set_change_index: i64,
    pub name: String,
    pub visibility: String,
    pub is_entry: bool,
    pub generic_type_params: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub return_types: Option<serde_json::Value>,
}

impl WriteDatabase for NewModuleFunction {}

#[derive(Clone, Debug, PartialEq, Eq, Queryable)]
pub struct ModuleFunction {
    pub id: i32,
    pub module_address: String,
    pub module_name: String,
    pub move_modules_transaction_version: i64,
    pub move_modules_write_set_change_index: i64,
    pub name: String,
    pub visibility: String,
    pub is_entry: bool,
    pub generic_type_params: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub return_types: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposedFunctions {
    pub name: String,
    pub visibility: String,
    pub is_entry: bool,
    pub generic_type_params: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub r#return: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericTypeParams {
    pub constraints: Vec<String>,
}

impl ExposedFunctions {
    pub fn to_module_function(
        &self,
        module_address: &str,
        module_name: &str,
        move_modules_transaction_version: i64,
        move_modules_write_set_change_index: i64,
    ) -> NewModuleFunction {
        NewModuleFunction {
            id: None,
            module_address: module_address.to_string(),
            module_name: module_name.to_string(),
            move_modules_transaction_version,
            move_modules_write_set_change_index,
            name: self.name.to_string(),
            visibility: self.visibility.to_string(),
            is_entry: self.is_entry,
            generic_type_params: self.generic_type_params.clone(),
            params: self.params.clone(),
            return_types: self.r#return.clone(),
        }
    }
}

impl ModuleFunction {
    pub fn get_latest_function(conn: &mut PooledConnection) -> Result<ModuleFunction, DbError> {
        use crate::schema::module_function::dsl::*;

        Ok(module_function.order(id.desc()).first(conn)?)
    }
}

#[cfg(test)]
pub mod module_test {
    use crate::models::module_function::{ExposedFunctions, GenericTypeParams};

    #[test]
    pub fn deserialize_module_functions() {
        let data = r#"
[
  {
    "name": "pack",
    "visibility": "public",
    "is_entry": false,
    "generic_type_params": [
      {
        "constraints": [
          "drop",
          "store"
        ]
      }
    ],
    "params": [
      "T0"
    ],
    "return": [
      "0x1::any::Any"
    ]
  },
  {
    "name": "type_name",
    "visibility": "public",
    "is_entry": false,
    "generic_type_params": [],
    "params": [
      "&0x1::any::Any"
    ],
    "return": [
      "&0x1::string::String"
    ]
  },
  {
    "name": "unpack",
    "visibility": "public",
    "is_entry": false,
    "generic_type_params": [
      {
        "constraints": []
      }
    ],
    "params": [
      "0x1::any::Any"
    ],
    "return": [
      "T0"
    ]
  }
]
            "#;

        let res: Vec<ExposedFunctions> = serde_json::from_str(data).unwrap();
        let generic_type_params_raw = res
            .get(0)
            .map(|i| i.generic_type_params.clone())
            .unwrap()
            .unwrap();
        let params_raw = res.get(0).map(|i| i.params.clone()).unwrap().unwrap();
        let returns_raw = res.get(0).map(|i| i.r#return.clone()).unwrap().unwrap();

        let generic_type_params =
            serde_json::from_value::<Vec<GenericTypeParams>>(generic_type_params_raw).unwrap();
        let params = serde_json::from_value::<Vec<String>>(params_raw).unwrap();
        let returns = serde_json::from_value::<Vec<String>>(returns_raw).unwrap();

        assert_eq!(
            generic_type_params,
            vec![GenericTypeParams {
                constraints: vec![String::from("drop"), String::from("store")]
            }]
        );
        assert_eq!(params, vec![String::from("T0")]);
        assert_eq!(returns, vec![String::from("0x1::any::Any")]);
    }
}
