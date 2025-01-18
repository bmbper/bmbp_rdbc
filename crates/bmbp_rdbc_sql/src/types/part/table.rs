use crate::types::{RdbcQuery, RdbcWhereFilter};

pub enum RdbcTable {
    SimpleTable(OrmSimpleTable),
    SchemaTable(OrmSchemaTable),
    QueryTable(RdbcQueryTable),
    RawTable(RdbcRawTable)
}

pub struct OrmSimpleTable{
    pub table: String,
    pub alias: String,
}

pub struct OrmSchemaTable{
    pub schema: String,
    pub name: String,
    pub alias: String,
}
pub struct RdbcQueryTable {
    pub query: RdbcQuery,
    pub alias: String,
}
pub struct RdbcRawTable {
    pub table: String,
}


pub struct RdbcJoinTable {
    pub table: RdbcTable,
    pub join_type: JoinType,
    pub filter: RdbcWhereFilter
}
pub enum JoinType{
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
}