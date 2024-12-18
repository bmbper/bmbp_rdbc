use bmbp_rdbc_type::{RdbcIdent, RdbcValue};
use std::collections::HashMap;

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
impl<T> From<T> for RdbcColumn
where
    T: RdbcIdent,
{
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
    pub fn rdbc_value_alias<T>(value: RdbcValue, alias: T) -> RdbcValueColumn
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
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        match self {
            RdbcTableInner::Table(table) => table.get_filter(),
            RdbcTableInner::Query(query) => query.get_filter(),
        }
    }
    pub fn get_filter_mut(&mut self) -> &mut QueryFilter {
        match self {
            RdbcTableInner::Table(table) => table.get_filter_mut(),
            RdbcTableInner::Query(query) => query.get_filter_mut(),
        }
    }
    pub fn set_filter(&mut self, filter: QueryFilter) -> &mut Self {
        let _ = match self {
            RdbcTableInner::Table(table) => {
                let _ = table.set_filter(filter);
            }
            RdbcTableInner::Query(query) => {
                let _ = query.set_filter(filter);
            }
        };
        self
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

    pub fn temp_table(table: QueryWrapper) -> RdbcTableInner {
        RdbcTableInner::Query(RdbcQueryTable::query(table))
    }
    pub fn temp_table_alias<T>(table: QueryWrapper, alias: T) -> RdbcTableInner
    where
        T: ToString,
    {
        RdbcTableInner::Query(RdbcQueryTable::query_alias(table, alias))
    }
    pub fn join_table<T>(table: T, left: RdbcTableJoinType) -> RdbcTableInner
    where
        T: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::join_table(table, left))
    }
    pub fn join_table_alias<T, A>(table: T, alias: A, left: RdbcTableJoinType) -> RdbcTableInner
    where
        T: ToString,
        A: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::join_table_alias(table, alias, left))
    }
    pub fn join_schema_table<S, T>(schema: S, table: T, left: RdbcTableJoinType) -> RdbcTableInner
    where
        S: ToString,
        T: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::join_schema_table(schema, table, left))
    }
    pub fn join_schema_table_alias<S, T, A>(
        schema: S,
        table: T,
        alias: A,
        left: RdbcTableJoinType,
    ) -> RdbcTableInner
    where
        S: ToString,
        T: ToString,
        A: ToString,
    {
        RdbcTableInner::Table(RdbcSchemaTable::join_schema_table_alias(
            schema, table, alias, left,
        ))
    }
    pub(crate) fn join_temp_table(
        table: QueryWrapper,
        join_type: RdbcTableJoinType,
    ) -> RdbcTableInner {
        RdbcTableInner::Query(RdbcQueryTable::join_table(table, join_type))
    }
    pub(crate) fn join_temp_table_alias<SA>(
        table: QueryWrapper,
        alias: SA,
        join_type: RdbcTableJoinType,
    ) -> RdbcTableInner
    where
        SA: ToString,
    {
        RdbcTableInner::Query(RdbcQueryTable::join_table_alias(table, alias, join_type))
    }
}

impl RdbcTableInner {
    pub fn or(&mut self) -> &mut Self {
        match self {
            RdbcTableInner::Table(ref mut table) => {
                table.or();
            }
            RdbcTableInner::Query(ref mut table) => {
                table.or();
            }
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
    filter_: Option<QueryFilter>,
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
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter_.as_ref()
    }

    pub fn get_mut_params(&mut self) -> Option<&mut HashMap<String, RdbcValue>> {
        self.params_.as_mut()
    }
}

impl RdbcSchemaTable {
    pub fn table<T>(table: T) -> RdbcSchemaTable
    where
        T: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: None,
            filter_: Some(QueryFilter::new()),
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
            filter_: Some(QueryFilter::new()),
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
            filter_: Some(QueryFilter::new()),
            params_: None,
        }
    }
    pub fn schema_table_alias<SS, ST, SA>(schema: SS, table: ST, alias: SA) -> RdbcSchemaTable
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
            filter_: Some(QueryFilter::new()),
            params_: None,
        }
    }
    pub fn join_table<T>(table: T, join_type: RdbcTableJoinType) -> RdbcSchemaTable
    where
        T: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: None,
            join_: Some(join_type),
            filter_: Some(QueryFilter::new()),
            params_: None,
        }
    }
    pub fn join_table_alias<T, A>(
        table: T,
        alias: A,
        join_type: RdbcTableJoinType,
    ) -> RdbcSchemaTable
    where
        T: ToString,
        A: ToString,
    {
        RdbcSchemaTable {
            schema_: None,
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: Some(join_type),
            filter_: Some(QueryFilter::new()),
            params_: None,
        }
    }
    fn join_schema_table<S, T>(schmea: S, table: T, join_type: RdbcTableJoinType) -> RdbcSchemaTable
    where
        S: ToString,
        T: ToString,
    {
        RdbcSchemaTable {
            schema_: Some(schmea.to_string()),
            name_: table.to_string(),
            alias_: None,
            join_: Some(join_type),
            filter_: Some(QueryFilter::new()),
            params_: None,
        }
    }

    pub fn join_schema_table_alias<S, T, A>(
        schmea: S,
        table: T,
        alias: A,
        join_type: RdbcTableJoinType,
    ) -> RdbcSchemaTable
    where
        S: ToString,
        T: ToString,
        A: ToString,
    {
        RdbcSchemaTable {
            schema_: Some(schmea.to_string()),
            name_: table.to_string(),
            alias_: Some(alias.to_string()),
            join_: Some(join_type),
            filter_: Some(QueryFilter::new()),
            params_: None,
        }
    }
}

impl RdbcSchemaTable {
    fn set_filter(&mut self, filter: QueryFilter) -> &mut Self {
        self.filter_ = Some(filter);
        self
    }
    fn get_filter_mut(&mut self) -> &mut QueryFilter {
        if self.filter_.is_none() {
            self.filter_ = Some(QueryFilter::new());
        }
        self.filter_.as_mut().unwrap()
    }
    fn create_filter(&mut self, concat: RdbcConcatType) -> &mut Self {
        let filter = self.filter_.take().unwrap();
        let new_filter = QueryFilter::concat_with_filter(concat, filter);
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
        self.filter_
            .as_mut()
            .unwrap()
            .eq_(RdbcColumn::from(column), RdbcValue::from(value));
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
    filter_: Option<QueryFilter>,
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
    pub fn get_filter(&self) -> Option<&QueryFilter> {
        self.filter_.as_ref()
    }
    pub fn get_filter_mut(&mut self) -> &mut QueryFilter {
        if self.filter_.is_none() {
            self.filter_ = Some(QueryFilter::new());
        }
        self.filter_.as_mut().unwrap()
    }
    fn set_filter(&mut self, filter: QueryFilter) -> &mut Self {
        self.filter_ = Some(filter);
        self
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
    fn join_table(table: QueryWrapper, join_type: RdbcTableJoinType) -> RdbcQueryTable {
        RdbcQueryTable {
            name_: table,
            alias_: None,
            join_: Some(join_type),
            filter_: None,
        }
    }
    fn join_table_alias<A>(
        table: QueryWrapper,
        alias: A,
        join_type: RdbcTableJoinType,
    ) -> RdbcQueryTable
    where
        A: ToString,
    {
        RdbcQueryTable {
            name_: table,
            alias_: Some(alias.to_string()),
            join_: Some(join_type),
            filter_: None,
        }
    }
    // TODO
    fn or(&mut self) -> &mut Self {
        self
    }
}

pub enum RdbcConcatType {
    And,
    Or,
}

/// RdbcTableFilterImpl 表查询条件实现
pub struct QueryFilter {
    concat_: RdbcConcatType,
    item_: Vec<QueryFilterItem>,
    params_: Option<HashMap<String, RdbcValue>>,
}
impl QueryFilter {
    pub fn get_concat(&self) -> &RdbcConcatType {
        &self.concat_
    }
    pub fn get_item(&self) -> &Vec<QueryFilterItem> {
        &self.item_
    }
    pub fn get_params(&self) -> Option<&HashMap<String, RdbcValue>> {
        self.params_.as_ref()
    }

    pub fn add_filter(&mut self, filter: QueryFilter) -> &mut Self {
        self.item_.push(QueryFilterItem::Filter(filter));
        self
    }
    pub(crate) fn eq_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::eq_(column, value));
        self
    }
    pub fn eq_column<RC, VC>(&mut self, column: RC, value: VC) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<VC>,
    {
        self.item_.push(QueryFilterItem::eq_column(
            RdbcColumn::from(column),
            RdbcColumn::from(value),
        ));
        self
    }
    pub(crate) fn ne_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::ne_(column, value));
        self
    }
    pub(crate) fn ne_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::ne_column(column, value));
        self
    }
    pub(crate) fn ge_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::ge_(column, value));
        self
    }
    pub(crate) fn ge_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::ge_column(column, value));
        self
    }
    pub(crate) fn gt_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::gt_(column, value));
        self
    }
    pub(crate) fn gt_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::gt_column(column, value));
        self
    }
    pub(crate) fn le_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::le_(column, value));
        self
    }
    pub(crate) fn le_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::le_column(column, value));
        self
    }
    pub(crate) fn lt_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::lt_(column, value));
        self
    }
    pub(crate) fn lt_column(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::lt_column(column, value));
        self
    }
    pub(crate) fn null_(&mut self, column: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::null_(column));
        self
    }
    pub(crate) fn not_null_(&mut self, column: RdbcColumn) -> &mut Self {
        self.item_.push(QueryFilterItem::not_null_(column));
        self
    }
    pub fn between_(
        &mut self,
        column: RdbcColumn,
        value_start: RdbcValue,
        value_end: RdbcValue,
    ) -> &mut Self {
        self.item_
            .push(QueryFilterItem::between_(column, value_start, value_end));
        self
    }
    pub fn not_between_(
        &mut self,
        column: RdbcColumn,
        value_start: RdbcValue,
        value_end: RdbcValue,
    ) -> &mut Self {
        self.item_.push(QueryFilterItem::not_between_(
            column,
            value_start,
            value_end,
        ));
        self
    }
    pub fn like_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::like_(column, value));
        self
    }
    pub fn like_left_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::like_left_(column, value));
        self
    }
    pub fn like_right_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::like_right_(column, value));
        self
    }
    pub fn not_like_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_.push(QueryFilterItem::not_like_(column, value));
        self
    }
    pub fn not_like_left_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_
            .push(QueryFilterItem::not_like_left_(column, value));
        self
    }
    pub fn not_like_right_(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self.item_
            .push(QueryFilterItem::not_like_right_(column, value));
        self
    }
    pub fn in_v_<RC, RV>(&mut self, column: RC, value: Vec<RV>) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        let rc = RdbcColumn::from(column);
        let rv: Vec<RdbcValue> = value.into_iter().map(|v| RdbcValue::from(v)).collect();
        self.item_.push(QueryFilterItem::in_v(rc, rv));
        self
    }
    pub fn in_v_s_<RC, RV>(&mut self, column: RC, value: &[RV]) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: for<'a> From<&'a RV>,
    {
        let rc = RdbcColumn::from(column);
        let rv = value.into_iter().map(|v| RdbcValue::from(v)).collect();
        self.item_.push(QueryFilterItem::in_v(rc, rv));
        self
    }
    pub fn in_v(&mut self, column: RdbcColumn, value: Vec<RdbcValue>) -> &mut Self {
        self.item_.push(QueryFilterItem::in_v(column, value));
        self
    }

    pub fn in_v_slice(&mut self, column: RdbcColumn, value: &[RdbcValue]) -> &mut Self {
        let rv: Vec<RdbcValue> = value.into_iter().map(|v| v.clone()).collect();
        self.item_.push(QueryFilterItem::in_v(column, rv));
        self
    }
    pub fn in_query(&mut self, column: RdbcColumn, value: QueryWrapper) -> &mut Self {
        self.item_.push(QueryFilterItem::in_query(column, value));
        self
    }
    pub fn not_in_v_<RC, RV>(&mut self, column: RC, value: Vec<RV>) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        let rc = RdbcColumn::from(column);
        let rv: Vec<RdbcValue> = value.into_iter().map(|v| RdbcValue::from(v)).collect();
        self.item_.push(QueryFilterItem::not_in_v(rc, rv));
        self
    }
    pub fn not_in_v_s_<RC, RV>(&mut self, column: RC, value: &[RV]) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: for<'a> From<&'a RV>,
    {
        let rc = RdbcColumn::from(column);
        let rv: Vec<RdbcValue> = value.into_iter().map(|v| RdbcValue::from(v)).collect();
        self.item_.push(QueryFilterItem::not_in_v(rc, rv));
        self
    }
    pub fn not_in_v(&mut self, column: RdbcColumn, value: Vec<RdbcValue>) -> &mut Self {
        self.item_.push(QueryFilterItem::not_in_v(column, value));
        self
    }

    pub fn not_in_v_slice(&mut self, column: RdbcColumn, value: &[RdbcValue]) -> &mut Self {
        let rv: Vec<RdbcValue> = value.into_iter().map(|v| v.clone()).collect();
        self.item_.push(QueryFilterItem::not_in_v(column, rv));
        self
    }

    pub fn not_in_query(&mut self, column: RdbcColumn, value: QueryWrapper) -> &mut Self {
        self.item_
            .push(QueryFilterItem::not_in_query(column, value));
        self
    }
    pub fn exists_(&mut self, column: RdbcColumn, value: QueryWrapper) -> &mut Self {
        self.item_.push(QueryFilterItem::exists_(column, value));
        self
    }
    pub fn not_exists_(&mut self, column: RdbcColumn, value: QueryWrapper) -> &mut Self {
        self.item_.push(QueryFilterItem::not_exists_(column, value));
        self
    }
    pub fn new() -> QueryFilter {
        QueryFilter {
            concat_: RdbcConcatType::And,
            item_: vec![],
            params_: None,
        }
    }
    pub fn concat(concat: RdbcConcatType) -> QueryFilter {
        QueryFilter {
            concat_: concat,
            item_: vec![],
            params_: None,
        }
    }
    pub fn concat_with_filter(concat: RdbcConcatType, filter: QueryFilter) -> QueryFilter {
        QueryFilter {
            concat_: concat,
            item_: vec![QueryFilterItem::filter(filter)],
            params_: None,
        }
    }
}

pub enum QueryFilterItem {
    Value(RdbcValueFilterItem),
    Column(RdbcColumnFilterItem),
    Filter(QueryFilter),
    Query(RdbcQueryFilterItem),
}

impl QueryFilterItem {
    fn filter(filter: QueryFilter) -> QueryFilterItem {
        QueryFilterItem::Filter(filter)
    }
}

impl QueryFilterItem {
    pub(crate) fn eq_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::eq_(column, value))
    }
    pub fn eq_column(column: RdbcColumn, value: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::eq_column(column, value))
    }
    pub(crate) fn ne_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::ne_(column, value))
    }
    pub fn ne_column(column: RdbcColumn, value: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::ne_column(column, value))
    }
    pub(crate) fn ge_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::ge_(column, value))
    }
    pub fn ge_column(column: RdbcColumn, value: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::ge_column(column, value))
    }
    pub(crate) fn gt_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::gt_(column, value))
    }
    pub fn gt_column(column: RdbcColumn, value: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::gt_column(column, value))
    }
    pub(crate) fn le_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::le_(column, value))
    }
    pub fn le_column(column: RdbcColumn, value: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::le_column(column, value))
    }
    pub(crate) fn lt_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::lt_(column, value))
    }
    pub fn lt_column(column: RdbcColumn, value: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::lt_column(column, value))
    }

    pub fn null_(column: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::null_(column))
    }
    pub fn not_null_(column: RdbcColumn) -> QueryFilterItem {
        QueryFilterItem::Column(RdbcColumnFilterItem::not_null_(column))
    }
    pub fn like_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::like_(column, value))
    }
    pub fn like_left_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::like_left_(column, value))
    }
    pub fn like_right_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::like_right_(column, value))
    }

    pub fn not_like_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::not_like_(column, value))
    }
    pub fn not_like_left_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::not_like_left_(column, value))
    }
    pub fn not_like_right_(column: RdbcColumn, value: RdbcValue) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::not_like_right_(column, value))
    }
    pub(crate) fn between_(
        column: RdbcColumn,
        value_start: RdbcValue,
        value_end: RdbcValue,
    ) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::between_(
            column,
            value_start,
            value_end,
        ))
    }
    pub(crate) fn not_between_(
        column: RdbcColumn,
        value_start: RdbcValue,
        value_end: RdbcValue,
    ) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::not_between_(
            column,
            value_start,
            value_end,
        ))
    }
    pub fn in_v(column: RdbcColumn, value_vec: Vec<RdbcValue>) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::in_v(column, value_vec))
    }
    pub fn in_query(column: RdbcColumn, query_wrapper: QueryWrapper) -> QueryFilterItem {
        QueryFilterItem::Query(RdbcQueryFilterItem::in_query(column, query_wrapper))
    }
    pub fn not_in_v(column: RdbcColumn, value_vec: Vec<RdbcValue>) -> QueryFilterItem {
        QueryFilterItem::Value(RdbcValueFilterItem::not_in_v(column, value_vec))
    }
    pub fn not_in_query(column: RdbcColumn, value: QueryWrapper) -> QueryFilterItem {
        QueryFilterItem::Query(RdbcQueryFilterItem::not_in_query(column, value))
    }
    pub fn exists_(column: RdbcColumn, value: QueryWrapper) -> QueryFilterItem {
        QueryFilterItem::Query(RdbcQueryFilterItem::exists_query(column, value))
    }
    pub fn not_exists_(column: RdbcColumn, value: QueryWrapper) -> QueryFilterItem {
        QueryFilterItem::Query(RdbcQueryFilterItem::not_exists_query(column, value))
    }
}

pub struct RdbcValueFilterItem {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<RdbcValue>,
    ignore_null: bool,
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
impl RdbcValueFilterItem {
    pub fn eq_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::Eq,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn ne_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::NotEq,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn ge_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::GtEq,
            value: Some(value),
            ignore_null: false,
        }
    }

    pub fn gt_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::Gt,
            value: Some(value),
            ignore_null: false,
        }
    }

    pub fn le_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::LtEq,
            value: Some(value),
            ignore_null: false,
        }
    }

    pub fn lt_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::Lt,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn like_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::Like,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn like_left_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::LikeLeft,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn like_right_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::LikeRight,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn not_like_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::NotLike,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn not_like_left_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::NotLikeLeft,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn not_like_right_(column: RdbcColumn, value: RdbcValue) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::NotLikeRight,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn between_(
        column: RdbcColumn,
        value_start: RdbcValue,
        value_end: RdbcValue,
    ) -> RdbcValueFilterItem {
        let value = RdbcValue::Vec(vec![value_start, value_end]);
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::Between,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn not_between_(
        column: RdbcColumn,
        value_start: RdbcValue,
        value_end: RdbcValue,
    ) -> RdbcValueFilterItem {
        let value = RdbcValue::Vec(vec![value_start, value_end]);
        RdbcValueFilterItem {
            column_: column,
            compare_: RdbcCompareType::NotBetween,
            value: Some(value),
            ignore_null: false,
        }
    }
    pub fn in_v(column: RdbcColumn, value_vec: Vec<RdbcValue>) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::In,
            value: Some(RdbcValue::Vec(value_vec)),
            ignore_null: true,
        }
    }
    pub fn not_in_v(column: RdbcColumn, value_vec: Vec<RdbcValue>) -> RdbcValueFilterItem {
        RdbcValueFilterItem {
            column_: RdbcColumn::from(column),
            compare_: RdbcCompareType::NotIn,
            value: Some(RdbcValue::Vec(value_vec)),
            ignore_null: true,
        }
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
impl RdbcColumnFilterItem {
    pub fn eq_column(column: RdbcColumn, value: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::Eq,
            value: Some(value),
        }
    }
    pub fn ne_column(column: RdbcColumn, value: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::NotEq,
            value: Some(value),
        }
    }
    pub fn ge_column(column: RdbcColumn, value: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::GtEq,
            value: Some(value),
        }
    }
    pub fn gt_column(column: RdbcColumn, value: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::Gt,
            value: Some(value),
        }
    }
    pub fn le_column(column: RdbcColumn, value: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::LtEq,
            value: Some(value),
        }
    }
    pub fn lt_column(column: RdbcColumn, value: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::Lt,
            value: Some(value),
        }
    }
    pub fn null_(column: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::IsNull,
            value: None,
        }
    }
    pub fn not_null_(column: RdbcColumn) -> RdbcColumnFilterItem {
        RdbcColumnFilterItem {
            column_: column,
            compare_: RdbcCompareType::IsNotNull,
            value: None,
        }
    }
}

pub struct RdbcQueryFilterItem {
    column_: RdbcColumn,
    compare_: RdbcCompareType,
    value: Option<QueryWrapper>,
}
impl RdbcQueryFilterItem {
    pub fn get_column(&self) -> &RdbcColumn {
        &self.column_
    }
    pub fn get_compare(&self) -> &RdbcCompareType {
        &self.compare_
    }
    pub fn get_value(&self) -> Option<&QueryWrapper> {
        self.value.as_ref()
    }
}
impl RdbcQueryFilterItem {
    fn in_query(rdbc_column: RdbcColumn, query_wrapper: QueryWrapper) -> RdbcQueryFilterItem {
        RdbcQueryFilterItem {
            column_: rdbc_column,
            compare_: RdbcCompareType::In,
            value: Some(query_wrapper),
        }
    }
    fn not_in_query(rdbc_column: RdbcColumn, query_wrapper: QueryWrapper) -> RdbcQueryFilterItem {
        RdbcQueryFilterItem {
            column_: rdbc_column,
            compare_: RdbcCompareType::NotIn,
            value: Some(query_wrapper),
        }
    }
    fn exists_query(rdbc_column: RdbcColumn, query_wrapper: QueryWrapper) -> RdbcQueryFilterItem {
        RdbcQueryFilterItem {
            column_: rdbc_column,
            compare_: RdbcCompareType::Exists,
            value: Some(query_wrapper),
        }
    }
    fn not_exists_query(
        rdbc_column: RdbcColumn,
        query_wrapper: QueryWrapper,
    ) -> RdbcQueryFilterItem {
        RdbcQueryFilterItem {
            column_: rdbc_column,
            compare_: RdbcCompareType::NotExists,
            value: Some(query_wrapper),
        }
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
    Between,
    NotBetween,
}
impl RdbcCompareType {
    pub fn name(&self) -> String {
        match self {
            RdbcCompareType::Eq => "=".to_string(),
            RdbcCompareType::NotEq => "<>".to_string(),
            RdbcCompareType::Gt => ">".to_string(),
            RdbcCompareType::GtEq => ">=".to_string(),
            RdbcCompareType::Lt => "<".to_string(),
            RdbcCompareType::LtEq => "<=".to_string(),
            RdbcCompareType::Like => "LIKE".to_string(),
            RdbcCompareType::LikeLeft => "LIKE".to_string(),
            RdbcCompareType::LikeRight => "LIKE".to_string(),
            RdbcCompareType::NotLike => "NOT LIKE".to_string(),
            RdbcCompareType::NotLikeLeft => "NOT LIKE".to_string(),
            RdbcCompareType::NotLikeRight => "NOT LIKE".to_string(),
            RdbcCompareType::In => "IN".to_string(),
            RdbcCompareType::NotIn => "NOT IN".to_string(),
            RdbcCompareType::IsNull => "IS NULL".to_string(),
            RdbcCompareType::IsNotNull => "IS NOT NULL".to_string(),
            RdbcCompareType::Exists => "EXISTS".to_string(),
            RdbcCompareType::NotExists => "NOT EXISTS".to_string(),
            RdbcCompareType::Between => "BETWEEN".to_string(),
            RdbcCompareType::NotBetween => "NOT BETWEEN".to_string(),
        }
    }
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
            RdbcDmlValue::VALUE(_) => true,
            RdbcDmlValue::COLUMN(_) => false,
        }
    }
    pub fn is_column(&self) -> bool {
        match self {
            RdbcDmlValue::VALUE(_) => false,
            RdbcDmlValue::COLUMN(_) => true,
        }
    }
    pub fn get_value(&self) -> Option<&RdbcValue> {
        match self {
            RdbcDmlValue::VALUE(value) => Some(value),
            RdbcDmlValue::COLUMN(_) => None,
        }
    }
    pub fn get_column(&self) -> Option<&RdbcColumn> {
        match self {
            RdbcDmlValue::VALUE(_) => None,
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
    RdbcTableInner::join_table(table, RdbcTableJoinType::Left)
}

pub fn inner_table<T>(table: T) -> RdbcTableInner
where
    T: ToString,
{
    RdbcTableInner::join_table(table, RdbcTableJoinType::Inner)
}

pub fn right_table<T>(table: T) -> RdbcTableInner
where
    T: ToString,
{
    RdbcTableInner::join_table(table, RdbcTableJoinType::Right)
}

pub fn full_table<T>(table: T) -> RdbcTableInner
where
    T: ToString,
{
    RdbcTableInner::join_table(table, RdbcTableJoinType::Full)
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
