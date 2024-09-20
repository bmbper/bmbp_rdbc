use bmbp_rdbc_type::{RdbcIdent, RdbcValue};

use crate::{QueryFilter, QueryWrapper, RdbcColumn, RdbcConcatType, RdbcTableInner};

use super::RdbcTableJoinType;

/// RdbcTableFilter query filter trait
pub trait RdbcTableFilter {
    fn init_filter(&mut self) -> &mut Self;
    fn get_filter_mut(&mut self) -> &mut QueryFilter;
    fn with_filter(&mut self, concat_type: RdbcConcatType) -> &mut Self;
    fn add_filter(&mut self, filter: QueryFilter) -> &mut Self {
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
    fn eq_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .eq_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn eq_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<RV>,
    {
        self.get_filter_mut()
            .eq_column(RdbcColumn::from(column), RdbcColumn::from(value));
        self
    }

    fn ne_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .ne_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn ne_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<RV>,
    {
        self.get_filter_mut()
            .ne_column(RdbcColumn::from(column), RdbcColumn::from(value));
        self
    }

    fn ge_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .ge_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn ge_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<RV>,
    {
        self.get_filter_mut()
            .ge_column(RdbcColumn::from(column), RdbcColumn::from(value));
        self
    }
    fn gt_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .gt_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn gt_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<RV>,
    {
        self.get_filter_mut()
            .gt_column(RdbcColumn::from(column), RdbcColumn::from(value));
        self
    }
    fn le_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .le_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn le_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<RV>,
    {
        self.get_filter_mut()
            .le_column(RdbcColumn::from(column), RdbcColumn::from(value));
        self
    }

    fn lt_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .lt_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn lt_col<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcColumn: From<RV>,
    {
        self.get_filter_mut()
            .lt_column(RdbcColumn::from(column), RdbcColumn::from(value));
        self
    }
    fn like_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .like_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn like_left_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .like_left_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn like_right_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .like_right_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn not_like_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .not_like_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn not_like_left_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .not_like_left_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn not_like_right_<RC, RV>(&mut self, column: RC, value: RV) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        self.get_filter_mut()
            .not_like_right_(RdbcColumn::from(column), RdbcValue::from(value));
        self
    }
    fn null_<RC>(&mut self, column: RC) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.get_filter_mut().null_(RdbcColumn::from(column));
        self
    }
    fn not_null_<RC>(&mut self, column: RC) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.get_filter_mut().not_null_(RdbcColumn::from(column));
        self
    }
    fn between_<RC, RVS, RVE>(&mut self, column: RC, value_start: RVS, value_end: RVE) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RVS>,
        RdbcValue: From<RVE>,
    {
        self.get_filter_mut().between_(
            RdbcColumn::from(column),
            RdbcValue::from(value_start),
            RdbcValue::from(value_end),
        );
        self
    }
    fn not_between_<RC, RVS, RVE>(
        &mut self,
        column: RC,
        value_start: RVS,
        value_end: RVE,
    ) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RVS>,
        RdbcValue: From<RVE>,
    {
        self.get_filter_mut().not_between_(
            RdbcColumn::from(column),
            RdbcValue::from(value_start),
            RdbcValue::from(value_end),
        );
        self
    }
    fn in_v<RC, RV>(&mut self, column: RC, value: Vec<RV>) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        let v_vec = value.into_iter().map(|v| RdbcValue::from(v)).collect();
        self.get_filter_mut().in_v(RdbcColumn::from(column), v_vec);
        self
    }
    fn in_v_slice<RC, RV>(&mut self, column: RC, value: &[RV]) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
        RV: Clone,
    {
        let v_vec = value
            .into_iter()
            .map(|v| RdbcValue::from(v.clone()))
            .collect();
        self.get_filter_mut().in_v(RdbcColumn::from(column), v_vec);
        self
    }
    fn in_query<RC>(&mut self, column: RC, value: QueryWrapper) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.get_filter_mut()
            .in_query(RdbcColumn::from(column), value);
        self
    }
    fn not_in_v<RC, RV>(&mut self, column: RC, value: Vec<RV>) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
    {
        let v_vec = value.into_iter().map(|v| RdbcValue::from(v)).collect();
        self.get_filter_mut()
            .not_in_v(RdbcColumn::from(column), v_vec);
        self
    }
    fn not_in_v_slice<RC, RV>(&mut self, column: RC, value: &[RV]) -> &mut Self
    where
        RdbcColumn: From<RC>,
        RdbcValue: From<RV>,
        RV: Clone,
    {
        let v_vec = value
            .into_iter()
            .map(|v| RdbcValue::from(v.clone()))
            .collect();
        self.get_filter_mut()
            .not_in_v(RdbcColumn::from(column), v_vec);
        self
    }

    fn not_in_query<RC>(&mut self, column: RC, value: QueryWrapper) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.get_filter_mut()
            .not_in_query(RdbcColumn::from(column), value);
        self
    }

    fn exists_<RC>(&mut self, column: RC, value: QueryWrapper) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.get_filter_mut()
            .exists_(RdbcColumn::from(column), value);
        self
    }
    fn not_exists_<RC>(&mut self, column: RC, value: QueryWrapper) -> &mut Self
    where
        RdbcColumn: From<RC>,
    {
        self.get_filter_mut()
            .not_exists_(RdbcColumn::from(column), value);
        self
    }
}

pub trait RdbcTableWrapper {
    fn get_table_mut(&mut self) -> &mut Vec<RdbcTableInner>;
    fn get_join_mut(&mut self) -> &mut Vec<RdbcTableInner>;
    fn table<ST>(&mut self, table: ST) -> &mut Self
    where
        ST: RdbcIdent,
    {
        self.get_table_mut()
            .push(RdbcTableInner::table(table.get_ident()));
        self
    }
    fn table_alias<ST, SA>(&mut self, table: ST, alias: SA) -> &mut Self
    where
        ST: RdbcIdent,
        SA: RdbcIdent,
    {
        self.get_table_mut().push(RdbcTableInner::table_alias(
            table.get_ident(),
            alias.get_ident(),
        ));
        self
    }
    fn schema_table<TS, TT>(&mut self, schema: TS, table: TT) -> &mut Self
    where
        TS: RdbcIdent,
        TT: RdbcIdent,
    {
        self.get_table_mut().push(RdbcTableInner::schema_table(
            schema.get_ident(),
            table.get_ident(),
        ));
        self
    }
    fn schema_table_alias<SS, ST, SA>(&mut self, schema: SS, table: ST, alias: SA) -> &mut Self
    where
        SS: RdbcIdent,
        ST: RdbcIdent,
        SA: RdbcIdent,
    {
        self.get_table_mut()
            .push(RdbcTableInner::schema_table_alias(
                schema.get_ident(),
                table.get_ident(),
                alias.get_ident(),
            ));
        self
    }
    fn temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_table_mut().push(RdbcTableInner::temp_table(table));
        self
    }
    fn temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_table_mut()
            .push(RdbcTableInner::temp_table_alias(table, alias.get_ident()));
        self
    }
    fn rdbc_table<T>(&mut self, table: T) -> &mut Self
    where
        RdbcTableInner: From<T>,
    {
        self.get_table_mut().push(RdbcTableInner::from(table));
        self
    }

    fn join_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::table(table.get_ident()));
        self
    }
    fn join_table_alias<TS, TA>(&mut self, table: TS, alias: TA) -> &mut Self
    where
        TS: RdbcIdent,
        TA: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::table_alias(
            table.get_ident(),
            alias.get_ident(),
        ));
        self
    }
    fn join_schema_table<SS, ST>(&mut self, schema: SS, table: ST) -> &mut Self
    where
        SS: RdbcIdent,
        ST: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::schema_table(
            schema.get_ident(),
            table.get_ident(),
        ));
        self
    }
    fn join_schema_table_alias<SS, ST, SA>(&mut self, schema: SS, table: ST, alias: SA) -> &mut Self
    where
        SS: RdbcIdent,
        ST: RdbcIdent,
        SA: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::schema_table_alias(
            schema.get_ident(),
            table.get_ident(),
            alias.get_ident(),
        ));
        self
    }

    fn join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_join_mut().push(RdbcTableInner::temp_table(table));
        self
    }
    fn join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::temp_table_alias(table, alias.get_ident()));
        self
    }
    fn join_rdbc_table(&mut self, table: RdbcTableInner) -> &mut Self {
        self.get_join_mut().push(table);
        self
    }
    fn left_join_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table(
            table.get_ident(),
            RdbcTableJoinType::Left,
        ));
        self
    }
    fn left_join_table_alias<T, A>(&mut self, table: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table_alias(
            table.get_ident(),
            alias.get_ident(),
            RdbcTableJoinType::Left,
        ));
        self
    }
    fn left_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_schema_table(
            schema.get_ident(),
            table.get_ident(),
            RdbcTableJoinType::Left,
        ));
        self
    }

    fn left_join_schema_table_alias<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_schema_table_alias(
                schema.get_ident(),
                table.get_ident(),
                alias.get_ident(),
                RdbcTableJoinType::Left,
            ));
        self
    }
    fn left_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_join_mut().push(RdbcTableInner::join_temp_table(
            table,
            RdbcTableJoinType::Left,
        ));
        self
    }
    fn left_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_temp_table_alias(
                table,
                alias.get_ident(),
                RdbcTableJoinType::Left,
            ));
        self
    }

    fn right_join_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table(
            table.get_ident(),
            RdbcTableJoinType::Right,
        ));
        self
    }
    fn right_join_table_alias<T, A>(&mut self, table: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table_alias(
            table.get_ident(),
            alias.get_ident(),
            RdbcTableJoinType::Right,
        ));
        self
    }
    fn right_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_schema_table(
            schema.get_ident(),
            table.get_ident(),
            RdbcTableJoinType::Right,
        ));
        self
    }

    fn right_join_schema_table_alias<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_schema_table_alias(
                schema.get_ident(),
                table.get_ident(),
                alias.get_ident(),
                RdbcTableJoinType::Right,
            ));
        self
    }
    fn right_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_join_mut().push(RdbcTableInner::join_temp_table(
            table,
            RdbcTableJoinType::Right,
        ));
        self
    }
    fn right_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_temp_table_alias(
                table,
                alias.get_ident(),
                RdbcTableJoinType::Right,
            ));
        self
    }

    fn full_join_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table(
            table.get_ident(),
            RdbcTableJoinType::Full,
        ));
        self
    }
    fn full_join_table_alias<T, A>(&mut self, table: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table_alias(
            table.get_ident(),
            alias.get_ident(),
            RdbcTableJoinType::Full,
        ));
        self
    }
    fn full_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_schema_table(
            schema.get_ident(),
            table.get_ident(),
            RdbcTableJoinType::Full,
        ));
        self
    }

    fn full_join_schema_table_alias<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_schema_table_alias(
                schema.get_ident(),
                table.get_ident(),
                alias.get_ident(),
                RdbcTableJoinType::Full,
            ));
        self
    }
    fn full_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_join_mut().push(RdbcTableInner::join_temp_table(
            table,
            RdbcTableJoinType::Full,
        ));
        self
    }
    fn full_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_temp_table_alias(
                table,
                alias.get_ident(),
                RdbcTableJoinType::Full,
            ));
        self
    }

    fn inner_join_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table(
            table.get_ident(),
            RdbcTableJoinType::Inner,
        ));
        self
    }
    fn inner_join_table_alias<T, A>(&mut self, table: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_table_alias(
            table.get_ident(),
            alias.get_ident(),
            RdbcTableJoinType::Inner,
        ));
        self
    }
    fn inner_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.get_join_mut().push(RdbcTableInner::join_schema_table(
            schema.get_ident(),
            table.get_ident(),
            RdbcTableJoinType::Inner,
        ));
        self
    }

    fn inner_join_schema_table_alias<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_schema_table_alias(
                schema.get_ident(),
                table.get_ident(),
                alias.get_ident(),
                RdbcTableJoinType::Inner,
            ));
        self
    }
    fn inner_join_temp_table(&mut self, table: QueryWrapper) -> &mut Self {
        self.get_join_mut().push(RdbcTableInner::join_temp_table(
            table,
            RdbcTableJoinType::Inner,
        ));
        self
    }
    fn inner_join_temp_table_alias<T>(&mut self, table: QueryWrapper, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.get_join_mut()
            .push(RdbcTableInner::join_temp_table_alias(
                table,
                alias.get_ident(),
                RdbcTableJoinType::Inner,
            ));
        self
    }

    fn on(&mut self) -> &mut QueryFilter {
        let join_vec = self.get_join_mut();
        let len = join_vec.len();
        let join_table = join_vec.get_mut(len - 1).unwrap();
        join_table.get_filter_mut()
    }
}
