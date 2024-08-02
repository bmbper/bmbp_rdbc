use std::collections::HashMap;

use crate::build::{mysql_build_sql, pg_build_sql};
use crate::{
    RdbcDataBase, QueryWrapper, RdbcColumn, RdbcConcatType, RdbcDmlValue, RdbcFilterInner,
    RdbcTableInner, RdbcValue,
};

pub trait RdbcFilterWrapper {
    fn init_filter(&mut self) -> &mut Self;
    fn get_filter_mut(&mut self) -> &mut RdbcFilterInner;
    fn with_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self;
    fn add_filter(&mut self, filter: RdbcFilterInner) -> &mut Self {
        self.get_filter_mut().add_filter(filter);
        self
    }

    fn and(&mut self) -> &mut Self {
        self.with_filter(RdbcConcatType::And);
        self
    }
    fn or(&mut self) -> &mut Self {
        self.with_filter(RdbcConcatType::And);
        self
    }
    fn eq_<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self.get_filter_mut().eq_(column, value);
        self
    }
    fn eq_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn eq_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn eq_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_eq<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_eq_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_eq_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_eq_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_eq_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn ne_<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self.get_filter_mut().ne_(column, value);
        self
    }
    fn ne_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn ne_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn ne_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_ne<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_ne_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_ne_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_ne_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_ne_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn ge<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn ge_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn ge_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn ge_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_ge<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_ge_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_ge_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_ge_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_ge_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn gt<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn gt_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn gt_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn gt_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_gt<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_gt_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_gt_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_gt_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_gt_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn le<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn le_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn le_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn le_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_le<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_le_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_le_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_le_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_le_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn lt<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn lt_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn lt_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn lt_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_lt<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_lt_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_lt_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_lt_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_lt_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn like<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self.get_filter_mut().like_value(column, value);
        self
    }
    fn like_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn like_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn like_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_like<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_like_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_like_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_like_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_like_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn like_left<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        self
    }
    fn like_left_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcColumn: From<RV>,
    {
        self.get_filter_mut().like_left_col(column, value);
        self
    }
    fn like_left_value<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
        where
            RdbcColumn: From<RC>,
            RdbcValue: From<RV>,
    {
        self.get_filter_mut().like_left_value(column, value);
        self
    }
    fn like_left_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_like_left<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_like_left_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_like_left_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_like_left_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_like_left_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn like_right<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn like_right_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn like_right_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn like_right_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_like_right<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_like_right_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_like_right_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_like_right_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_like_right_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn not_like<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn not_like_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn not_like_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn not_like_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_not_like<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_not_like_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_not_like_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_not_like_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_not_like_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn not_like_left<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcDmlValue: From<V>,
    {
        self
    }
    fn not_like_left_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn not_like_left_value<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self.get_filter_mut().not_like_left_value(column, value);
        self
    }
    fn not_like_left_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_not_like_left<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_not_like_left_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_not_like_left_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_not_like_left_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_not_like_left_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }
    fn not_like_right<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn not_like_right_col<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcColumn: From<V>,
    {
        self
    }
    fn not_like_right_value<T, V>(&mut self, column: T, value: RdbcValue) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn not_like_right_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_not_like_right<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_not_like_right_col<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            RdbcColumn: From<V>,
    {
        self
    }
    fn col_not_like_right_value(&mut self, column: RdbcColumn, value: RdbcValue) -> &mut Self {
        self
    }
    fn col_not_like_right_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn col_not_like_right_rdbc_col(&mut self, column: RdbcColumn, value: RdbcColumn) -> &mut Self {
        self
    }

    fn in_v<T, V>(&mut self, column: T, value: Vec<V>) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn in_v_slice<T, V>(&mut self, column: T, value: &[V]) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn in_query<T, V>(&mut self, column: T, value: QueryWrapper) -> &mut Self
        where
            RdbcColumn: From<T>,
    {
        self
    }
    fn in_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_in_v<V>(&mut self, column: RdbcColumn, value: Vec<V>) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_in_slice<V>(&mut self, column: RdbcColumn, value: &[V]) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_in_query(&mut self, column: RdbcColumn, value: QueryWrapper) -> &mut Self {
        self
    }
    fn col_in_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }

    fn not_in_v<T, V>(&mut self, column: T, value: Vec<V>) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn not_in_v_slice<T, V>(&mut self, column: T, value: &[V]) -> &mut Self
        where
            RdbcColumn: From<T>,
            RdbcValue: From<V>,
    {
        self
    }
    fn not_in_query<T, V>(&mut self, column: T, value: QueryWrapper) -> &mut Self
        where
            RdbcColumn: From<T>,
    {
        self
    }
    fn not_in_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            V: ToString,
    {
        self
    }
    fn col_not_in_v<V>(&mut self, column: RdbcColumn, value: Vec<V>) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_not_in_slice<V>(&mut self, column: RdbcColumn, value: &[V]) -> &mut Self
        where
            RdbcValue: From<V>,
    {
        self
    }
    fn col_not_in_query(&mut self, column: RdbcColumn, value: QueryWrapper) -> &mut Self {
        self
    }
    fn col_not_in_raw<V>(&mut self, column: RdbcColumn, value: V) -> &mut Self
        where
            V: ToString,
    {
        self
    }
    fn is_null<T>(&mut self, column: T) -> &mut Self
        where
            RdbcColumn: From<T>,
    {
        self
    }
    fn col_is_null(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }
    fn not_null<T>(&mut self, column: T) -> &mut Self
        where
            RdbcColumn: From<T>,
    {
        self
    }
    fn col_not_null(&mut self, column: RdbcColumn) -> &mut Self {
        self
    }

    fn exists_<T>(&mut self, column: T, value: QueryWrapper) -> &mut Self
        where
            RdbcColumn: From<T>,
    {
        self
    }
    fn exists_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            T: ToString,
    {
        self
    }
    fn not_exists_<T>(&mut self, column: T, value: QueryWrapper) -> &mut Self
        where
            RdbcColumn: From<T>,
    {
        self
    }
    fn not_exists_raw<T, V>(&mut self, column: T, value: V) -> &mut Self
        where
            RdbcColumn: From<T>,
            T: ToString,
    {
        self
    }
}

pub trait RdbcTableWrapper {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner>;
    fn get_join_mut(&mut self) -> &mut Vec<RdbcTableInner>;
    fn table<ST>(&mut self, table: ST) -> &mut Self
        where
            ST: ToString,
    {
        self.get_table_mut().push(RdbcTableInner::table(table));
        self
    }
    fn table_alias<ST, SA>(&mut self, table: ST, alias: SA) -> &mut Self
        where
            ST: ToString,
            SA: ToString,
    {
        self.get_table_mut()
            .push(RdbcTableInner::table_alias(table, alias));
        self
    }
    fn schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
        where
            T: ToString,
    {
        self.get_table_mut()
            .push(RdbcTableInner::schema_table(schema, table));
        self
    }
    fn schema_table_alias<SS, ST, SA>(&mut self, schema: SS, table: ST, alias: SA) -> &mut Self
        where
            SS: ToString,
            ST: ToString,
            SA: ToString,
    {
        self.get_table_mut()
            .push(RdbcTableInner::schema_table_alias(schema, table, alias));
        self
    }
    fn temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_table_mut().push(RdbcTableInner::temp_table(table));
        self
    }
    fn temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self.get_table_mut()
            .push(RdbcTableInner::temp_table_alias(table, alias));
        self
    }
    fn rdbc_table<T>(&mut self, table: T) -> &mut Self
        where
            RdbcTableInner: From<T>,
    {
        self.get_table_mut().push(RdbcTableInner::from(table));
        self
    }

    fn on(&mut self) -> Option<&mut RdbcTableInner> {
        None
    }
    fn on_index(&mut self, index: usize) -> Option<&mut RdbcTableInner> {
        None
    }
    fn join_table<T>(&mut self, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self
    }
    fn join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn join_rdbc_table(&mut self, table: RdbcTableInner) -> &mut Self {
        self.get_join_mut().push(table);
        self
    }
    fn left_join_table<T>(&mut self, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn left_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn left_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }

    fn left_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn left_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self
    }
    fn left_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn left_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self
        where
            T: Into<RdbcTableInner>,
    {
        self
    }

    fn right_join_table<T>(&mut self, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn right_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn right_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn right_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn right_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self
    }
    fn right_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn right_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self
        where
            T: Into<RdbcTableInner>,
    {
        self
    }

    fn full_join_table<T>(&mut self, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn full_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn full_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn full_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn full_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self
    }
    fn full_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn full_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self
        where
            T: Into<RdbcTableInner>,
    {
        self
    }
    fn inner_join_table<T>(&mut self, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn inner_join_table_alias<T>(&mut self, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn inner_join_schema_table<T>(&mut self, schema: T, table: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn inner_join_schema_table_alias<T>(&mut self, schema: T, table: T, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn inner_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self
    }
    fn inner_join_temp_table_as_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
        where
            T: ToString,
    {
        self
    }
    fn inner_join_rdbc_table<T>(&mut self, mut table: T) -> &mut Self
        where
            T: Into<RdbcTableInner>,
    {
        self
    }
}

/// RdbcSQLParser 语句解析器
pub trait RdbcSQL {
    fn build_sql(&self, database_type: RdbcDataBase) -> (String, Vec<RdbcValue>) {
        let (sql, params) = self.build_script(database_type.clone());
        match database_type {
            RdbcDataBase::Postgres => pg_build_sql(sql, params),
            RdbcDataBase::MySQL => mysql_build_sql(sql, params),
        }
    }
    fn build_script(&self, database_type: RdbcDataBase) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    /// build_raw_sql build the value in sql
    /// like :
    /// ```sql
    /// select name from demo where name = '1'
    /// ```
    fn build_raw_sql(&self, database_type: RdbcDataBase) -> String {
        "".to_string()
    }
}
