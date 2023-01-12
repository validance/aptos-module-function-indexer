-- Your SQL goes here

CREATE TABLE IF NOT EXISTS module_function
(
    id                                  SERIAL PRIMARY KEY,
    module_address                      VARCHAR(66)  NOT NULL,
    module_name                         VARCHAR(256) NOT NULL,
    move_modules_transaction_version    BIGINT       NOT NULL,
    move_modules_write_set_change_index BIGINT       NOT NULL,
    name                                VARCHAR(256) NOT NULL,
    visibility                          VARCHAR(32)  NOT NULL,
    is_entry                            BOOLEAN      NOT NULL,
    generic_type_params                 jsonb,
    params                              jsonb,
    return_types                        jsonb
);

CREATE TABLE move_modules
(
    transaction_version      BIGINT      NOT NULL,
    write_set_change_index   BIGINT      NOT NULL,
    transaction_block_height BIGINT      NOT NULL,
    name                     TEXT        NOT NULL,
    address                  VARCHAR(66) NOT NULL,
    bytecode                 bytea,
    friends                  jsonb,
    exposed_functions        jsonb,
    structs                  jsonb,
    is_deleted               BOOLEAN     NOT NULL,
    inserted_at              TIMESTAMP   NOT NULL DEFAULT NOW(),

    PRIMARY KEY (transaction_version, write_set_change_index)
);

CREATE INDEX idx_module_function_module_move_module_ref ON module_function (move_modules_transaction_version, move_modules_write_set_change_index);
CREATE INDEX mm_addr_name_ver_index ON move_modules (address, name, transaction_version);
CREATE INDEX mm_insat_index ON move_modules (inserted_at);
