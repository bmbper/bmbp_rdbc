use bmbp_rdbc_type::{RdbcDataBase, RdbcValue};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::{
    RdbcColumn, RdbcConcatType, RdbcDmlValue, RdbcOrder, RdbcTableFilter, QueryFilter,
    RdbcTableInner, RdbcTableWrapper,
};

pub struct UpdateWrapper {
    driver_: RwLock<Option<RdbcDataBase>>,
    set_values_: Vec<(RdbcColumn, Option<RdbcDmlValue>)>,
    table_: Vec<RdbcTableInner>,
    join_: Option<Vec<RdbcTableInner>>,
    filter_: Option<QueryFilter>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<QueryFilter>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    params_: Option<HashMap<String, RdbcValue>>,
}

impl UpdateWrapper {
    pub fn new() -> UpdateWrapper {
        UpdateWrapper {
            driver_: RwLock::new(None),
            set_values_: vec![],
            table_: Vec::new(),
            join_: None,
            filter_: None,
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            params_: None,
        }
    }
    pub fn set_driver(&self, driver: RdbcDataBase) -> &Self {
        *self.driver_.write().unwrap() = Some(driver);
        self
    }
    pub fn get_table(&self) -> &Vec<RdbcTableInner> {
        &self.table_
    }
    pub fn get_set_values(&self) -> &Vec<(RdbcColumn, Option<RdbcDmlValue>)> {
        &self.set_values_
    }
    pub fn get_join(&self) -> Option<&Vec<RdbcTableInner>> {
        self.join_.as_ref()
    }
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter_.as_ref()
    }
    pub fn get_group_by(&self) -> Option<&Vec<RdbcColumn>> {
        self.group_by_.as_ref()
    }
    pub fn get_having(&self) -> Option<&QueryFilter> {
        self.having_.as_ref()
    }
    pub fn get_order(&self) -> Option<&Vec<RdbcOrder>> {
        self.order_.as_ref()
    }
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit_.as_ref()
    }
    pub fn get_offset(&self) -> Option<&u64> {
        self.offset_.as_ref()
    }
    pub fn get_params_mut(&mut self) -> Option<&mut HashMap<String, RdbcValue>> {
        self.params_.as_mut()
    }
}

impl UpdateWrapper {
    pub fn set<SC, RV>(&mut self, column: SC, value: RV) -> &mut Self
    where
        RdbcColumn: From<SC>,
        RdbcDmlValue: From<RV>,
    {
        self.set_values_
            .push((RdbcColumn::from(column), Some(RdbcDmlValue::from(value))));
        self
    }
}

impl RdbcTableWrapper for UpdateWrapper {
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

impl RdbcTableFilter for UpdateWrapper {
    fn init_filter(&mut self) -> &mut Self {
        if self.filter_.is_none() {
            self.filter_ = Some(QueryFilter::new());
        }
        self
    }
    fn get_filter_mut(&mut self) -> &mut QueryFilter {
        self.init_filter();
        self.filter_.as_mut().unwrap()
    }
    fn with_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self {
        let filter_ = {
            if self.filter_.is_some() {
                QueryFilter::concat_with_filter(concat_type, self.filter_.take().unwrap())
            } else {
                QueryFilter::concat(concat_type)
            }
        };
        self.filter_ = Some(filter_);
        self
    }
}
