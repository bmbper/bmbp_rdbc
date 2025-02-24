use serde::{Deserialize, Serialize};
use crate::{RdbcQuery, RdbcWhereFilter};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcTable {
    SimpleTable(RdbcSimpleTable),
    QueryTable(RdbcQueryTable),
    RawTable(RdbcRawTable),
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcSimpleTable {
    pub schema: String,
    pub table: String,
    pub alias: String,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcQueryTable {
    pub query: RdbcQuery,
    pub alias: String,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcRawTable {
    pub table: String,
    pub alias: String,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcJoinTable {
    pub table: RdbcTable,
    pub join_type: JoinType,
    pub filter: Option<RdbcWhereFilter>,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
}
