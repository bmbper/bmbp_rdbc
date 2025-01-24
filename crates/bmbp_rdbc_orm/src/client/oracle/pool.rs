use std::sync::Arc;
use bmbp_rdbc_type::{Executor, RdbcError};
use crate::ds::RdbcDbConfig;

pub struct RdbcOraclePool{
    db_config: Arc<RdbcDbConfig>,
}

impl RdbcOraclePool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcOraclePool { db_config };
        Ok(pool)
    }
}
impl Executor for RdbcOraclePool {}