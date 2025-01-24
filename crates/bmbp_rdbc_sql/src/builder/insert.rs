use crate::types::RdbcInsert;
use crate::{RdbcColumn, RdbcDmlColumn, RdbcDmlValue, RdbcQuery, RdbcSimpleTable, RdbcTable};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};
pub struct RdbcInsertBuilder {
    insert: RdbcInsert,
}
impl RdbcInsertBuilder {
    pub fn build(&self) -> &RdbcInsert {
        &self.insert
    }
}

impl RdbcInsertBuilder {
    pub fn new() -> Self {
        RdbcInsertBuilder {
            insert: RdbcInsert::new(),
        }
    }
    pub fn insert_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.insert.table = Some(RdbcTable::SimpleTable(RdbcSimpleTable::from(table.name())));
        self
    }
    pub fn insert_col<T>(&mut self, col: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.insert.column.push(col.name());
        self
    }
    pub fn insert_cols<T>(&mut self, col: Vec<T>) -> &mut Self
    where
        T: RdbcIdent,
    {
        for item in col {
            self.insert.column.push(item.name());
        }
        self
    }
    pub fn insert_value<V>(&mut self, v: V) -> &mut Self
    where
        RdbcValue: From<V>,
    {
        self.insert
            .values
            .push(RdbcDmlValue::VALUE(RdbcValue::from(v)));
        self
    }
    pub fn insert_values<V>(&mut self, v: Vec<V>) -> &mut Self
    where
        RdbcValue: From<V>,
    {
        for item in v {
            self.insert
                .values
                .push(RdbcDmlValue::VALUE(RdbcValue::from(item)));
        }
        self
    }

    pub fn insert_col_val<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.insert_col(column).insert_value(value);
        self
    }

    pub fn insert_dml_col(&mut self, column: RdbcColumn, value: RdbcDmlValue) -> &mut Self {
        self.insert
            .dml_columns
            .push(RdbcDmlColumn { column, value });
        self
    }
    pub fn insert_query(&mut self, query: RdbcQuery) -> &mut Self {
        self.insert.query = Some(query);
        self
    }
}
