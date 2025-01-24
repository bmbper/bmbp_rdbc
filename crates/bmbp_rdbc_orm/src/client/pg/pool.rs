use std::sync::Arc;
use bmbp_rdbc_type::{Executor, RdbcError};
use crate::ds::RdbcDbConfig;
use crate::rdbc_error::RdbcError;

pub struct RdbcPgPool{
    db_config:Arc<RdbcDbConfig>
}
impl RdbcPgPool {
    pub(crate) async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Self, RdbcError> {
        let pool = RdbcPgPool { db_config };
        Ok(pool)
    }
}
impl Executor for RdbcPgPool {}