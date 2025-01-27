use std::sync::Arc;
use crate::ds::RdbcDbConfig;

pub struct RdbcSqliteConnection {
    db_config: Arc<RdbcDbConfig>,
}
