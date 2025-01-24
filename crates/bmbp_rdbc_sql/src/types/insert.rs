use crate::types::{RdbcDmlColumn, RdbcQuery};
use crate::{RdbcDmlValue, RdbcTable};
use bmbp_rdbc_type::RdbcValue;

pub struct RdbcInsert {
    pub(crate) table: Option<RdbcTable>,
    pub(crate) column: Vec<String>,
    pub(crate) values: Vec<RdbcDmlValue>,
    pub(crate) dml_columns: Vec<RdbcDmlColumn>,
    pub(crate) query: Option<RdbcQuery>,
}

impl RdbcInsert {
    pub fn new() -> Self {
        RdbcInsert {
            table: None,
            column: vec![],
            values: vec![],
            dml_columns: vec![],
            query: None,
        }
    }
}
