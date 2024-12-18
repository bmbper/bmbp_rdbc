use bmbp_rdbc_type::{RdbcDataBase, RdbcIdent, RdbcTableIdent, RdbcValue};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::{
    RdbcColumn, RdbcColumnOrder, RdbcConcatType, RdbcOrder, RdbcTableFilter, QueryFilter,
    RdbcTableInner, RdbcTableWrapper, RdbcValueColumn,
};

pub struct QueryWrapper {
    driver_: RwLock<Option<RdbcDataBase>>,
    select_: Vec<RdbcColumn>,
    table_: Vec<RdbcTableInner>,
    join_: Option<Vec<RdbcTableInner>>,
    filter_: Option<QueryFilter>,
    group_by_: Option<Vec<RdbcColumn>>,
    having_: Option<QueryFilter>,
    order_: Option<Vec<RdbcOrder>>,
    limit_: Option<u64>,
    offset_: Option<u64>,
    union_all: Option<Vec<QueryWrapper>>,
    union_only: Option<Vec<QueryWrapper>>,
    distinct_: bool,
    params_: Option<HashMap<String, RdbcValue>>,
}

/// 构造函数
impl QueryWrapper {
    pub fn new() -> QueryWrapper {
        let query = QueryWrapper {
            driver_: RwLock::new(None),
            select_: vec![],
            table_: vec![],
            join_: Some(vec![]),
            filter_: Some(QueryFilter::new()),
            group_by_: None,
            having_: None,
            order_: None,
            limit_: None,
            offset_: None,
            union_all: None,
            union_only: None,
            params_: None,
            distinct_: false,
        };
        query
    }
    pub fn new_from<T>() -> QueryWrapper
    where
        T: RdbcTableIdent,
    {
        let mut query = QueryWrapper::new();
        query.table(T::get_table().get_ident());
        for item in T::get_columns() {
            query.select(item.get_ident());
        }
        query
    }
    fn init_order(&mut self) -> &mut Self {
        if self.order_.is_none() {
            self.order_ = Some(vec![]);
        }
        self
    }
    fn init_group_by(&mut self) -> &mut Self {
        if self.group_by_.is_none() {
            self.group_by_ = Some(vec![]);
        }
        self
    }
}

// Query的查询方法
impl QueryWrapper {
    pub fn set_driver(&self, driver: RdbcDataBase) -> &Self {
        *self.driver_.write().unwrap() = Some(driver);
        self
    }

    pub fn get_select(&self) -> &Vec<RdbcColumn> {
        &self.select_
    }
    pub fn get_table(&self) -> &Vec<RdbcTableInner> {
        &self.table_
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
    pub fn get_union_all(&self) -> Option<&Vec<QueryWrapper>> {
        self.union_all.as_ref()
    }
    pub fn get_union_only(&self) -> Option<&Vec<QueryWrapper>> {
        self.union_only.as_ref()
    }
    pub fn get_distinct(&self) -> bool {
        self.distinct_.clone()
    }
    pub fn get_mut_params(&mut self) -> &mut HashMap<String, RdbcValue> {
        if self.params_.is_none() {
            self.params_ = Some(HashMap::new());
        }
        return self.params_.as_mut().unwrap();
    }
}

impl QueryWrapper {
    pub fn set_distinct(&mut self, distinct: bool) -> &mut Self {
        self.distinct_ = distinct;
        self
    }
    pub fn column<C>(&mut self, column: C) -> &mut Self
    where
        C: RdbcIdent,
    {
        self.select(column.get_ident());
        self
    }

    // RdbcColumn: From<RC>, RdbcValue: From<RV>, SS: ToString, ST: ToString, SC: ToString, SA: ToString
    pub fn select<RC>(&mut self, column: RC) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.select_.push(RdbcColumn::from(column));
        self
    }
    pub fn select_vec<RC>(&mut self, columns: Vec<RC>) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        for column in columns {
            self.select(column);
        }
        self
    }
    pub fn select_table_column<ST, SC>(&mut self, table: ST, column: SC) -> &mut Self
    where
        SC: RdbcIdent,
        ST: RdbcIdent,
    {
        self.select_.push(RdbcColumn::table_column(
            table.get_ident(),
            column.get_ident(),
        ));
        self
    }
    pub fn select_table_column_as_alias<ST, SC, SA>(
        &mut self,
        table: ST,
        column: SC,
        alias: SA,
    ) -> &mut Self
    where
        ST: RdbcIdent,
        SC: RdbcIdent,
        SA: RdbcIdent,
    {
        self.select_.push(RdbcColumn::table_column_as_alias(
            table.get_ident(),
            column.get_ident(),
            alias.get_ident(),
        ));
        self
    }
    pub fn select_schema_table_column<SS, ST, SC>(
        &mut self,
        schema: SS,
        table: ST,
        column: SC,
    ) -> &mut Self
    where
        SS: RdbcIdent,
        ST: RdbcIdent,
        SC: RdbcIdent,
    {
        self.select_.push(RdbcColumn::schema_table_column(
            schema.get_ident(),
            table.get_ident(),
            column.get_ident(),
        ));
        self
    }
    pub fn select_schema_table_column_as_alias<SS, ST, SC, SA>(
        &mut self,
        schema: SS,
        table: ST,
        column: SC,
        alias: SA,
    ) -> &mut Self
    where
        SS: ToString,
        ST: ToString,
        SC: ToString,
        SA: ToString,
    {
        self.select_.push(RdbcColumn::schema_table_column_as_alias(
            schema, table, column, alias,
        ));
        self
    }
    pub fn select_value<RV>(&mut self, column: RV) -> &mut Self
    where
        RdbcValue: From<RV>,
    {
        self.select_
            .push(RdbcColumn::Value(RdbcValueColumn::rdbc_value(
                RdbcValue::from(column),
            )));
        self
    }
    pub fn select_value_alias<RV, RA>(&mut self, column: RV, alias: RA) -> &mut Self
    where
        RdbcValue: From<RV>,
        RA: RdbcIdent,
    {
        self.select_
            .push(RdbcColumn::Value(RdbcValueColumn::rdbc_value_alias(
                RdbcValue::from(column),
                alias.get_ident(),
            )));
        self
    }

    pub fn order_by<SC>(&mut self, column: SC, is_asc: bool) -> &mut Self
    where
        RdbcColumn: From<SC>,
    {
        self.init_order();
        let order = match is_asc {
            true => RdbcColumnOrder::asc_(column),
            false => RdbcColumnOrder::desc_(column),
        };
        self.order_.as_mut().unwrap().push(RdbcOrder::Column(order));
        self
    }
    pub fn order_asc<SC>(&mut self, column: SC) -> &mut Self
    where
        RdbcColumn: From<SC>,
    {
        self.init_order();
        self.order_
            .as_mut()
            .unwrap()
            .push(RdbcOrder::Column(RdbcColumnOrder::asc_(column)));
        self
    }
    pub fn order_desc<SC>(&mut self, column: SC) -> &mut Self
    where
        RdbcColumn: From<SC>,
    {
        self.init_order();
        self.order_
            .as_mut()
            .unwrap()
            .push(RdbcOrder::Column(RdbcColumnOrder::desc_(column)));
        self
    }
    pub fn order_slice<SC>(&mut self, column: &[SC], is_asc: bool) -> &mut Self
    where
        SC: RdbcIdent,
    {
        for item in column {
            self.order_by(RdbcColumn::from(item.get_ident()), is_asc);
        }
        self
    }
    pub fn order_slice_asc<SC>(&mut self, column: &[SC]) -> &mut Self
    where
        SC: ToString,
    {
        for item in column {
            self.order_by(RdbcColumn::from(item.get_ident()), true);
        }

        self
    }
    pub fn order_slice_desc<SC>(&mut self, column: &[SC]) -> &mut Self
    where
        SC: ToString,
    {
        for item in column {
            self.order_by(RdbcColumn::from(item.get_ident()), false);
        }
        self
    }
    pub fn order_column(&mut self, column: RdbcColumn, is_asc: bool) -> &mut Self {
        self.order_by(column, is_asc);
        self
    }
    pub fn order_column_vec(&mut self, column: Vec<RdbcColumn>, is_asc: bool) -> &mut Self {
        for item in column {
            self.order_by(item, is_asc);
        }
        self
    }

    pub fn order_column_asc(&mut self, column: RdbcColumn) -> &mut Self {
        self.order_by(column, true);
        self
    }
    pub fn order_column_vec_asc(&mut self, column: Vec<RdbcColumn>) -> &mut Self {
        for item in column {
            self.order_by(item, true);
        }
        self
    }

    pub fn order_column_desc(&mut self, column: RdbcColumn) -> &mut Self {
        self.order_by(column, false);
        self
    }
    pub fn order_column_vec_desc(&mut self, column: Vec<RdbcColumn>) -> &mut Self {
        for item in column {
            self.order_by(item, false);
        }
        self
    }

    pub fn group_by<RC>(&mut self, column: RC) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.init_group_by();
        self.group_by_
            .as_mut()
            .unwrap()
            .push(RdbcColumn::from(column));
        self
    }
}

impl RdbcTableWrapper for QueryWrapper {
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

impl RdbcTableFilter for QueryWrapper {
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
