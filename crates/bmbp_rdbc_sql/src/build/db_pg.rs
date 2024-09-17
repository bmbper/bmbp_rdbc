use crate::build::types::RdbcSQLBuilder;
use crate::{
    DeleteWrapper, InsertWrapper, QueryWrapper, RdbcColumn, RdbcFunc, RdbcFuncColumn, RdbcOrder,
    RdbcOrderType, RdbcQueryColumn, RdbcQueryTable, RdbcSchemaTable, RdbcTableColumn,
    RdbcTableFilterImpl, RdbcTableInner, RdbcTableJoinType, RdbcValueColumn, UpdateWrapper,
};
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;
use std::vec;

pub struct PgRdbcSQLBuilder;

impl RdbcSQLBuilder for PgRdbcSQLBuilder {
    fn build_query_script(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        PgScriptBuilder::build_query(query_wrapper)
    }

    fn build_insert_script(insert_wrapper: &InsertWrapper) -> (String, HashMap<String, RdbcValue>) {
        todo!()
    }

    fn build_update_script(update_wrapper: &UpdateWrapper) -> (String, HashMap<String, RdbcValue>) {
        todo!()
    }

    fn build_delete_script(delete_wrapper: &DeleteWrapper) -> (String, HashMap<String, RdbcValue>) {
        todo!()
    }

    fn build_query_sql(query_wrapper: &QueryWrapper) -> (String, Vec<RdbcValue>) {
        todo!()
    }

    fn build_insert_sql(insert_wrapper: &InsertWrapper) -> (String, Vec<RdbcValue>) {
        todo!()
    }

    fn build_update_sql(update_wrapper: &UpdateWrapper) -> (String, Vec<RdbcValue>) {
        todo!()
    }

    fn build_delete_sql(delete_wrapper: &DeleteWrapper) -> (String, Vec<RdbcValue>) {
        todo!()
    }

    fn build_raw_query(query_wrapper: &QueryWrapper) -> String {
        todo!()
    }

    fn build_raw_insert(insert_wrapper: &InsertWrapper) -> String {
        todo!()
    }

    fn build_raw_update(update_wrapper: &UpdateWrapper) -> String {
        todo!()
    }

    fn build_raw_delete(delete_wrapper: &DeleteWrapper) -> String {
        todo!()
    }
}

struct PgScriptBuilder;

impl PgScriptBuilder {
    fn build_query(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        let mut query_sql = vec![];
        let mut query_params = HashMap::new();
        // select
        let (select_sql, select_params) = Self::build_select(query_wrapper.get_select());
        query_sql.push(select_sql);
        query_params.extend(select_params);
        // from
        let (from_table_sql, from_table_params) = Self::build_table(query_wrapper.get_table());
        query_sql.push(from_table_sql);
        query_params.extend(from_table_params);
        // where
        let (filter_sql, filter_params) = Self::build_filter(query_wrapper.get_filter());
        query_sql.push(filter_sql);
        query_params.extend(filter_params);

        // group by
        let (group_sql, group_params) = Self::build_group(query_wrapper.get_group_by());
        query_sql.push(group_sql);
        query_params.extend(group_params);
        // having
        let (having_sql, having_params) = Self::build_having(query_wrapper.get_having());
        query_sql.push(having_sql);
        query_params.extend(having_params);
        // order by
        let (order_sql, order_params) = Self::build_order(query_wrapper.get_order());
        query_sql.push(order_sql);
        query_params.extend(order_params);
        // limit sql
        let (limit_sql, limit_params) = Self::build_limit(query_wrapper.get_limit());
        query_sql.push(limit_sql);
        query_params.extend(limit_params);
        (query_sql.join("\n"), query_params)
    }
    fn build_select(select_columns: &Vec<RdbcColumn>) -> (String, HashMap<String, RdbcValue>) {
        let mut select_sql = vec![];
        let mut select_params = HashMap::new();
        for column in select_columns {
            let (column_sql, column_params) = match column {
                RdbcColumn::Table(tc) => PgScriptSelectBuilder::build_select_column(tc),
                RdbcColumn::Query(qc) => PgScriptSelectBuilder::build_select_query(qc),
                RdbcColumn::Func(fc) => PgScriptSelectBuilder::build_select_func(fc),
                RdbcColumn::Value(vc) => PgScriptSelectBuilder::build_select_value(vc),
            };
            select_sql.push(column_sql);
            select_params.extend(column_params);
        }
        if select_sql.is_empty() {
            ("*".to_string(), HashMap::new())
        } else {
            (format!("SELECT {}", select_sql.join(",")), select_params)
        }
    }
    fn build_table(from_tables: &Vec<RdbcTableInner>) -> (String, HashMap<String, RdbcValue>) {
        let (mut from_sql, from_prams) = PgScriptTableBuilder::build_table(from_tables);
        if !from_sql.is_empty() {
            from_sql = format!(" FROM {}", from_sql);
        }
        (from_sql, from_prams)
    }
    fn build_filter(filter: Option<&RdbcTableFilterImpl>) -> (String, HashMap<String, RdbcValue>) {
        let (mut filter_sql, filter_params) = PgScriptFilterBuilder::build_filter(filter);
        if !filter_sql.is_empty() {
            filter_sql = format!(" WHERE {}", filter_sql);
        }
        (filter_sql, filter_params)
    }
    fn build_group(group_by: Option<&Vec<RdbcColumn>>) -> (String, HashMap<String, RdbcValue>) {
        let (mut group_sql, group_params) = PgScriptGroupBuilder::build_group(group_by);
        if !group_sql.is_empty() {
            group_sql = format!(" GROUP BY {}", group_sql);
        }
        (group_sql, group_params)
    }
    fn build_having(having: Option<&RdbcTableFilterImpl>) -> (String, HashMap<String, RdbcValue>) {
        let (mut having_sql, having_params) = PgScriptHavingBuilder::build_having(having);
        if !having_sql.is_empty() {
            having_sql = format!(" HAVING {}", having_sql);
        }
        (having_sql, having_params)
    }
    fn build_order(order: Option<&Vec<RdbcOrder>>) -> (String, HashMap<String, RdbcValue>) {
        let (mut order_sql, order_params) = PgScriptOrderBuilder::build_order(order);
        if !order_sql.is_empty() {
            order_sql = format!(" ORDER BY {}", order_sql);
        }
        (order_sql, order_params)
    }
    fn build_limit(limit: Option<&u64>) -> (String, HashMap<String, RdbcValue>) {
        let (mut limit_sql, limit_params) = PgScriptLimitBuilder::build_limit(limit);
        if !limit_sql.is_empty() {
            limit_sql = format!(" LIMIT {}", limit_sql);
        }
        (limit_sql, limit_params)
    }
}

struct PgScriptSelectBuilder;

impl PgScriptSelectBuilder {
    fn build_select_column(tc: &RdbcTableColumn) -> (String, HashMap<String, RdbcValue>) {
        let mut column_sql = tc.get_name().to_string();
        if let Some(alias) = tc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        if let Some(table) = tc.get_table() {
            column_sql = format!("{}.{}", table, column_sql);
        }
        if let Some(schema) = tc.get_schema() {
            column_sql = format!("{}.{}", schema, column_sql);
        }
        (column_sql, HashMap::new())
    }
    fn build_select_query(qc: &RdbcQueryColumn) -> (String, HashMap<String, RdbcValue>) {
        let (mut column_sql, column_params) = PgRdbcSQLBuilder::build_query_script(qc.get_name());
        if let Some(alias) = qc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        (column_sql, column_params)
    }
    fn build_select_func(fc: &RdbcFuncColumn) -> (String, HashMap<String, RdbcValue>) {
        let (mut column_sql, column_params) = PgScriptFucBuilder::build_func(fc.get_name());
        if let Some(alias) = fc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        (column_sql, column_params)
    }
    fn build_select_value(vc: &RdbcValueColumn) -> (String, HashMap<String, RdbcValue>) {
        let mut column_sql = PgScriptValueBuilder::build_value(vc.get_name());
        if let Some(alias) = vc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        (column_sql, HashMap::new())
    }
}

struct PgScriptTableBuilder;

impl PgScriptTableBuilder {
    fn build_table(from_tables: &[RdbcTableInner]) -> (String, HashMap<String, RdbcValue>) {
        let mut table_vec = vec![];
        let mut join_table_vec = vec![];
        for table in from_tables {
            if table.get_join().is_some() {
                join_table_vec.push(table);
            } else {
                table_vec.push(table);
            }
        }
        let (table_sql, mut table_params) = Self::build_table_vec_script(table_vec);
        let (table_join_sql, table_join_params) = Self::build_table_join_vec_script(join_table_vec);
        let table_sql = format!("{} \n {}", table_sql, table_join_sql);
        table_params.extend(table_join_params);
        (table_sql, table_params)
    }

    fn build_table_vec_script(
        from_tables: Vec<&RdbcTableInner>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut table_vec: Vec<String> = vec![];
        let mut table_params = HashMap::new();
        for table in from_tables {
            let (t_sql, t_params) = match table {
                RdbcTableInner::Table(t) => Self::build_schema_table_script(t),
                RdbcTableInner::Query(q) => Self::build_query_table_script(q),
            };
            table_vec.push(t_sql);
            table_params.extend(t_params);
        }
        (table_vec.join(",\n"), table_params)
    }
    fn build_table_join_vec_script(
        from_tables: Vec<&RdbcTableInner>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut table_vec: Vec<String> = vec![];
        let mut table_params = HashMap::new();
        for table in from_tables {
            let (t_sql, t_params) = match table {
                RdbcTableInner::Table(t) => Self::build_schema_table_script(t),
                RdbcTableInner::Query(q) => Self::build_query_table_script(q),
            };
            table_vec.push(t_sql);
            table_params.extend(t_params);
        }

        (table_vec.join("\n"), table_params)
    }

    fn build_schema_table_script(t: &RdbcSchemaTable) -> (String, HashMap<String, RdbcValue>) {
        let mut table_sql = format!("{}", t.get_name());
        let mut table_params = HashMap::new();
        if let Some(alias) = t.get_alias() {
            if !alias.is_empty() {
                table_sql = format!("{} AS \"{}\"", table_sql, alias);
            }
        }
        if let Some(schema) = t.get_schema() {
            if !schema.is_empty() {
                table_sql = format!("{}.{}", schema, table_sql);
            }
        }

        if let Some(join_type) = t.get_join() {
            let join_type_sql = match join_type {
                RdbcTableJoinType::Inner => "INNER JOIN ",
                RdbcTableJoinType::Left => "LEFT JOIN ",
                RdbcTableJoinType::Right => "RIGHT JOIN ",
                RdbcTableJoinType::Full => "FULL JOIN ",
            };
            table_sql = format!("{} {}", join_type_sql, table_sql);

            if let Some(on_filter) = t.get_filter() {
                let (on_filter_sql, on_filter_params) =
                    PgScriptFilterBuilder::build_filter(Some(on_filter));
                if !on_filter_sql.is_empty() {
                    table_sql = format!("{} ON {}", table_sql, on_filter_sql);
                    table_params.extend(on_filter_params);
                }
            }
        }
        (table_sql, table_params)
    }
    fn build_query_table_script(t: &RdbcQueryTable) -> (String, HashMap<String, RdbcValue>) {
        let query_wrapper = t.get_name();
        let (mut query_sql, mut query_params) = PgScriptBuilder::build_query(query_wrapper);
        if query_sql.is_empty() {
            return ("".to_string(), HashMap::new());
        }
        query_sql = format!("({})", query_sql);
        if let Some(alias) = t.get_alias() {
            if !alias.is_empty() {
                query_sql = format!("{} AS \"{}\"", query_sql, alias);
            }
        }

        if let Some(join_type) = t.get_join() {
            let join_type_sql = match join_type {
                RdbcTableJoinType::Inner => "INNER JOIN ",
                RdbcTableJoinType::Left => "LEFT JOIN ",
                RdbcTableJoinType::Right => "RIGHT JOIN ",
                RdbcTableJoinType::Full => "FULL JOIN ",
            };
            query_sql = format!("{} {}", join_type_sql, query_sql);

            if let Some(on_filter) = t.get_filter() {
                let (on_filter_sql, on_filter_params) =
                    PgScriptFilterBuilder::build_filter(Some(on_filter));
                if !on_filter_sql.is_empty() {
                    query_sql = format!("{} ON {}", query_sql, on_filter_sql);
                    query_params.extend(on_filter_params);
                }
            }
        }
        (query_sql, query_params)
    }
}

struct PgScriptFilterBuilder;
impl PgScriptFilterBuilder {
    fn build_filter(filter: Option<&RdbcTableFilterImpl>) -> (String, HashMap<String, RdbcValue>) {
        if filter.is_none() {
            return ("".to_string(), HashMap::new());
        }
        ("".to_string(), HashMap::new())
    }
}

struct PgScriptColumnBuilder;
impl PgScriptColumnBuilder {
    fn build_column(rdbc_column: &crate::RdbcColumn) -> (String, HashMap<String, RdbcValue>) {
        match rdbc_column {
            RdbcColumn::Table(tc) => Self::build_table_column(tc),
            RdbcColumn::Query(qc) => Self::build_query_column(qc),
            RdbcColumn::Func(fc) => Self::build_func_column(fc),
            RdbcColumn::Value(vc) => Self::build_value_column(vc),
        }
    }

    fn build_value_column(vc: &RdbcValueColumn) -> (String, HashMap<String, RdbcValue>) {
        let v = PgScriptValueBuilder::build_value(vc.get_name());
        if let Some(alias) = vc.get_alias() {
            if !alias.is_empty() {
                return (format!("{} AS \"{}\"", v, alias), HashMap::new());
            }
        }
        (v, HashMap::new())
    }

    fn build_query_column(qc: &RdbcQueryColumn) -> (String, HashMap<String, RdbcValue>) {
        let (mut query_sql, query_params) = PgScriptBuilder::build_query(qc.get_name());
        if let Some(alias) = qc.get_alias() {
            if !alias.is_empty() {
                query_sql = format!("({}) AS \"{}\"", query_sql, alias);
            }
        }
        (query_sql, query_params)
    }

    fn build_table_column(tc: &RdbcTableColumn) -> (String, HashMap<String, RdbcValue>) {
        let mut column_sql = format!("{}", tc.get_name());
        let column_params = HashMap::new();
        if let Some(alias) = tc.get_alias() {
            if !alias.is_empty() {
                column_sql = format!("{} AS \"{}\"", column_sql, alias);
            }
        }
        if let Some(table_alias) = tc.get_table() {
            if !table_alias.is_empty() {
                column_sql = format!("{}.{}", table_alias, column_sql);
            }
        }
        if let Some(schema) = tc.get_schema() {
            if !schema.is_empty() {
                column_sql = format!("{}.{}", schema, column_sql);
            }
        }
        (column_sql, column_params)
    }

    fn build_func_column(fc: &RdbcFuncColumn) -> (String, HashMap<String, RdbcValue>) {
        let column = fc.get_name();
        let (mut func_sql, func_params) = PgScriptFucBuilder::build_func(column);
        if let Some(alias) = fc.get_alias() {
            if !alias.is_empty() {
                func_sql = format!("{} AS \"{}\"", func_sql, alias);
            }
        }
        (func_sql, func_params)
    }
}

struct PgScriptFucBuilder;

impl PgScriptFucBuilder {
    fn build_func(func: &RdbcFunc) -> (String, HashMap<String, RdbcValue>) {
        match func {
            RdbcFunc::CONCAT(v) => Self::build_concat(v),
            RdbcFunc::REPLACE(v) => Self::build_replace(v),
        }
    }

    fn build_concat(func_column: &crate::RdbcConcatFunc) -> (String, HashMap<String, RdbcValue>) {
        let mut concat_vec: Vec<String> = vec![];
        let mut concat_params = HashMap::new();
        let concat_str = func_column.get_liter().unwrap_or(&"".to_string()).clone();
        for item in func_column.get_columns() {
            let (column_sql, column_params) = PgScriptColumnBuilder::build_column(item);
            concat_vec.push(column_sql);
            concat_params.extend(column_params);
        }
        (concat_vec.join(&concat_str), concat_params)
    }

    fn build_replace(func_column: &crate::RdbcReplaceFunc) -> (String, HashMap<String, RdbcValue>) {
        let column = func_column.get_column();
        let (column_sql, column_params) =
            PgScriptColumnBuilder::build_column(PgScriptColumnBuilder::build_table_column(column));
        let old_value = func_column.get_old_value();
        let new_value = func_column.get_new_value();
        (
            format!("REPLACE({},{},{})", column_sql, old_value, new_value),
            column_params,
        )
    }
}

struct PgScriptOrderBuilder;
impl PgScriptOrderBuilder {
    fn build_order(order_op: Option<&Vec<RdbcOrder>>) -> (String, HashMap<String, RdbcValue>) {
        let mut order_vec: Vec<String> = vec![];
        let mut order_params = HashMap::new();
        if let Some(orders) = order_op {
            for item in orders {
                let (fileld_sql, field_params) = match item {
                    RdbcOrder::Column(order_col) => Self::build_column_order(order_col),
                };
                order_vec.push(fileld_sql);
                order_params.extend(field_params);
            }
        }

        (order_vec.join(","), order_params)
    }

    fn build_column_order(
        order_col: &crate::RdbcColumnOrder,
    ) -> (String, HashMap<String, RdbcValue>) {
        let order_type = match order_col.get_order() {
            RdbcOrderType::Asc => " ASC ".to_string(),
            RdbcOrderType::Desc => " DESC ".to_string(),
        };
        let (coloumn_sql, column_params) =
            PgScriptColumnBuilder::build_column(order_col.get_column());
        if !coloumn_sql.is_empty() {
            return (format!("{} {}", coloumn_sql, order_type), column_params);
        }
        ("".to_string(), HashMap::new())
    }
}
struct PgScriptGroupBuilder;
impl PgScriptGroupBuilder {
    fn build_group(group_by: Option<&Vec<RdbcColumn>>) -> (String, HashMap<String, RdbcValue>) {
        let mut group_vec: Vec<String> = vec![];
        let mut group_params = HashMap::new();
        if let Some(group_by_orders) = group_by {
            for item in group_by_orders {
                let (fileld_sql, field_params) = PgScriptColumnBuilder::build_column(item);
                group_vec.push(fileld_sql);
                group_params.extend(field_params);
            }
        }
        (group_vec.join(","), group_params)
    }
}
struct PgScriptHavingBuilder;
impl PgScriptHavingBuilder {
    fn build_having(having: Option<&RdbcTableFilterImpl>) -> (String, HashMap<String, RdbcValue>) {
        let (mut having_sql, having_params) = PgScriptFilterBuilder::build_filter(having);
        if !having_sql.is_empty() {
            having_sql = format!("HAVING {}", having_sql);
        }
        (having_sql, having_params)
    }
}
struct PgScriptLimitBuilder;
impl PgScriptLimitBuilder {
    fn build_limit(limit: Option<&u64>) -> (String, HashMap<String, RdbcValue>) {
        match limit {
            Some(v) => (format!("{}", v), HashMap::new()),
            None => ("".to_string(), HashMap::new()),
        }
    }
}

struct PgScriptValueBuilder;
impl PgScriptValueBuilder {
    fn build_value(value: &RdbcValue) -> String {
        match value {
            RdbcValue::Int(v) => {
                format!("{}", v)
            }
            RdbcValue::BigInt(v) => {
                format!("{}", v)
            }
            RdbcValue::Float(v) => {
                format!("{}", v)
            }
            RdbcValue::BigFloat(v) => {
                format!("{}", v)
            }
            RdbcValue::String(v) => {
                format!("'{}'", v)
            }
            RdbcValue::DateTime(v) => {
                format!("'{}'", v)
            }
            RdbcValue::Bool(v) => {
                if *v {
                    "'true'".to_string()
                } else {
                    "'false'".to_string()
                }
            }
            RdbcValue::Vec(_) => "''".to_string(),
            RdbcValue::Map(_) => "''".to_string(),
            RdbcValue::Null => "null".to_string(),
        }
    }
}

struct PgSQLBuilder;
impl PgSQLBuilder {}

struct PgRawSqlBuilder;
impl PgRawSqlBuilder {}
