use crate::types::{RdbcColumn, RdbcQuery};
use bmbp_rdbc_type::RdbcValue;

pub struct RdbcWhereFilter {
    pub type_: RdbcFilterType,
    pub conditions: Vec<RdbcWhereCondition>,
}
impl RdbcWhereFilter {
    pub fn new()->Self{
        RdbcWhereFilter{
            type_: RdbcFilterType::And,
            conditions: vec![],
        }
    }
    
}
pub enum RdbcFilterType {
    And,
    Or,
}

pub enum RdbcWhereCondition {
    Simple(RdbcWhereSimpleCondition),
    NEST(RdbcWhereNestCondition),
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
    LIKE,
    IN,
    NotIn,
    IsNull,
    IsNotNull,
    BETWEEN,
    NotBetween,
}
pub enum RdbcFilterValue {
    Value(RdbcValue),
    Column(RdbcColumn),
    Query(RdbcQuery),
}

pub struct RdbcWhereNestCondition {
    pub condition: RdbcWhereFilter,
}

pub struct RdbcWhereRawCondition {
    pub condition: String,
}
