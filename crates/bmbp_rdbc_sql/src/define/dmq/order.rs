use serde::{Deserialize, Serialize};
use crate::RdbcColumn;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcOrderColumn {
    pub column: RdbcColumn,
    pub order_type: RdbcOrderType,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcOrderType {
    Asc,
    Desc,
}