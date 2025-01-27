use std::sync::Arc;
use crate::ds::RdbcDbConfig;

pub struct RdbcOracleTransaction {
    db_config: Arc<RdbcDbConfig>,
}
