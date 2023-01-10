use config::DbConfig;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let _db_config = DbConfig::from_env();
}
