use serde::{Deserialize, Serialize};
use bmbp_rdbc_type::RdbcValue;
use crate::define::dmq::{RdbcColumn, RdbcQuery};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcWhereFilter {
    pub type_: RdbcFilterType,
    pub conditions: Vec<RdbcWhereCondition>,
    pub distinct: bool,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcFilterType {
    And,
    Or,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcWhereCondition {
    Simple(RdbcWhereSimpleCondition),
    Nest(RdbcWhereNestCondition),
    Raw(RdbcWhereRawCondition),
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcWhereSimpleCondition {
    pub column: RdbcColumn,
    pub compare: RdbcCompare,
    pub value: RdbcFilterValue,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcCompare {
    EQ,
    NE,
    GT,
    GE,
    LT,
    LE,
    Like,
    LikeLeft,
    LikeRight,
    NotLike,
    NotLikeLeft,
    NotLikeRight,
    IN,
    NotIn,
    Exists,
    NotExits,
    IsNull,
    IsNotNull,
    Between,
    NotBetween,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcWhereNestCondition {
    pub condition: RdbcWhereFilter,
}
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RdbcWhereRawCondition {
    pub condition: String,
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub enum RdbcFilterValue {
    None,
    Value(RdbcValue),
    Column(RdbcColumn),
    Query(RdbcQuery),
    Script(String),
    Raw(String),
}