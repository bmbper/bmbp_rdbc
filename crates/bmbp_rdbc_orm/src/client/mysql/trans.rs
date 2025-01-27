use std::sync::Arc;
use crate::ds::RdbcDbConfig;

pub struct RdbcMysqlTransaction {
    db_config: Arc<RdbcDbConfig>,
}
