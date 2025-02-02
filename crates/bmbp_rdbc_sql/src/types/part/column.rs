use crate::types::{RdbcFunc, RdbcQuery, RdbcTable};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};

#[derive(Debug, Clone)]
pub enum RdbcColumn {
    SimpleColumn(RdbcSimpleColumn),
    TableColumn(RdbcTableColumn),
    QueryColumn(RdbcQueryColumn),
    ValueColumn(RdbcValueColumn),
    FuncColumn(RdbcFuncColumn),
    RawColumn(RdbcRawColumn),
}

impl<T> From<T> for RdbcColumn
where
    T: RdbcIdent,
{
    fn from(value: T) -> Self {
        RdbcColumn::SimpleColumn(RdbcSimpleColumn::from(value))
    }
}

impl From<RdbcQueryColumn> for RdbcColumn {
    fn from(value: RdbcQueryColumn) -> Self {
        RdbcColumn::QueryColumn(value)
    }
}
impl From<RdbcValueColumn> for RdbcColumn {
    fn from(value: RdbcValueColumn) -> Self {
        RdbcColumn::ValueColumn(value)
    }
}
impl From<RdbcQuery> for RdbcColumn {
    fn from(value: RdbcQuery) -> Self {
        RdbcColumn::from(RdbcQueryColumn::from(value))
    }
}
#[derive(Debug, Clone)]
pub struct RdbcSimpleColumn {
    pub column: String,
}
impl<T> From<T> for RdbcSimpleColumn
where
    T: RdbcIdent,
{
    fn from(column: T) -> Self {
        RdbcSimpleColumn {
            column: column.name(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RdbcTableColumn {
    pub table: RdbcTable,
    pub column: String,
}
#[derive(Debug, Clone)]
pub struct RdbcQueryColumn {
    pub column: RdbcQuery,
}
impl From<RdbcQuery> for RdbcQueryColumn {
    fn from(value: RdbcQuery) -> Self {
        RdbcQueryColumn { column: value }
    }
}
#[derive(Debug, Clone)]
pub struct RdbcValueColumn {
    pub column: RdbcValue,
}
impl From<RdbcValue> for RdbcValueColumn {
    fn from(value: RdbcValue) -> Self {
        RdbcValueColumn { column: value }
    }
}
#[derive(Debug, Clone)]
pub struct RdbcFuncColumn {
    pub func: RdbcFunc,
}
#[derive(Debug, Clone)]
pub struct RdbcRawColumn {
    pub column: String,
}

impl From<String> for RdbcRawColumn {
    fn from(column: String) -> Self {
        RdbcRawColumn { column }
    }
}

#[derive(Debug, Clone)]
pub struct RdbcDmlColumn {
    pub column: RdbcColumn,
    pub value: RdbcDmlValue,
}
#[derive(Debug, Clone)]
pub enum RdbcDmlValue {
    VALUE(RdbcValue),
    COLUMN(RdbcColumn),
    FUNC(RdbcFunc),
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct RdbcOrderColumn {
    pub column: RdbcColumn,
    pub order_type: RdbcOrderType,
}
#[derive(Debug, Clone)]
pub struct RdbcGroupColumn {
    pub column: RdbcColumn,
}
impl From<RdbcColumn> for RdbcGroupColumn {
    fn from(value: RdbcColumn) -> Self {
        RdbcGroupColumn { column: value }
    }
}
#[derive(Debug, Clone)]
pub enum RdbcOrderType {
    Asc,
    Desc,
}
