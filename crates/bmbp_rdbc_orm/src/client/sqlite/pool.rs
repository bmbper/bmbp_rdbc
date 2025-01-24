use crate::ds::RdbcDbConfig;
use crate::rdbc_error::RdbcError;
use std::sync::Arc;

pub struct RdbcSqlitePool {
    db_config: Arc<RdbcDbConfig>,
}

impl RdbcSqlitePool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcSqlitePool { db_config };
        Ok(pool)
    }
}
