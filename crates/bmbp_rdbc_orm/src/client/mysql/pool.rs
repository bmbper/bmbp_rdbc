use crate::ds::RdbcDbConfig;
use std::sync::Arc;
use bmbp_rdbc_type::{Executor, RdbcError};

pub struct RdbcMysqlPool {
    db_config: Arc<RdbcDbConfig>,
}

impl RdbcMysqlPool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcMysqlPool { db_config };
        Ok(pool)
    }
}

impl Executor for RdbcMysqlPool {}