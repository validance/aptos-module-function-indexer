-- Your SQL goes here

CREATE TABLE IF NOT EXISTS module_function
(
    id                                  SERIAL PRIMARY KEY,
    move_modules_transaction_version    BIGINT       NOT NULL,
    move_modules_write_set_change_index BIGINT       NOT NULL,
    name                                VARCHAR(256) NOT NULL,
    visibility                          VARCHAR(32)  NOT NULL,
    is_entry                            BOOLEAN      NOT NULL,
    generic_type_params                 jsonb,
    params                              jsonb,
    return_types                        jsonb
);

CREATE INDEX idx_module_function_module_move_module_ref ON module_function (move_modules_transaction_version, move_modules_write_set_change_index);
