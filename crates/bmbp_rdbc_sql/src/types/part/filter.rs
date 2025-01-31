use crate::types::{RdbcColumn, RdbcQuery};
use bmbp_rdbc_type::RdbcValue;

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
pub enum RdbcFilterType {
    And,
    Or,
}
pub enum RdbcWhereCondition {
    Simple(RdbcWhereSimpleCondition),
    Nest(RdbcWhereNestCondition),
    Raw(RdbcWhereRawCondition),
}

pub struct RdbcWhereSimpleCondition {
    pub column: RdbcColumn,
    pub compare: RdbcCompare,
    pub value: RdbcFilterValue,
}
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
pub enum RdbcFilterValue {
    Value(RdbcValue),
    Column(RdbcColumn),
    Query(RdbcQuery),
    Script(String),
    Raw(String),
}

pub struct RdbcWhereNestCondition {
    pub condition: RdbcWhereFilter,
}

pub struct RdbcWhereRawCondition {
    pub condition: String,
}
