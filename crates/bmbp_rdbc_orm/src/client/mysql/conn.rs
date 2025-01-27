use std::sync::Arc;
use crate::ds::RdbcDbConfig;

pub struct RdbcMysqlConnection {
    db_config: Arc<RdbcDbConfig>,
}