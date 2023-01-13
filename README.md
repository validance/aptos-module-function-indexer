# Aptos Move Module Function Indexer

Without adding new tables/processor on native aptos indexer, fetches module data from
indexed database, parses functions and index them.

## Setup
1. Install Rust: https://www.rust-lang.org/tools/install
2. Setup Diesel ORM:
   ```shell
   sudo apt-get install libpq-dev
   cargo install diesel_cli --no-default-features --features postgres
   ```
3. Edit values in .env file, Postgresql only.
    * DATABASE_URL: Database of function indexer
    * FULL_INDEXER_URL: Database of native Aptos indexer
   
4. Diesel migration
    ```
   cd crates/database 
   diesel migration run 
   ```
   
## Logs
There are three different types of log on the runtime.

* fetch_module_task(query modules from aptos native indexer) 
* parser_task(parse functions from modules)
* function_indexing_task(insert functions to db)

These tasks run in asynchronous manner and pass data through crossbeam-channel