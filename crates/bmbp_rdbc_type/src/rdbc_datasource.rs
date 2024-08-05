use crate::RdbcDataBase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RdbcDataSource {
    typ: RdbcDataBase,
    host: Option<String>,
    port: Option<u32>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
    ignore_case: Option<bool>,
    use_ssl: Option<bool>,
    init_conn_size: Option<usize>,
    grow_conn_size: Option<usize>,
    max_conn_size: Option<usize>,
    max_idle_conn: Option<usize>,
    max_wait_time: Option<usize>,
    max_idle_time: Option<usize>,
}
