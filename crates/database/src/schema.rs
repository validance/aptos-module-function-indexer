// @generated automatically by Diesel CLI.

diesel::table! {
    module_function (id) {
        id -> Int4,
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
