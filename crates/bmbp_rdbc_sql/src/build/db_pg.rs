use crate::build::types::RdbcSQLBuilder;
use crate::{
    table, DeleteWrapper, InsertWrapper, QueryWrapper, RdbcColumn, RdbcFunc, RdbcFuncColumn,
    RdbcOrder, RdbcQueryColumn, RdbcTableColumn, RdbcTableFilterImpl, RdbcTableInner,
    RdbcValueColumn, UpdateWrapper,
};
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;

pub struct PgSqlBuilder;

impl RdbcSQLBuilder for PgSqlBuilder {
    fn build_query_script(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        PgScriptBuilder::build_query_script(query_wrapper)
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
    fn build_query_script(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        let mut query_sql = vec![];
        let mut query_params = HashMap::new();
        // select
        let (select_sql, select_params) =
            Self::build_query_select_script(query_wrapper.get_select());
        query_sql.push(select_sql);
        query_params.extend(select_params);
        // from
        let (from_table_sql, from_table_params) =
            Self::build_query_table_script(query_wrapper.get_table());
        query_sql.push(from_table_sql);
        query_params.extend(from_table_params);
        // where
        let (filter_sql, filter_params) =
            Self::build_query_filter_script(query_wrapper.get_filter());
        query_sql.push(filter_sql);
        query_params.extend(filter_params);

        // group by
        let (group_sql, group_params) =
            Self::build_query_group_script(query_wrapper.get_group_by());
        query_sql.push(group_sql);
        query_params.extend(group_params);

        // having
        let (having_sql, having_params) =
            Self::build_query_having_script(query_wrapper.get_having());
        query_sql.push(having_sql);
        query_params.extend(having_params);

        // order by
        let (order_sql, order_params) = Self::build_query_order_script(query_wrapper.get_order());
        query_sql.push(order_sql);
        query_params.extend(order_params);
        // limit
        return (query_sql.join("\n"), query_params);
    }
    fn build_query_select_script(
        select_columns: &Vec<RdbcColumn>,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut select_sql = vec![];
        let mut select_params = HashMap::new();
        for column in select_columns {
            let (column_sql, column_params) = match column {
                RdbcColumn::Table(tc) => PgSelectSqlBuilder::build_select_table_column_script(tc),
                RdbcColumn::Query(qc) => PgSelectSqlBuilder::build_select_query_column_script(qc),
                RdbcColumn::Func(fc) => PgSelectSqlBuilder::build_select_func_column_script(fc),
                RdbcColumn::Value(vc) => PgSelectSqlBuilder::build_select_value_column_script(vc),
            };
            select_sql.push(column_sql);
            select_params.extend(column_params);
        }
        if select_sql.is_empty() {
            ("".to_string(), HashMap::new())
        } else {
            (format!("SELECT {}", select_sql.join(",")), select_params)
        }
    }
    fn build_query_table_script(
        from_tables: &Vec<RdbcTableInner>,
    ) -> (String, HashMap<String, RdbcValue>) {
        PgTableSqlBUilder::build_table_script(from_tables)
    }
    fn build_query_filter_script(
        filter: Option<&RdbcTableFilterImpl>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_query_group_script(
        group_by: Option<&Vec<RdbcColumn>>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_query_having_script(
        having: Option<&RdbcTableFilterImpl>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_query_order_script(
        order: Option<&Vec<RdbcOrder>>,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_query_limit_script(limit: &Option<i64>) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
}

struct PgSelectSqlBuilder;

impl PgSelectSqlBuilder {
    fn build_select_table_column_script(
        tc: &RdbcTableColumn,
    ) -> (String, HashMap<String, RdbcValue>) {
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
    fn build_select_query_column_script(
        qc: &RdbcQueryColumn,
    ) -> (String, HashMap<String, RdbcValue>) {
        let (mut column_sql, column_params) = PgSqlBuilder::build_query_script(qc.get_name());
        if let Some(alias) = qc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        (column_sql, column_params)
    }
    fn build_select_func_column_script(
        fc: &RdbcFuncColumn,
    ) -> (String, HashMap<String, RdbcValue>) {
        let (mut column_sql, column_params) = PgFuncScriptBuilder::build_func_script(fc.get_name());
        if let Some(alias) = fc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        (column_sql, column_params)
    }
    fn build_select_value_column_script(
        vc: &RdbcValueColumn,
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut column_sql = match vc.get_name() {
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
        };
        if let Some(alias) = vc.get_alias() {
            column_sql = format!("{} AS \"{}\"", column_sql, alias);
        }
        (column_sql, HashMap::new())
    }
}

struct PgTableSqlBUilder;
impl PgTableSqlBUilder {
    fn build_table_script(from_tables: &[RdbcTableInner]) -> (String, HashMap<String, RdbcValue>) {
        let mut table_vec: Vec<String> = vec![];
        let mut table_params = HashMap::new();
        for table in from_tables {
            match table {
                RdbcTableInner::Table(t) => {}
                RdbcTableInner::Query(q) => {}
            }
        }

        (table_vec.join("\n"), table_params)
    }
}

struct PgFuncScriptBuilder;

impl PgFuncScriptBuilder {
    fn build_func_script(func: &RdbcFunc) -> (String, HashMap<String, RdbcValue>) {
        match func {
            RdbcFunc::CONCAT(v) => {}
            RdbcFunc::REPLACE(v) => {}
        }
        ("".to_string(), HashMap::new())
    }
}

struct PgSQLBuilder;

struct PgRawSQLBuilder;
