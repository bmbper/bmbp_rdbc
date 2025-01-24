use crate::render::render::RdbcSqlRender;
use crate::types::RdbcQuery;
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;

pub struct RdbcQueryRender {
    query: RdbcQuery,
}
impl RdbcSqlRender for RdbcQueryRender {
}
