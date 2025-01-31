use crate::types::{RdbcQuery, RdbcWhereFilter};

pub enum RdbcTable {
    SimpleTable(RdbcSimpleTable),
    QueryTable(RdbcQueryTable),
    RawTable(RdbcRawTable),
}

impl RdbcTable {
    pub(crate) fn new_with_schema_table(schema: String, table: String) -> RdbcTable {
        RdbcTable::SimpleTable(RdbcSimpleTable::new_with_schema_table(schema, table))
    }
}

impl From<(String)> for RdbcTable {
    fn from(table: String) -> Self {
        RdbcTable::SimpleTable(RdbcSimpleTable::from((table)))
    }
}
impl From<(String, String, String)> for RdbcTable {
    fn from((schema, table, alias): (String, String, String)) -> Self {
        RdbcTable::SimpleTable(RdbcSimpleTable::from((schema, table, alias)))
    }
}
impl From<(String, String)> for RdbcTable {
    fn from((table, alias): (String, String)) -> Self {
        RdbcTable::SimpleTable(RdbcSimpleTable::from((table, alias)))
    }
}
impl From<RdbcQuery> for RdbcTable {
    fn from(value: RdbcQuery) -> Self {
        RdbcTable::QueryTable(RdbcQueryTable::from(value))
    }
}
impl From<(RdbcQuery, String)> for RdbcTable {
    fn from((query, alias): (RdbcQuery, String)) -> Self {
        RdbcTable::QueryTable(RdbcQueryTable::from((query, alias)))
    }
}
pub struct RdbcSimpleTable {
    pub schema: String,
    pub table: String,
    pub alias: String,
}
impl RdbcSimpleTable {
    pub(crate) fn new_with_schema_table(schema: String, table: String) -> Self {
        RdbcSimpleTable {
            schema,
            table,
            alias: "".to_string(),
        }
    }
}
impl From<String> for RdbcSimpleTable {
    fn from(table: String) -> Self {
        RdbcSimpleTable {
            schema: "".to_string(),
            table,
            alias: "".to_string(),
        }
    }
}
impl From<(String, String)> for RdbcSimpleTable {
    fn from((table, alias): (String, String)) -> Self {
        RdbcSimpleTable {
            schema: "".to_string(),
            table,
            alias,
        }
    }
}
impl From<(String, String, String)> for RdbcSimpleTable {
    fn from((schema, table, alias): (String, String, String)) -> Self {
        RdbcSimpleTable {
            schema,
            table,
            alias,
        }
    }
}
pub struct RdbcQueryTable {
    pub query: RdbcQuery,
    pub alias: String,
}

impl From<RdbcQuery> for RdbcQueryTable {
    fn from(value: RdbcQuery) -> Self {
        RdbcQueryTable {
            query: value,
            alias: "".to_string(),
        }
    }
}
impl From<(RdbcQuery, String)> for RdbcQueryTable {
    fn from((query, alias): (RdbcQuery, String)) -> Self {
        RdbcQueryTable { query, alias }
    }
}
pub struct RdbcRawTable {
    pub table: String,
    pub alias: String,
}
impl From<String> for RdbcRawTable {
    fn from(value: String) -> Self {
        RdbcRawTable {
            table: value,
            alias: "".to_string(),
        }
    }
}
impl From<(String, String)> for RdbcRawTable {
    fn from((table, alias): (String, String)) -> Self {
        RdbcRawTable { table, alias }
    }
}

pub struct RdbcJoinTable {
    pub table: RdbcTable,
    pub join_type: JoinType,
    pub filter: Option<RdbcWhereFilter>,
}
pub enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
}
