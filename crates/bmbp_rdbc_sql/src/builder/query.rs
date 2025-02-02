use crate::builder::part::{RdbcFilterBuilder, RdbcJoinTableBuilder, RdbcTableBuilder};
use crate::types::RdbcQuery;
use crate::{
    RdbcColumn, RdbcFuncColumn, RdbcGroupColumn,  RdbcJoinTable, RdbcOrderColumn,
    RdbcOrderType, RdbcQueryColumn, RdbcRawColumn, RdbcSelectColumn, RdbcSimpleColumn, RdbcTable,
    RdbcTableColumn, RdbcValueColumn, RdbcWhereFilter,
};
use bmbp_rdbc_type::{RdbcIdent, RdbcTableIdent, RdbcValue, RdbcValueIdent};


pub struct RdbcQueryBuilder {
    query: RdbcQuery,
}
pub struct RdbcHavingBuilder {
    filter: Option<RdbcWhereFilter>,
}
impl RdbcQueryBuilder {
    pub fn new() -> Self {
        RdbcQueryBuilder {
            query: RdbcQuery {
                select: vec![],
                table: vec![],
                join_table: vec![],
                where_: None,
                order_by: vec![],
                group_by: vec![],
                having: None,
                limit: None,
                offset: None,
                union: vec![],
                union_all: vec![],
            },
        }
    }
    pub fn from_table<T>() -> Self
    where
        T: RdbcTableIdent,
    {
        let mut builder = Self::new();
        builder.select_vec(T::columns());
        builder.table_as(T::table_name(), T::table_alias());
        builder
    }
    pub fn build(&self) -> RdbcQuery {
        self.query.clone()
    }
}
/// select 构建
impl RdbcQueryBuilder {
    pub fn select<T>(&mut self, column: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from(column.name()));
        self
    }
    pub fn select_as<C, A>(&mut self, column: C, alias: A) -> &mut Self
    where
        C: RdbcIdent,
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column.name(), alias.name())));
        self
    }
    pub fn select_vec<T>(&mut self, column_vec: Vec<T>) -> &mut Self
    where
        T: RdbcIdent,
    {
        for column in column_vec.iter() {
            self.query
                .select
                .push(RdbcSelectColumn::from(column.name()));
        }
        self
    }
    pub fn select_vec_as<C, A>(&mut self, column_vec: Vec<(C, A)>) -> &mut Self
    where
        C: RdbcIdent,
        A: RdbcIdent,
    {
        for (column, alias) in column_vec.iter() {
            self.query
                .select
                .push(RdbcSelectColumn::from((column.name(), alias.name())));
        }
        self
    }

    pub fn select_slice<T>(&mut self, column_vec: &[T]) -> &mut Self
    where
        T: RdbcIdent,
    {
        for column in column_vec.iter() {
            self.query
                .select
                .push(RdbcSelectColumn::from(column.name()));
        }
        self
    }
    pub fn select_slice_as<C, A>(&mut self, column_vec: &[(C, A)]) -> &mut Self
    where
        C: RdbcIdent,
        A: RdbcIdent,
    {
        for (column, alias) in column_vec.iter() {
            self.query
                .select
                .push(RdbcSelectColumn::from((column.name(), alias.name())));
        }
        self
    }

    pub fn select_query(&mut self, query: RdbcQuery) -> &mut Self {
        self.select_query_column(RdbcQueryColumn::from(query))
    }

    pub fn select_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_query_column_as(RdbcQueryColumn::from(query), alias)
    }
    pub fn select_raw<T>(&mut self, value: T) -> &mut Self
    where
        T: RdbcValueIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from(RdbcRawColumn::from(value.value())));
        self
    }

    pub fn select_raw_as<T, A>(&mut self, value: T, alias: A) -> &mut Self
    where
        T: RdbcValueIdent,
        A: RdbcIdent,
    {
        self.query.select.push(RdbcSelectColumn::from((
            RdbcRawColumn::from(value.value()),
            alias.name(),
        )));
        self
    }

    pub fn select_value<T>(&mut self, value: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.select_rdbc_value(RdbcValue::Varchar(value.name()));
        self
    }

    pub fn select_value_as<T, A>(&mut self, value: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.select_rdbc_value_as(RdbcValue::Varchar(value.name()), alias.name());
        self
    }
    pub fn select_rdbc_value(&mut self, value: RdbcValue) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(value));
        self
    }

    pub fn select_rdbc_value_as<A>(&mut self, value: RdbcValue, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((value, alias.name())));
        self
    }
    pub fn select_rdbc_column(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }

    pub fn select_rdbc_column_as<A>(&mut self, column: RdbcColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_simple_column(&mut self, column: RdbcSimpleColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_simple_column_as<A>(&mut self, column: RdbcSimpleColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column, alias.name())));
        self
    }
    pub fn select_table_column(&mut self, column: RdbcTableColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_table_column_as<A>(&mut self, column: RdbcTableColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column, alias.name())));
        self
    }
    pub fn select_query_column(&mut self, column: RdbcQueryColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_query_column_as<A>(&mut self, column: RdbcQueryColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column, alias.name())));
        self
    }

    pub fn select_value_column(&mut self, column: RdbcValueColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_value_column_as<A>(&mut self, column: RdbcValueColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column, alias.name())));
        self
    }

    pub fn select_func_column(&mut self, column: RdbcFuncColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_func_column_as<A>(&mut self, column: RdbcFuncColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column, alias.name())));
        self
    }

    pub fn select_raw_column(&mut self, column: RdbcRawColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(column));
        self
    }

    pub fn select_raw_column_as<A>(&mut self, column: RdbcRawColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.query
            .select
            .push(RdbcSelectColumn::from((column, alias.name())));
        self
    }

    pub fn select_select_column(&mut self, column: RdbcSelectColumn) -> &mut Self {
        self.query.select.push(column);
        self
    }

    pub fn select_func(&mut self, func: RdbcFuncColumn) -> &mut Self {
        self.query.select.push(RdbcSelectColumn::from(func));
        self
    }
}

impl RdbcQueryBuilder {
    pub fn order<C>(&mut self, column: C, order: RdbcOrderType) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        let rdbc_order_column = RdbcOrderColumn {
            column: RdbcColumn::from(column),
            order_type: order,
        };
        self.query.order_by.push(rdbc_order_column);
        self
    }
    pub fn order_asc<C>(&mut self, column: C) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        self.order(column, RdbcOrderType::Asc)
    }
    pub fn order_vec_asc<C>(&mut self, column: Vec<C>) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        for item in column {
            self.order(item, RdbcOrderType::Asc);
        }
        self
    }

    pub fn order_desc<C>(&mut self, column: C) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        self.order(column, RdbcOrderType::Desc)
    }
    pub fn order_vec_desc<C>(&mut self, column: Vec<C>) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        for item in column {
            self.order(item, RdbcOrderType::Desc);
        }
        self
    }
}

impl RdbcQueryBuilder {
    pub fn group_by<C>(&mut self, column: C) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        self.query
            .group_by
            .push(RdbcGroupColumn::from(RdbcColumn::from(column)));
        self
    }
    pub fn group_by_vec<C>(&mut self, column: Vec<C>) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        for item in column {
            self.group_by(item);
        }
        self
    }
}

impl RdbcQueryBuilder {
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.query.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.query.offset = Some(offset);
        self
    }
    pub fn having(&mut self, filter: RdbcWhereFilter) -> &mut Self {
        self.query.having = Some(filter);
        self
    }
    pub fn having_builder(&mut self, builder: RdbcHavingBuilder) -> &mut Self {
        self.query.having = builder.filter;
        self
    }
}

impl RdbcTableBuilder for RdbcQueryBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable> {
        self.query.table.as_mut()
    }
}
impl RdbcJoinTableBuilder for RdbcQueryBuilder {
    fn table_join_mut(&mut self) -> &mut Vec<RdbcJoinTable> {
        self.query.join_table.as_mut()
    }
}
impl RdbcFilterBuilder for RdbcQueryBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.query.where_.get_or_insert(RdbcWhereFilter::new())
    }
    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.query.where_.take()
    }
}
impl RdbcFilterBuilder for RdbcHavingBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.filter.get_or_insert(RdbcWhereFilter::new())
    }
    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.filter.take()
    }
}
