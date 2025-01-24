use crate::types::{RdbcDmlColumn, RdbcQuery};
use crate::RdbcTable;
use bmbp_rdbc_type::RdbcValue;

pub struct RdbcInsert {
    pub(crate) table: RdbcTable,
    column: Vec<String>,
    values: Vec<RdbcValue>,
    dml_columns: Vec<RdbcDmlColumn>,
    query: Option<RdbcQuery>,
}
