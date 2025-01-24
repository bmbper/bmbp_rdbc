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

pub struct RdbcDbPoolConfig {
    pub int_size: u16,
    pub max_size: u16,
    pub max_idle: u16,
    pub wait_timeout: Duration,
}
