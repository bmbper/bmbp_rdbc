use std::sync::RwLock;

use bmbp_rdbc_type::RdbcDataBase;

use crate::{QueryWrapper, RdbcDmlValue, RdbcTableColumn, RdbcTableInner, RdbcTableWrapper};

pub struct InsertWrapper {
    driver_: RwLock<Option<RdbcDataBase>>,
    table_: Vec<RdbcTableInner>,
    join_: Option<Vec<RdbcTableInner>>,
    column_: Vec<RdbcTableColumn>,
    values_: Vec<RdbcDmlValue>,
    column_values: Vec<(RdbcTableColumn, RdbcDmlValue)>,
    query_: Option<QueryWrapper>,
}

impl InsertWrapper {
    pub fn new() -> InsertWrapper {
        InsertWrapper {
            driver_: RwLock::new(None),
            table_: Vec::new(),
            join_: None,
            column_: Vec::new(),
            values_: Vec::new(),
            column_values: Vec::new(),
            query_: None,
        }
    }
    pub fn set_driver(&self, driver: RdbcDataBase) -> &Self {
        *self.driver_.write().unwrap() = Some(driver);
        self
    }

    pub fn get_table(&self) -> &Vec<RdbcTableInner> {
        &self.table_
    }
    pub fn get_column(&self) -> &Vec<RdbcTableColumn> {
        &self.column_
    }
    pub fn get_values(&self) -> &Vec<RdbcDmlValue> {
        &self.values_
    }
    pub fn get_column_values(&self) -> &Vec<(RdbcTableColumn, RdbcDmlValue)> {
        &self.column_values
    }
    pub fn get_query(&self) -> Option<&QueryWrapper> {
        self.query_.as_ref()
    }
}

impl InsertWrapper {
    pub fn insert_query(&mut self, query: QueryWrapper) -> &mut Self {
        self.query_ = Some(query);
        self
    }

    pub fn insert_column<RC>(&mut self, column: RC) -> &mut Self
    where
        RdbcTableColumn: From<RC>,
    {
        self.column_.push(RdbcTableColumn::from(column));
        self
    }
    pub fn insert_value<RV>(&mut self, value: RV) -> &mut Self
    where
        RdbcDmlValue: From<RV>,
    {
        self.values_.push(RdbcDmlValue::from(value));
        self
    }

    pub fn insert_column_value<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcTableColumn: From<RC>,
        RdbcDmlValue: From<RV>,
    {
        self.column_values
            .push((RdbcTableColumn::from(column), RdbcDmlValue::from(value)));
        self
    }
}
impl RdbcTableWrapper for InsertWrapper {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner> {
        self.table_.as_mut()
    }
    fn get_join_mut(&mut self) -> &mut Vec<RdbcTableInner> {
        if self.join_.is_none() {
            self.join_ = Some(vec![]);
        }
        self.join_.as_mut().unwrap()
    }
}
