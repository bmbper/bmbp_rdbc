use std::sync::Arc;
use crate::ds::RdbcDbConfig;
use crate::rdbc_error::RdbcError;

pub struct RdbcOraclePool{
    db_config: Arc<RdbcDbConfig>,
}

impl RdbcOraclePool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcOraclePool { db_config };
        Ok(pool)
    }
}