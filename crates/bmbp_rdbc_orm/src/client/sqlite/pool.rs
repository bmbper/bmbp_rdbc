use crate::ds::RdbcDbConfig;
use std::sync::Arc;
use bmbp_rdbc_type::{Executor, RdbcError};

pub struct RdbcSqlitePool {
    db_config: Arc<RdbcDbConfig>,
}

impl RdbcSqlitePool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcSqlitePool { db_config };
        Ok(pool)
    }
}
impl Executor for RdbcSqlitePool {}