use std::sync::Arc;
use bmbp_rdbc_type::Executor;
use crate::ds::RdbcDbConfig;

pub struct RdbcSqliteConnection {
    db_config: Arc<RdbcDbConfig>,
}
impl Executor for RdbcSqliteConnection {}
