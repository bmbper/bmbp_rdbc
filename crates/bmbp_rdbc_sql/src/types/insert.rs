use bmbp_rdbc_type::RdbcValue;
use crate::types::{OrmSchemaTable, RdbcDmlColumn, RdbcQuery};

pub struct RdbcInsert {
    table: OrmSchemaTable,
    column: Vec<String>,
    values: Vec<RdbcValue>,
    dml_columns: Vec<RdbcDmlColumn>,
    query: Option<RdbcQuery>,
}
