use crate::types::{RdbcColumn, RdbcQuery};
use bmbp_rdbc_type::RdbcValue;
#[derive(Debug, Clone)]
pub struct RdbcWhereFilter {
    pub type_: RdbcFilterType,
    pub conditions: Vec<RdbcWhereCondition>,
    pub distinct: bool,
}
impl RdbcWhereFilter {
    pub fn new() -> Self {
        RdbcWhereFilter {
            type_: RdbcFilterType::And,
            conditions: vec![],
            distinct: false,
        }
    }
}
#[derive(Debug, Clone)]
pub enum RdbcFilterType {
    And,
    Or,
}
#[derive(Debug, Clone)]
pub enum RdbcWhereCondition {
    Simple(RdbcWhereSimpleCondition),
    Nest(RdbcWhereNestCondition),
    Raw(RdbcWhereRawCondition),
}

#[derive(Debug, Clone)]
pub struct RdbcWhereSimpleCondition {
    pub column: RdbcColumn,
    pub compare: RdbcCompare,
    pub value: RdbcFilterValue,
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum RdbcFilterValue {
    None,
    Value(RdbcValue),
    Column(RdbcColumn),
    Query(RdbcQuery),
    Script(String),
    Raw(String),
}
#[derive(Debug, Clone)]
pub struct RdbcWhereNestCondition {
    pub condition: RdbcWhereFilter,
}
#[derive(Debug, Clone)]
pub struct RdbcWhereRawCondition {
    pub condition: String,
}
