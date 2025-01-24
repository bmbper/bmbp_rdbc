use std::sync::Arc;
use bmbp_rdbc_type::Executor;
use crate::ds::RdbcDbConfig;

pub struct RdbcMysqlConnection {
    db_config: Arc<RdbcDbConfig>,
}
impl Executor for RdbcMysqlConnection {}
