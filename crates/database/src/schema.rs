// @generated automatically by Diesel CLI.

diesel::table! {
    module_function (id) {
        id -> Int4,
        module_address -> Varchar,
        module_name -> Varchar,
        move_modules_transaction_version -> Int8,
        move_modules_write_set_change_index -> Int8,
        name -> Varchar,
        visibility -> Varchar,
        is_entry -> Bool,
        generic_type_params -> Nullable<Jsonb>,
        params -> Nullable<Jsonb>,
        return_types -> Nullable<Jsonb>,
    }
}

diesel::table! {
    move_modules (transaction_version, write_set_change_index) {
        transaction_version -> Int8,
        write_set_change_index -> Int8,
        transaction_block_height -> Int8,
        name -> Text,
        address -> Varchar,
        bytecode -> Nullable<Bytea>,
        friends -> Nullable<Jsonb>,
        exposed_functions -> Nullable<Jsonb>,
        structs -> Nullable<Jsonb>,
        is_deleted -> Bool,
        inserted_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    module_function,
    move_modules,
);
