# Aptos Move Module Function Indexer

Without adding new tables/processor on native aptos indexer, fetches module data from
indexed database, parses functions and index them.

## Setup
1. Edit values in .env file. Postgresql only.
    * DATABASE_URL: Database of function indexer
    * FULL_INDEXER_URL: Database of native Aptos indexer
   
2. Diesel migration
    ```
   cd crates/database 
   diesel migration run 
   ```
   
## Logs
There are three different types of log on the runtime.

fetch_module_task -> parser_task -> function_indexing_task

These tasks run in asynchronous manner and pass data through crossbeam-channel