use std::collections::HashMap;
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};

use crate::{QueryWrapper, RdbcFunc};

/// RdbcColumn SELECT 返回列

pub enum RdbcColumn {
    Table(RdbcTableColumn),
    Query(RdbcQueryColumn),
    Func(RdbcFuncColumn),
    Value(RdbcValueColumn),
}

impl RdbcColumn {
    pub fn is_value(&self) -> bool {
        match self {
            RdbcColumn::Value(_) => true,
            _ => false,
        }
    }

    pub fn get_value(&self) -> Option<&RdbcValue> {
        match self {
            RdbcColumn::Value(column) => Some(column.get_name()),
            _ => None,
        }
    }
    pub fn is_query(&self) -> bool {
        match self {
            RdbcColumn::Query(_) => true,
            _ => false,
        }
    }
    pub fn get_query(&self) -> Option<&QueryWrapper> {
        match self {
            RdbcColumn::Query(column) => Some(column.get_name()),
            _ => None,
        }
    }
}

impl From<RdbcTableColumn> for RdbcColumn {
    fn from(column: RdbcTableColumn) -> Self {
        RdbcColumn::Table(column)
    }
}

impl From<RdbcQueryColumn> for RdbcColumn {
    fn from(column: RdbcQueryColumn) -> Self {
        RdbcColumn::Query(column)
    }
}

impl From<RdbcFuncColumn> for RdbcColumn {
    fn from(column: RdbcFuncColumn) -> Self {
        RdbcColumn::Func(column)
    }
}

impl From<RdbcValueColumn> for RdbcColumn {
    fn from(column: RdbcValueColumn) -> Self {
        RdbcColumn::Value(column)
    }
}

/// impl RdbcIdent impl RdbcColumn
impl<T> From<T> for RdbcColumn where T: RdbcIdent {
    fn from(value: T) -> Self {
        RdbcColumn::Table(RdbcTableColumn::column(value.get_ident()))
    }
}


impl RdbcColumn {
    pub fn column<T>(name: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Table(RdbcTableColumn::column(name))
    }
    pub fn column_as_alias<T, E>(name: T, alias: E) -> RdbcColumn
        where
            T: ToString,
            E: ToString,
    {
        RdbcColumn::Table(RdbcTableColumn::column_as_alias(name, alias))
    }
    pub fn table_column<T, C>(table: T, name: C) -> RdbcColumn
        where
            T: ToString,
            C: ToString,
    {
        RdbcColumn::Table(RdbcTableColumn::table_column(table, name))
    }
    pub fn table_column_as_alias<ST, SC, SA>(table: ST, column: SC, alias: SA) -> RdbcColumn
        where
            ST: ToString,
            SC: ToString,
            SA: ToString,
    {
        RdbcColumn::Table(RdbcTableColumn::table_column_as_alias(table, column, alias))
    }

    pub fn schema_table_column<S, T, C>(schema: S, table: T, name: C) -> RdbcColumn
        where
            S: ToString,
            T: ToString,
            C: ToString,
    {
        RdbcColumn::Table(RdbcTableColumn::schema_table_column(schema, table, name))
    }
    pub fn schema_table_column_as_alias<S, T, C, A>(
        schema: S,
        table: T,
        name: C,
        alias: A,
    ) -> RdbcColumn
        where
            S: ToString,
            T: ToString,
            C: ToString,
            A: ToString,
    {
        RdbcColumn::Table(RdbcTableColumn::schema_table_column_as_alias(
            schema, table, name, alias,
        ))
    }
    pub fn rdbc_value(value: RdbcValue) -> RdbcColumn {
        RdbcColumn::Value(RdbcValueColumn::rdbc_value(value))
    }

    pub fn rdbc_value_alias<T>(value: RdbcValue, alias: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Value(RdbcValueColumn::rdbc_value_alias(value, alias))
    }
    pub fn raw_value<T>(value: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Value(RdbcValueColumn::raw_value(value))
    }
    pub fn raw_value_alias<T>(value: T, alias: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Value(RdbcValueColumn::raw_value_alias(value, alias))
    }

    pub fn string_value<T>(value: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Value(RdbcValueColumn::string_value(value))
    }
    pub fn string_value_alias<T>(value: T, alias: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Value(RdbcValueColumn::string_value_alias(value, alias))
    }
    pub fn query(query: QueryWrapper) -> RdbcColumn {
        RdbcColumn::Query(RdbcQueryColumn::query(query))
    }
    pub fn query_alias<T>(query: QueryWrapper, alias: T) -> RdbcColumn
        where
            T: ToString,
    {
        RdbcColumn::Query(RdbcQueryColumn::query_alias(query, alias))
    }

    pub fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcColumn {
        RdbcColumn::Func(RdbcFuncColumn::concat_split(columns, split_))
    }
    pub fn concat(columns: Vec<RdbcColumn>) -> RdbcColumn {
        RdbcColumn::Func(RdbcFuncColumn::concat(columns))
    }
    pub fn replace<C, OV, NV>(column: C, old_value: OV, new_value: NV) -> RdbcColumn
        where
            RdbcTableColumn: From<C>,
            OV: ToString,
            NV: ToString,
    {
        RdbcColumn::Func(RdbcFuncColumn::replace(
            RdbcTableColumn::from(column),
            old_value.to_string(),
            new_value.to_string(),
        ))
    }
}

pub struct RdbcTableColumn {
    schema_: Option<String>,
    table_: Option<String>,
    name_: String,
    alias_: Option<String>,
}

impl RdbcTableColumn {
    pub fn get_schema(&self) -> Option<&String> {
        self.schema_.as_ref()
    }
    pub fn get_table(&self) -> Option<&String> {
        self.table_.as_ref()
    }
    pub fn get_name(&self) -> &String {
        &self.name_
    }
    pub fn get_alias(&self) -> Option<&String> {
        self.alias_.as_ref()
    }
}

impl RdbcTableColumn {
    fn column<T>(name: T) -> RdbcTableColumn
        where
            T: ToString,
    {
        RdbcTableColumn {
            schema_: None,
            table_: None,
            name_: name.to_string(),
            alias_: None,
        }
    }
    fn column_as_alias<T, E>(name: T, alias: E) -> RdbcTableColumn
        where
            T: ToString,
            E: ToString,
    {
        RdbcTableColumn {
            schema_: None,
            table_: None,
            name_: name.to_string(),
            alias_: Some(alias.to_string()),
        }
    }
    fn table_column<T, C>(table: T, name: C) -> RdbcTableColumn
        where
            T: ToString,
            C: ToString,
    {
        RdbcTableColumn {
            schema_: None,
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: None,
        }
    }
    fn table_column_as_alias<ST, SC, SA>(table: ST, name: SC, alias: SA) -> RdbcTableColumn
        where
            ST: ToString,
            SC: ToString,
            SA: ToString,
    {
        RdbcTableColumn {
            schema_: None,
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: Some(alias.to_string()),
        }
    }
    fn schema_table_column<S, T, C>(schema: S, table: T, name: C) -> RdbcTableColumn
        where
            S: ToString,
            T: ToString,
            C: ToString,
    {
        RdbcTableColumn {
            schema_: Some(schema.to_string()),
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: None,
        }
    }
    fn schema_table_column_as_alias<S, T, C, A>(
        schema: S,
        table: T,
        name: C,
        alias: A,
    ) -> RdbcTableColumn
        where
            S: ToString,
            T: ToString,
            C: ToString,
            A: ToString,
    {
        RdbcTableColumn {
            schema_: Some(schema.to_string()),
            table_: Some(table.to_string()),
            name_: name.to_string(),
            alias_: Some(alias.to_string()),
        }
    }
}

impl<T> From<T> for RdbcTableColumn
    where
        T: ToString,
{
    fn from(value: T) -> Self {
        RdbcTableColumn::column(value)
    }
}

pub struct RdbcValueColumn {
    name_: RdbcValue,
    alias_: Option<String>,
}

impl RdbcValueColumn {
    pub fn get_name(&self) -> &RdbcValue {
        &self.name_
    }
    pub fn get_alias(&self) -> Option<&String> {
        self.alias_.as_ref()
    }
}

impl RdbcValueColumn {
    pub fn rdbc_value(value: RdbcValue) -> RdbcValueColumn {
        RdbcValueColumn {
            name_: value,
            alias_: None,
        }
    }
    fn rdbc_value_alias<T>(value: RdbcValue, alias: T) -> RdbcValueColumn
        where
            T: ToString,
    {
        RdbcValueColumn {
            name_: value,
            alias_: Some(alias.to_string()),
        }
    }
    fn raw_value<T>(value: T) -> RdbcValueColumn
        where
            T: ToString,
    {
        RdbcValueColumn {
            name_: RdbcValue::String(value.to_string()),
            alias_: None,
        }
    }
    fn raw_value_alias<T>(value: T, alias: T) -> RdbcValueColumn
        where
            T: ToString,
    {
        RdbcValueColumn {
            name_: RdbcValue::String(value.to_string()),
            alias_: Some(alias.to_string()),
        }
    }

    fn string_value<T>(value: T) -> RdbcValueColumn
        where
            T: ToString,
    {
        RdbcValueColumn {
            name_: RdbcValue::String(format!("'{}'", value.to_string())),
            alias_: None,
        }
    }
    fn string_value_alias<T>(value: T, alias: T) -> RdbcValueColumn
        where
            T: ToString,
    {
        RdbcValueColumn {
            name_: RdbcValue::String(format!("'{}'", value.to_string())),
            alias_: Some(alias.to_string()),
        }
    }
}

pub struct RdbcFuncColumn {
    columns_: RdbcFunc,
    alias_: Option<String>,
}

impl RdbcFuncColumn {
    pub fn get_name(&self) -> &RdbcFunc {
        &self.columns_
    }
    pub fn get_alias(&self) -> Option<&String> {
        self.alias_.as_ref()
    }
}

impl RdbcFuncColumn {
    fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcFuncColumn {
        RdbcFuncColumn {
            columns_: RdbcFunc::concat_split(columns, split_),
            alias_: None,
        }
    }
    fn concat(columns: Vec<RdbcColumn>) -> RdbcFuncColumn {
        RdbcFuncColumn {
            columns_: RdbcFunc::concat(columns),
            alias_: None,
        }
    }
    fn replace(column: RdbcTableColumn, old_value: String, new_value: String) -> RdbcFuncColumn {
        RdbcFuncColumn {
            columns_: RdbcFunc::replace(column, old_value, new_value),
            alias_: None,
        }
    }
}

pub struct RdbcQueryColumn {
    name_: QueryWrapper,
    alias_: Option<String>,
}

impl RdbcQueryColumn {
    pub fn get_name(&self) -> &QueryWrapper {
        &self.name_
    }
    pub fn get_alias(&self) -> Option<&String> {
        self.alias_.as_ref()
    }
}

impl RdbcQueryColumn {
    fn query(query: QueryWrapper) -> RdbcQueryColumn {
        RdbcQueryColumn {
            name_: query,
            alias_: None,
        }
    }
    fn query_alias<T>(query: QueryWrapper, alias: T) -> RdbcQueryColumn
        where
            T: ToString,
    {
        RdbcQueryColumn {
            name_: query,
            alias_: Some(alias.to_string()),
        }
    }
}

pub enum RdbcTableInner {
    Table(RdbcSchemaTable),
    Query(RdbcQueryTable),
}

impl RdbcTableInner {
    pub fn get_join(&self) -> Option<&RdbcTableJoinType> {
        match self {
            RdbcTableInner::Table(table) => table.get_join(),
            RdbcTableInner::Query(query) => query.get_join(),
        }
    }
    pub fn get_filter(&self) -> Option<&RdbcTableFilterImpl> {
        match self {
            RdbcTableInner::Table(table) => table.get_filter(),
            RdbcTableInner::Query(query) => query.get_filter(),
        }
    }
}

impl RdbcTableInner {
    pub(crate) fn table<T>(table: T) -> RdbcTableInner
        where
            T: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::table(table))
    }
    pub fn table_alias<T, V>(table: T, alias: V) -> RdbcTableInner
        where
            T: ToString,
            V: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::table_alias(table, alias))
    }
    pub fn schema_table<T>(schema: T, table: T) -> RdbcTableInner
        where
            T: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::schema_table(schema, table))
    }
    pub fn schema_table_alias<SS, ST, SA>(schema: SS, table: ST, alias: SA) -> RdbcTableInner
        where
            SS: ToString,
            ST: ToString,
            SA: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::schema_table_alias(schema, table, alias))
    }
    pub(crate) fn left_join_table<T>(table: T) -> Self
        where
            T: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::left_join_table(table))
    }
    pub fn temp_table(table: QueryWrapper) -> RdbcTableInner {
        RdbcTableInner::Query(RdbcQueryTable::query(table))
    }
    pub fn temp_table_alias<T>(table: QueryWrapper, alias: T) -> RdbcTableInner
        where
            T: ToString,
    {
        RdbcTableInner::Query(RdbcQueryTable::query_alias(table, alias))
    }
    fn query(table: QueryWrapper) -> RdbcTableInner {
        RdbcTableInner::Query(RdbcQueryTable::query(table))
    }
}

impl RdbcTableInner {
    pub fn join(&mut self, join_: RdbcTableJoinType) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.left_join();
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }
    pub fn left_join(&mut self) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.left_join();
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }
    fn eq_<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.eq(column, value);
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }
    pub fn or(&mut self) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.or();
            }
            RdbcTableInner::Query(ref mut table) => {}
        }
        self
    }

    pub fn on_eq<T, V, E, F>(&mut self, t1: T, c1: V, t2: E, c2: F) -> &mut Self
        where
            T: ToString,
            E: ToString,
            V: ToString,
            F: ToString,
    {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.eq_column(simple_column(t1, c1), simple_column(t2, c2));
            }
            RdbcTableInner::Query(ref mut table) => {
                table.eq_column(simple_column(t1, c1), simple_column(t2, c2));
            }
        }
        self
    }

    pub fn on_eq_col<RT, RC>(&mut self, column1: RT, column2: RC) -> &mut Self
        where
            RdbcColumn: From<RT>,
            RdbcColumn: From<RC>,
    {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.eq_column(RdbcColumn::from(column1), RdbcColumn::from(column2));
            }
            RdbcTableInner::Query(ref mut table) => {
                table.eq_column(RdbcColumn::from(column1), RdbcColumn::from(column2));
            }
        }
        self
    }
}

pub enum RdbcTableJoinType {
    Left,
    Right,
    Inner,
    Full,
}

pub struct RdbcSchemaTable {
    schema_: Option<String>,
    name_: String,
    alias_: Option<String>,
    join_: Option<RdbcTableJoinType>,
    filter_: Option<RdbcTableFilterImpl>,
    params_: Option<HashMap<String, RdbcValue>>,
}

impl RdbcSchemaTable {
    pub fn get_schema(&self) -> Option<&String> {
        self.schema_.as_ref()
    }
    pub fn get_name(&self) -> &String {
        &self.name_
    }
    pub fn get_alias(&self) -> Option<&String> {
        self.alias_.as_ref()
    }
    pub fn get_join(&self) -> Option<&RdbcTableJoinType> {
        self.join_.as_ref()
    }
    pub fn get_filter(&self) -> Option<&RdbcTableFilterImpl> {
        self.filter_.as_ref()
    }
}

impl RdbcSchemaTable {
    fn table<T>(table: T) -> RdbcSchemaTable
        where
            T: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: None,
            filter_: Some(RdbcTableFilterImpl::new()),
            params_: None,
        }
    }
    fn table_alias<T, V>(table: T, alias: V) -> RdbcSchemaTable
        where
            T: ToString,
            V: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: Some(RdbcTableFilterImpl::new()),
            params_: None,
        }
    }
    fn schema_table<T>(schema: T, table: T) -> RdbcSchemaTable
        where
            T: ToString,
    {
        RdbcSchemaTable {
            schema_: Some(schema.to_string()),
            name_: table.to_string(),
            alias_: None,
            join_: None,
            filter_: Some(RdbcTableFilterImpl::new()),
            params_: None,
        }
    }
    fn schema_table_alias<SS, ST, SA>(schema: SS, table: ST, alias: SA) -> RdbcSchemaTable
        where
            SS: ToString,
            ST: ToString,
            SA: ToString,
    {
        RdbcSchemaTable {
            schema_: Some(schema.to_string()),
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: Some(RdbcTableFilterImpl::new()),
            params_: None,
        }
    }
    fn left_join_table<T>(table: T) -> RdbcSchemaTable
        where
            T: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: Some(RdbcTableJoinType::Left),
            filter_: Some(RdbcTableFilterImpl::new()),
            params_: None,
        }
    }
    fn left_join_table_alias<T, A>(table: T, alias: A) -> RdbcSchemaTable
        where
            T: ToString,
            A: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: Some(RdbcTableJoinType::Left),
            filter_: Some(RdbcTableFilterImpl::new()),
            params_: None,
        }
    }
}

impl RdbcSchemaTable {
    fn create_filter(&mut self, concat: RdbcConcatType) -> &mut Self {
        let filter = self.filter_.take().unwrap();
        let new_filter = RdbcTableFilterImpl::concat_with_filter(concat, filter);
        self.filter_ = Some(new_filter);
        self
    }
    pub fn left_join(&mut self) -> &mut Self {
        self.join_ = Some(RdbcTableJoinType::Left);
        self
    }
    pub fn eq<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        self.filter_.as_mut().unwrap().eq_(column, value);
        self
    }
    pub fn eq_column(&mut self, col: RdbcColumn, val: RdbcColumn) -> &mut Self {
        self.filter_.as_mut().unwrap().eq_column(col, val);
        self
    }
    pub fn or(&mut self) -> &mut Self {
        self.create_filter(RdbcConcatType::Or);
        self
    }
    pub fn and(&mut self) -> &mut Self {
        self.create_filter(RdbcConcatType::And);
        self
    }
}

pub struct RdbcQueryTable {
    name_: QueryWrapper,
    alias_: Option<String>,
    join_: Option<RdbcTableJoinType>,
    filter_: Option<RdbcTableFilterImpl>,
}

impl RdbcQueryTable {
    pub fn get_name(&self) -> &QueryWrapper {
        &self.name_
    }
    pub fn get_alias(&self) -> Option<&String> {
        self.alias_.as_ref()
    }
    pub fn get_join(&self) -> Option<&RdbcTableJoinType> {
        self.join_.as_ref()
    }
    pub fn get_filter(&self) -> Option<&RdbcTableFilterImpl> {
        self.filter_.as_ref()
    }
}

impl RdbcQueryTable {
    fn query(table: QueryWrapper) -> RdbcQueryTable {
        RdbcQueryTable {
            name_: table,
            alias_: None,
            join_: None,
            filter_: None,
        }
    }
    fn query_alias<T>(table: QueryWrapper, alias: T) -> RdbcQueryTable
        where
            T: ToString,
    {
        RdbcQueryTable {
            name_: table,
            alias_: Some(alias.to_string()),
            join_: None,
            filter_: None,
        }
    }
    pub fn eq_column(&mut self, col: RdbcColumn, val: RdbcColumn) -> &mut Self {
        self.filter_.as_mut().unwrap().eq_column(col, val);
        self
    }
}

pub enum RdbcConcatType {
    And,
    Or,
}

/// RdbcTableFilterImpl 表查询条件实现
pub struct RdbcTableFilterImpl {
    concat_: RdbcConcatType,
    item_: Vec<RdbcFilterItem>,
    params_: Option<HashMap<String, RdbcValue>>,
}

impl RdbcTableFilterImpl {
    pub fn get_concat(&self) -> &RdbcConcatType {
        &self.concat_
    }
    pub fn get_item(&self) -> &Vec<RdbcFilterItem> {
        &self.item_
    }
}

impl RdbcTableFilterImpl {
    pub fn add_filter(&mut self, filter: RdbcTableFilterImpl) -> &mut Self {
        self.item_.push(RdbcFilterItem::Filter(filter));
        self
    }
    pub(crate) fn eq_<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self.item_.push(RdbcFilterItem::eq_(column, value));
        self
    }
    pub(crate) fn ne_<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self.item_.push(RdbcFilterItem::ne_(column, value));
        self
    }
    pub fn eq_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(RdbcFilterItem::eq_column(column, value));
        self
    }
    pub fn like_value<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        self.item_.push(RdbcFilterItem::like_value(column, value));
        self
    }
    pub fn like_left_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcColumn: From<RV>,
    {
        self.item_
            .push(RdbcFilterItem::like_left_col(column, value));
        self
    }
    pub fn like_left_value<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        self.item_
            .push(RdbcFilterItem::like_left_value(column, value));
        self
    }
    pub fn not_like_left_value<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        self.item_
            .push(RdbcFilterItem::not_like_left_value(column, value));
        self
    }
    pub fn in_v<RC, RV>(&mut self, column: RC, value: Vec<RV>) -> &mut Self where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV> {
        self.item_
            .push(RdbcFilterItem::in_v(column, value));
        self
    }

    pub fn in_v_slice<RC, RV>(&mut self, column: RC, value: &[RV]) -> &mut Self where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
        RV: Clone {
        self.item_
            .push(RdbcFilterItem::in_v_slice(column, value));
        self
    }
}

impl RdbcTableFilterImpl {
    pub fn new() -> RdbcTableFilterImpl {
        RdbcTableFilterImpl {
            concat_: RdbcConcatType::And,
            item_: vec![],
            params_: None,
        }
    }
    pub fn concat(concat: RdbcConcatType) -> RdbcTableFilterImpl {
        RdbcTableFilterImpl {
            concat_: concat,
            item_: vec![],
            params_: None,
        }
    }
    pub fn concat_with_filter(concat: RdbcConcatType, filter: RdbcTableFilterImpl) -> RdbcTableFilterImpl {
        RdbcTableFilterImpl {
            concat_: concat,
            item_: vec![RdbcFilterItem::filter(filter)],
            params_: None,
        }
    }
}

pub enum RdbcFilterItem {
    Value(RdbcValueFilterItem),
    Column(RdbcColumnFilterItem),
    Filter(RdbcTableFilterImpl),
}

impl RdbcFilterItem {
    fn filter(filter: RdbcTableFilterImpl) -> RdbcFilterItem {
        RdbcFilterItem::Filter(filter)
    }
}

impl RdbcFilterItem {
    pub(crate) fn eq_<T, V>(column: T, value: V) -> RdbcFilterItem
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        RdbcFilterItem::Value(RdbcValueFilterItem::eq_(column, value))
    }
    pub(crate) fn ne_<T, V>(column: T, value: V) -> RdbcFilterItem
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        RdbcFilterItem::Value(RdbcValueFilterItem::ne_(column, value))
    }
    pub fn eq_column(column: RdbcColumn, value: RdbcColumn) -> RdbcFilterItem {
        RdbcFilterItem::Column(RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::Eq,
            value: Some(value),
        })
    }
    pub fn like_left_col<RC, RV>(column: RC, value: RV) -> RdbcFilterItem
        where
            RdbcColumn: From<RC>,
            RdbcColumn: From<RV>,
    {
        RdbcFilterItem::Column(RdbcColumnFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::LikeLeft,
            value: Some(RdbcColumn::from(value)),
        })
    }
    pub fn like_value<RC, RV>(column: RC, value: RV) -> RdbcFilterItem
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        RdbcFilterItem::Value(RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::Like,
            value: Some(RdbcValue::from(value)),
            ignore_null: true,
        })
    }
    pub fn like_left_value<RC, RV>(column: RC, value: RV) -> RdbcFilterItem
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        RdbcFilterItem::Value(RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::LikeLeft,
            value: Some(RdbcValue::from(value)),
            ignore_null: true,
        })
    }
    pub fn not_like_left_value<RC, RV>(column: RC, value: RV) -> RdbcFilterItem
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        RdbcFilterItem::Value(RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::NotLikeLeft,
            value: Some(RdbcValue::from(value)),
            ignore_null: true,
        })
    }
    pub fn in_v<RC, RV>(column: RC, value_vec: Vec<RV>) -> RdbcFilterItem
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        let mut temp_value = vec![];
        for item in value_vec {
            temp_value.push(RdbcValue::from(item));
        }
        RdbcFilterItem::Value(RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::In,
            value: Some(RdbcValue::Vec(temp_value)),
            ignore_null: true,
        })
    }
    pub fn in_v_slice<RC, RV>(column: RC, value_vec: &[RV]) -> RdbcFilterItem
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
            RV: Clone
    {
        let mut temp_value = vec![];
        for item in value_vec {
            temp_value.push(RdbcValue::from(item.clone()));
        }
        RdbcFilterItem::Value(RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::In,
            value: Some(RdbcValue::Vec(temp_value)),
            ignore_null: true,
        })
    }
}

pub struct RdbcValueFilterItem {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcValue>,
    ignore_null: bool,
}

impl RdbcValueFilterItem {
    pub fn eq_<T, V>(column: T, value: V) -> RdbcValueFilterItem
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::Eq,
            value: Some(RdbcValue::from(value)),
            ignore_null: false,
        }
    }
    pub fn ne_<T, V>(column: T, value: V) -> RdbcValueFilterItem
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::NotEq,
            value: Some(RdbcValue::from(value)),
            ignore_null: false,
        }
    }
}

impl RdbcValueFilterItem {
    pub fn get_column(&self) -> &RdbcColumn {
        &self.column_
    }
    pub fn get_compare(&self) -> &RdbcCompareType {
        &self.compare_
    }
    pub fn get_value(&self) -> Option<&RdbcValue> {
        self.value.as_ref()
    }
    pub fn get_ignore_null(&self) -> bool {
        self.ignore_null
    }
}

pub struct RdbcColumnFilterItem {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcColumn>,
}

impl RdbcColumnFilterItem {
    pub fn get_column(&self) -> &RdbcColumn {
        &self.column_
    }
    pub fn get_compare(&self) -> &RdbcCompareType {
        &self.compare_
    }
    pub fn get_value(&self) -> Option<&RdbcColumn> {
        self.value.as_ref()
    }
}

pub enum RdbcCompareType {
    Eq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Like,
    LikeLeft,
    LikeRight,
    NotLike,
    NotLikeLeft,
    NotLikeRight,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Exists,
    NotExists,
}

pub enum RdbcOrder {
    Column(RdbcColumnOrder),
}

impl RdbcOrder {
    pub fn asc_<RC>(column: RC) -> RdbcOrder
        where
            RdbcColumn: From<RC>,
    {
        RdbcOrder::Column(RdbcColumnOrder::asc_(column))
    }

    pub fn desc_<RC>(column: RC) -> RdbcOrder
        where
            RdbcColumn: From<RC>,
    {
        RdbcOrder::Column(RdbcColumnOrder::asc_(column))
    }
}

pub struct RdbcColumnOrder {
    column: RdbcColumn,
    order: RdbcOrderType,
}

impl RdbcColumnOrder {
    pub fn asc_<RC>(column: RC) -> RdbcColumnOrder
        where
            RdbcColumn: From<RC>,
    {
        RdbcColumnOrder {
            column: RdbcColumn::from(column),
            order: RdbcOrderType::Asc,
        }
    }
    pub fn desc_<RC>(column: RC) -> RdbcColumnOrder
        where
            RdbcColumn: From<RC>,
    {
        RdbcColumnOrder {
            column: RdbcColumn::from(column),
            order: RdbcOrderType::Desc,
        }
    }

    pub fn get_column(&self) -> &RdbcColumn {
        &self.column
    }
    pub fn get_order(&self) -> &RdbcOrderType {
        &self.order
    }
}

pub enum RdbcOrderType {
    Asc,
    Desc,
}

pub enum RdbcDmlValue {
    VALUE(RdbcValue),
    COLUMN(RdbcColumn),
}

impl RdbcDmlValue {
    pub fn is_value(&self) -> bool {
        match self {
            RdbcDmlValue::VALUE(value) => true,
            RdbcDmlValue::COLUMN(column) => false,
        }
    }
    pub fn is_column(&self) -> bool {
        match self {
            RdbcDmlValue::VALUE(value) => false,
            RdbcDmlValue::COLUMN(column) => true,
        }
    }
    pub fn get_value(&self) -> Option<&RdbcValue> {
        match self {
            RdbcDmlValue::VALUE(value) => Some(value),
            RdbcDmlValue::COLUMN(column) => None,
        }
    }
    pub fn get_column(&self) -> Option<&RdbcColumn> {
        match self {
            RdbcDmlValue::VALUE(value) => None,
            RdbcDmlValue::COLUMN(column) => Some(column),
        }
    }
}

impl From<RdbcColumn> for RdbcDmlValue {
    fn from(value: RdbcColumn) -> Self {
        RdbcDmlValue::COLUMN(value)
    }
}

impl<T> From<T> for RdbcDmlValue
    where
        RdbcValue: From<T>,
{
    fn from(value: T) -> Self {
        RdbcDmlValue::VALUE(RdbcValue::from(value))
    }
}

pub fn table<T>(table: T) -> RdbcTableInner
    where
        T: ToString,
{
    RdbcTableInner::table(table)
}

pub fn left_table<T>(table: T) -> RdbcTableInner
    where
        T: ToString,
{
    RdbcTableInner::left_join_table(table)
}

pub fn inner_table<T>(table: T) -> RdbcTableInner
    where
        T: ToString,
{
    RdbcTableInner::table(table)
}

pub fn right_table<T>(table: T) -> RdbcTableInner
    where
        T: ToString,
{
    RdbcTableInner::table(table)
}

pub fn full_table<T>(table: T) -> RdbcTableInner
    where
        T: ToString,
{
    RdbcTableInner::table(table)
}

pub fn table_column<T, V>(table: T, column: V) -> RdbcTableColumn
    where
        T: ToString,
        V: ToString,
{
    RdbcTableColumn::table_column(table, column)
}

pub fn simple_column<T, V>(table: T, column: V) -> RdbcColumn
    where
        T: ToString,
        V: ToString,
{
    RdbcColumn::table_column(table, column)
}

pub fn value_column<V>(column: V) -> RdbcColumn
    where
        V: ToString,
{
    RdbcColumn::rdbc_value(RdbcValue::String(column.to_string()))
}
