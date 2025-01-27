use std::sync::Arc;
use crate::ds::RdbcDbConfig;

pub struct RdbcPgTransaction {
    db_config: Arc<RdbcDbConfig>,
}
