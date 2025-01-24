use crate::types::{RdbcFunc, RdbcQuery, RdbcTable};
use bmbp_rdbc_type::RdbcValue;

pub enum RdbcColumn {
    SimpleColumn(RdbcSimpleColumn),
    TableColumn(RdbcTableColumn),
    QueryColumn(RdbcQueryColumn),
    ValueColumn(RdbcValueColumn),
    FuncColumn(RdbcFuncColumn),
    RawColumn(RdbcRawColumn),
}

impl From<String> for RdbcColumn {
    fn from(value: String) -> Self {
        RdbcColumn::SimpleColumn(RdbcSimpleColumn::from(value))
    }
}

pub struct RdbcSimpleColumn {
    pub column: String,
}
impl From<String> for RdbcSimpleColumn {
    fn from(column: String) -> Self {
        RdbcSimpleColumn { column }
    }
}

pub struct RdbcTableColumn {
    pub table: RdbcTable,
    pub column: String,
}
pub struct RdbcQueryColumn {
    pub column: RdbcQuery,
}
impl From<RdbcQuery> for RdbcQueryColumn {
    fn from(value: RdbcQuery) -> Self {
        RdbcQueryColumn { column: value }
    }
}

pub struct RdbcValueColumn {
    pub column: RdbcValue,
}
impl From<RdbcValue> for RdbcValueColumn {
    fn from(value: RdbcValue) -> Self {
        RdbcValueColumn { column: value }
    }
}

pub struct RdbcFuncColumn {
    pub func: RdbcFunc,
}
pub struct RdbcRawColumn {
    pub column: String,
}

impl From<String> for RdbcRawColumn {
    fn from(column: String) -> Self {
        RdbcRawColumn { column }
    }
}

pub struct RdbcDmlColumn {
    pub column: RdbcColumn,
    pub value: RdbcValue,
}
pub struct RdbcSelectColumn {
    pub column: RdbcColumn,
    pub alias: String,
}

impl From<RdbcColumn> for RdbcSelectColumn {
    fn from(column: RdbcColumn) -> Self {
        RdbcSelectColumn {
            column,
            alias: "".to_string(),
        }
    }
}
impl From<RdbcSimpleColumn> for RdbcSelectColumn {
    fn from(column: RdbcSimpleColumn) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(column),
            alias: "".to_string(),
        }
    }
}
impl From<(RdbcSimpleColumn, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcSimpleColumn, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(column),
            alias,
        }
    }
}
impl From<RdbcTableColumn> for RdbcSelectColumn {
    fn from(column: RdbcTableColumn) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::TableColumn(column),
            alias: "".to_string(),
        }
    }
}
impl From<(RdbcTableColumn, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcTableColumn, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::TableColumn(column),
            alias,
        }
    }
}
impl From<RdbcQueryColumn> for RdbcSelectColumn {
    fn from(column: RdbcQueryColumn) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::QueryColumn(column),
            alias: "".to_string(),
        }
    }
}
impl From<(RdbcQueryColumn, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcQueryColumn, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::QueryColumn(column),
            alias,
        }
    }
}
impl From<RdbcRawColumn> for RdbcSelectColumn {
    fn from(value: RdbcRawColumn) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::RawColumn(value),
            alias: "".to_string(),
        }
    }
}
impl From<(RdbcRawColumn, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcRawColumn, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::RawColumn(column),
            alias,
        }
    }
}

impl From<RdbcFuncColumn> for RdbcSelectColumn {
    fn from(value: RdbcFuncColumn) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::FuncColumn(value),
            alias: "".to_string(),
        }
    }
}
impl From<(RdbcFuncColumn, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcFuncColumn, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::FuncColumn(column),
            alias,
        }
    }
}
impl From<String> for RdbcSelectColumn {
    fn from(c: String) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn::from(c)),
            alias: "".to_string(),
        }
    }
}
impl From<(String, String)> for RdbcSelectColumn {
    fn from((column, alias): (String, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn::from(column)),
            alias,
        }
    }
}

impl From<RdbcValueColumn> for RdbcSelectColumn {
    fn from(column: RdbcValueColumn) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(column),
            alias: "".to_string(),
        }
    }
}

impl From<(RdbcValueColumn, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcValueColumn, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(column),
            alias,
        }
    }
}

impl From<RdbcValue> for RdbcSelectColumn {
    fn from(value: RdbcValue) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(RdbcValueColumn { column: value }),
            alias: "".to_string(),
        }
    }
}

impl From<(RdbcValue, String)> for RdbcSelectColumn {
    fn from((column, alias): (RdbcValue, String)) -> Self {
        RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(RdbcValueColumn::from(column)),
            alias,
        }
    }
}
pub struct QueryFilterColumn {
    pub column: RdbcColumn,
}

pub struct RdbcOrderColumn {
    pub column: RdbcColumn,
    pub order_type: OrderType,
}
pub enum OrderType {
    Asc,
    Desc,
}
