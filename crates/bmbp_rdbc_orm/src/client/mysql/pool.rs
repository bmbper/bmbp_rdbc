use crate::ds::RdbcDbConfig;
use crate::rdbc_error::RdbcError;
use std::sync::Arc;

pub struct RdbcMysqlPool {
    db_config: Arc<RdbcDbConfig>,
}

impl RdbcMysqlPool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcMysqlPool { db_config };
        Ok(pool)
    }
}
