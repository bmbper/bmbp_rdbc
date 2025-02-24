use serde::{Deserialize, Serialize};
use crate::RdbcColumn;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcSelectColumn {
    pub column: RdbcColumn,
    pub alias: String,
}