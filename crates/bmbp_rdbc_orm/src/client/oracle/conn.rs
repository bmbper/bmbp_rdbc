use std::sync::Arc;
use crate::ds::RdbcDbConfig;

pub struct RdbcOracleConnection {
    db_config: Arc<RdbcDbConfig>,
}
