mod error;

use crate::error::Error;
use std::env;
use url::Url;

#[derive(Debug)]
pub struct DbConfig {
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub dbname: String,
}

impl From<Url> for DbConfig {
    fn from(value: Url) -> Self {
        DbConfig {
            scheme: value.scheme().to_string(),
            username: value.username().to_string(),
            password: value.password().unwrap().to_string(),
            host: value.host_str().unwrap().to_string(),
            port: value.port().unwrap().to_string(),
            dbname: value
                .path()
                .to_string()
                .strip_prefix('/')
                .unwrap()
                .to_string(),
        }
    }
}

impl ToString for DbConfig {
    fn to_string(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.scheme, self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

impl DbConfig {
    pub fn from_env(env: &str) -> Result<Self, Error> {
        let database_url_raw = env::var(env)?;
        let database_url = Url::parse(&database_url_raw)?;
        Ok(database_url.into())
    }
}

pub fn get_query_interval_from_env(env: &str) -> Result<u64, Error> {
    let query_interval_raw = env::var(env)?;
    Ok(query_interval_raw.parse()?)
}
