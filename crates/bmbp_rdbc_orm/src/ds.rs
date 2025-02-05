use chrono::Duration;

pub enum RdbcDbType {
    Mysql,
    Postgres,
    Sqlite,
    Oracle,
}
pub struct RdbcDbConfig {
    pub database_type: RdbcDbType,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String,
    pub schema: String,
    pub pool_config: Option<RdbcDbPoolConfig>,
}

impl RdbcDbConfig {
    pub fn new(
        db_type: RdbcDbType,
        host: &str,
        port: i32,
        user: &str,
        password: &str,
        database_name: &str,
        schema: &str,
        pool_config: Option<RdbcDbPoolConfig>,
    ) -> Self {
        RdbcDbConfig {
            database_type: db_type,
            host: host.to_string(),
            port: port as u16,
            user: user.to_string(),
            password: password.to_string(),
            database_name: database_name.to_string(),
            schema: schema.to_string(),
            pool_config,
        }
    }
}

pub struct RdbcDbPoolConfig {
    pub int_size: u16,
    pub max_size: u16,
    pub max_idle: u16,
    pub wait_timeout: Duration,
}
