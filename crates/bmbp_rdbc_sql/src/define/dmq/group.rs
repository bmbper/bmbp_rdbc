use serde::{Deserialize, Serialize};
use crate::define::dmq::RdbcColumn;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcGroupColumn {
    pub column: RdbcColumn,
}