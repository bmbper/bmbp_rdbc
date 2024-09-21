use crate::build::types::RdbcSQLBuilder;
use crate::{
    DeleteWrapper, InsertWrapper, QueryFilter, QueryWrapper, RdbcColumn, RdbcFunc, RdbcFuncColumn,
    RdbcOrder, RdbcOrderType, RdbcQueryColumn, RdbcQueryTable, RdbcSchemaTable, RdbcTableColumn,
    RdbcTableInner, RdbcTableJoinType, RdbcValueColumn, UpdateWrapper,
};
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;
use std::vec;
use uuid::Uuid;

pub struct PgRdbcSQLBuilder;
impl PgRdbcSQLBuilder {
    fn convert_script_to_sql(
        script_sql: &String,
        script_params: &HashMap<String, RdbcValue>,
    ) -> (String, Vec<RdbcValue>) {
        let mut sql = script_sql.clone();
        let mut sql_parmas = vec![];
        for (index, key) in script_params.keys().enumerate() {
            let v = script_params.get(key).unwrap_or(&RdbcValue::Null).clone();
            let params_str = format!("#{{{}}}", key);
            let params_index = format!("${}", index + 1);
            sql = sql.replace(&params_str, &params_index);
            sql_parmas.push(v);
        }
        (sql, sql_parmas)
    }
    fn convert_script_to_raw_sql(
        script_sql: &String,
        script_params: &HashMap<String, RdbcValue>,
    ) -> String {
        let mut raw_sql = script_sql.clone();
        for key in script_params.keys() {
            let v = script_params.get(key).unwrap_or(&RdbcValue::Null).clone();
            let params_str = format!("#{{{}}}", key);
            let value_index = PgRawValueBuilder::raw_value(&v);
            raw_sql = raw_sql.replace(&params_str, &value_index);
        }
        raw_sql
    }
}

impl RdbcSQLBuilder for PgRdbcSQLBuilder {
    fn build_query_script(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        PgScriptBuilder::build_query(query_wrapper)
    }

    fn build_insert_script(insert_wrapper: &InsertWrapper) -> (String, HashMap<String, RdbcValue>) {
        PgScriptBuilder::build_insert(insert_wrapper)
    }

    fn build_update_script(update_wrapper: &UpdateWrapper) -> (String, HashMap<String, RdbcValue>) {
        PgScriptBuilder::build_update(update_wrapper)
    }

    fn build_delete_script(delete_wrapper: &DeleteWrapper) -> (String, HashMap<String, RdbcValue>) {
        PgScriptBuilder::build_delete(delete_wrapper)
    }

    fn build_query_sql(query_wrapper: &QueryWrapper) -> (String, Vec<RdbcValue>) {
        let (query_sql, query_params) = Self::build_query_script(query_wrapper);
        Self::convert_script_to_sql(&query_sql, &query_params)
    }
    fn build_insert_sql(insert_wrapper: &InsertWrapper) -> (String, Vec<RdbcValue>) {
        let (insert_sql, insert_params) = Self::build_insert_script(insert_wrapper);
        Self::convert_script_to_sql(&insert_sql, &insert_params)
    }
    fn build_update_sql(update_wrapper: &UpdateWrapper) -> (String, Vec<RdbcValue>) {
        let (update_sql, update_params) = Self::build_update_script(update_wrapper);
        Self::convert_script_to_sql(&update_sql, &update_params)
    }

    fn build_delete_sql(delete_wrapper: &DeleteWrapper) -> (String, Vec<RdbcValue>) {
        let (delete_sql, delete_params) = Self::build_delete_script(delete_wrapper);
        Self::convert_script_to_sql(&delete_sql, &delete_params)
    }

    fn build_raw_query(query_wrapper: &QueryWrapper) -> String {
        let (query_sql, query_params) = Self::build_query_script(query_wrapper);
        Self::convert_script_to_raw_sql(&query_sql, &query_params)
    }

    fn build_raw_insert(insert_wrapper: &InsertWrapper) -> String {
        let (insert_sql, insert_params) = Self::build_insert_script(insert_wrapper);
        Self::convert_script_to_raw_sql(&insert_sql, &insert_params)
    }

    fn build_raw_update(update_wrapper: &UpdateWrapper) -> String {
        let (update_sql, update_params) = Self::build_update_script(update_wrapper);
        Self::convert_script_to_raw_sql(&update_sql, &update_params)
    }

    fn build_raw_delete(delete_wrapper: &DeleteWrapper) -> String {
        let (delete_sql, delete_params) = Self::build_delete_script(delete_wrapper);
        Self::convert_script_to_raw_sql(&delete_sql, &delete_params)
    }
}

struct PgScriptBuilder;

impl PgScriptBuilder {
    fn build_query(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        let mut query_sql = vec![];
        let mut query_params = HashMap::new();
        // select
        let (select_sql, select_params) =
            Self::build_select(query_wrapper.get_select(), query_wrapper.get_distinct());
        if !select_sql.is_empty() {
            query_sql.push(select_sql);
            query_params.extend(select_params);
        }
        // from
        let (from_table_sql, from_table_params) = Self::build_table(query_wrapper.get_table());
        if !from_table_sql.is_empty() {
            query_sql.push(from_table_sql);
            query_params.extend(from_table_params);
        }
        // join
        let (join_table_sql, join_table_params) =
            Self::build_table_join(query_wrapper.get_join().unwrap_or(&vec![]));
        if !join_table_sql.is_empty() {
            query_sql.push(join_table_sql);
            query_params.extend(join_table_params);
        }
        // where
        let (filter_sql, filter_params) = Self::build_filter(query_wrapper.get_filter());
        if !filter_sql.is_empty() {
            query_sql.push(filter_sql);
            query_params.extend(filter_params);
        }

        // group by
        let (group_sql, group_params) = Self::build_group(query_wrapper.get_group_by());
        if !group_sql.is_empty() {
            query_sql.push(group_sql);
            query_params.extend(group_params);
        }

        // having
        let (having_sql, having_params) = Self::build_having(query_wrapper.get_having());
        if !having_sql.is_empty() {
            query_sql.push(having_sql);
            query_params.extend(having_params);
        }

        // order by
        let (order_sql, order_params) = Self::build_order(query_wrapper.get_order());
        if !order_sql.is_empty() {
            query_sql.push(order_sql);
            query_params.extend(order_params);
        }
        // limit sql
        let (limit_sql, limit_params) = Self::build_limit(query_wrapper.get_limit());
        if !limit_sql.is_empty() {
            query_sql.push(limit_sql);
            query_params.extend(limit_params);
        }

        let (offset_sql, offset_params) = Self::build_offset(query_wrapper.get_offset());
        if !offset_sql.is_empty() {
            query_sql.push(offset_sql);
            query_params.extend(offset_params);
        }

        let mut union_all_sql = vec![];
        let mut union_all_params = HashMap::new();
        if let Some(union_vec) = query_wrapper.get_union_all() {
            for union_wrapper in union_vec {
                let (union_sql, union_params) = Self::build_query(union_wrapper);
                union_all_sql.push(union_sql);
                union_all_params.extend(union_params);
            }
        }
        if !union_all_sql.is_empty() {
            let union_sql = union_all_sql.join("\n UNION ALL \n");
            query_sql.push(format!(" UNION ALL {}", union_sql));
            query_params.extend(union_all_params);
        }
        let mut union_only_sql = vec![];
        let mut union_only_params = HashMap::new();
        if let Some(union_vec) = query_wrapper.get_union_only() {
            for union_wrapper in union_vec {
                let (union_sql, union_params) = Self::build_query(union_wrapper);
                union_only_sql.push(union_sql);
                union_only_params.extend(union_params);
            }
        }
        if !union_only_sql.is_empty() {
            let union_sql = union_only_sql.join("\n UNION \n");
            query_sql.push(format!(" UNION {}", union_sql));
            query_params.extend(union_only_params);
        }
        (query_sql.join("\n"), query_params)
    }
    fn build_select(
        select_columns: &Vec<RdbcColumn>,
        is_distinct: bool,
    ) -> (String, HashMap<String, RdbcValue>) {
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
            if is_distinct {
                ("SELECT DISTINCT * ".to_string(), HashMap::new())
            } else {
                ("SELECT * ".to_string(), HashMap::new())
            }
        } else {
            if is_distinct {
                (
                    format!("SELECT DISTINCT {}", select_sql.join(",")),
                    select_params,
                )
            } else {
                (format!("SELECT {}", select_sql.join(",")), select_params)
            }
        }
    }
    fn build_table(from_tables: &Vec<RdbcTableInner>) -> (String, HashMap<String, RdbcValue>) {
        let (mut from_sql, from_prams) = PgScriptTableBuilder::build_table(from_tables);
        if !from_sql.is_empty() {
            from_sql = format!(" FROM {}", from_sql);
        }
        (from_sql, from_prams)
    }
    fn build_table_join(from_tables: &Vec<RdbcTableInner>) -> (String, HashMap<String, RdbcValue>) {
        PgScriptTableBuilder::build_table(from_tables)
    }
    fn build_filter(filter: Option<&QueryFilter>) -> (String, HashMap<String, RdbcValue>) {
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
    fn build_having(having: Option<&QueryFilter>) -> (String, HashMap<String, RdbcValue>) {
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
    fn build_offset(offset: Option<&u64>) -> (String, HashMap<String, RdbcValue>) {
        let (mut offset_sql, limit_params) = PgScriptLimitBuilder::build_limit(offset);
        if !offset_sql.is_empty() {
            offset_sql = format!(" OFFSET {}", offset_sql);
        }
        (offset_sql, limit_params)
    }
    fn build_insert(insert_wrapper: &InsertWrapper) -> (String, HashMap<String, RdbcValue>) {
        let mut insert_params = HashMap::new();
        let (table_sql, table_params) =
            PgScriptTableBuilder::build_table(&insert_wrapper.get_table());
        let mut insert_sql = format!("INSERT INTO {}", table_sql);
        insert_params.extend(table_params);

        let mut c_v_c_sql = vec![];
        let mut c_v_v_sql = vec![];
        let mut c_v_params = HashMap::new();
        let c_v_vec = insert_wrapper.get_column_values();
        if !c_v_vec.is_empty() {
            for (c, v) in c_v_vec.iter() {
                let (tc_sql, tc_params) = PgScriptColumnBuilder::build_table_column(c);
                c_v_c_sql.push(tc_sql);
                c_v_params.extend(tc_params);
                match v {
                    crate::RdbcDmlValue::VALUE(rdbc_value) => {
                        let col_id = Uuid::new_v4().simple().to_string();
                        let v_sql = format!("#{{{}}}", col_id.clone());
                        c_v_v_sql.push(v_sql);
                        c_v_params.insert(col_id, rdbc_value.clone());
                    }
                    crate::RdbcDmlValue::COLUMN(rdbc_column) => {
                        let (col_v_sql, _) = PgScriptColumnBuilder::build_column(rdbc_column);
                        c_v_v_sql.push(col_v_sql);
                    }
                }
            }
        }

        let mut c_sql = vec![];
        let mut c_params = HashMap::new();
        let columns = insert_wrapper.get_column();
        for item in columns {
            let (c_tmp_sql, c_tmp_params) = PgScriptColumnBuilder::build_table_column(item);
            c_sql.push(c_tmp_sql);
            c_params.extend(c_tmp_params);
        }
        // insert into table(columns) value select * from table1
        if let Some(query) = insert_wrapper.get_query() {
            let (query_sql, query_params) = Self::build_query(query);
            if !c_sql.is_empty() {
                insert_sql = format!("{} ({}) ", insert_sql, c_sql.join(","));
            }
            insert_sql = format!("{} VALUE {}", insert_sql, query_sql);
            insert_params.extend(query_params);
            (insert_sql, insert_params)
        } else {
            c_v_c_sql.extend_from_slice(c_sql.as_slice());
            c_v_params.extend(c_params);
            for item in insert_wrapper.get_values() {
                match item {
                    crate::RdbcDmlValue::VALUE(rdbc_value) => {
                        let col_id = Uuid::new_v4().simple().to_string();
                        let v_sql = format!("#{{{}}}", col_id.clone());
                        c_v_v_sql.push(v_sql);
                        c_v_params.insert(col_id, rdbc_value.clone());
                    }
                    crate::RdbcDmlValue::COLUMN(rdbc_column) => {
                        let (col_v_sql, _) = PgScriptColumnBuilder::build_column(rdbc_column);
                        c_v_v_sql.push(col_v_sql);
                    }
                }
            }
            insert_params.extend(c_v_params);

            if !c_v_c_sql.is_empty() {
                insert_sql = format!("{} ({}) ", insert_sql, c_v_c_sql.join(","));
            }

            insert_sql = format!("{} VALUES ({})", insert_sql, c_v_v_sql.join(","));
            (insert_sql, insert_params)
        }
    }

    fn build_update(update_wrapper: &UpdateWrapper) -> (String, HashMap<String, RdbcValue>) {
        let mut update_sql = vec![];
        let mut update_prams = HashMap::new();
        let update_tables = update_wrapper.get_table();
        let mut major_table = vec![];
        let mut from_table = vec![];
        if !update_tables.is_empty() {
            major_table.push(update_tables.get(0).unwrap());
            for index in 1..update_tables.len() {
                from_table.push(update_tables.get(index).unwrap());
            }
        }

        // update table
        let (table_sql, table_prams) =
            PgScriptTableBuilder::build_table_by_slice(major_table.as_slice());
        if !table_sql.is_empty() {
            update_sql.push(format!(" UPDATE {}", table_sql));
            update_prams.extend(table_prams);
        }

        // set_values
        let mut set_sql = vec![];
        let mut set_params = HashMap::new();
        for (col, value_op) in update_wrapper.get_set_values() {
            let (col_sql, col_params) = PgScriptColumnBuilder::build_column(col);
            if let Some(value) = value_op {
                match value {
                    crate::RdbcDmlValue::VALUE(rdbc_value) => {
                        let col_id = Uuid::new_v4().simple().to_string();
                        set_sql.push(format!("{} = #{{{}}}", col_sql, col_id));
                        set_params.insert(col_id, rdbc_value.clone());
                    }
                    crate::RdbcDmlValue::COLUMN(rdbc_column) => {
                        let (col_v_sql, _) = PgScriptColumnBuilder::build_column(rdbc_column);
                        set_sql.push(format!("{} = {}", col_sql, col_v_sql));
                    }
                }
            } else {
                set_sql.push(format!("{} = NULL", col_sql));
                set_params.extend(col_params);
            }
        }
        if !set_sql.is_empty() {
            update_sql.push(format!("SET {} ", set_sql.join(",")));
            update_prams.extend(set_params);
        }
        // from table
        // update table
        let (from_sql, from_prams) =
            PgScriptTableBuilder::build_table_by_slice(from_table.as_slice());
        println!("from_sql=====>{},{}", from_sql, from_sql.len());
        if !from_sql.is_empty() {
            update_sql.push(format!(" FROM {}", from_sql));
            update_prams.extend(from_prams);
        }
        // join
        let (join_table_sql, join_table_params) =
            Self::build_table_join(update_wrapper.get_join().unwrap_or(&vec![]));
        update_sql.push(join_table_sql);
        update_prams.extend(join_table_params);

        // where
        let (filter_sql, filter_params) = Self::build_filter(update_wrapper.get_filter());
        update_sql.push(filter_sql);
        update_prams.extend(filter_params);
        // group by
        let (group_sql, group_params) = Self::build_group(update_wrapper.get_group_by());
        update_sql.push(group_sql);
        update_prams.extend(group_params);
        // having
        let (having_sql, having_params) = Self::build_having(update_wrapper.get_having());
        update_sql.push(having_sql);
        update_prams.extend(having_params);
        // order by
        let (order_sql, order_params) = Self::build_order(update_wrapper.get_order());
        update_sql.push(order_sql);
        update_prams.extend(order_params);
        // limit sql
        let (limit_sql, limit_params) = Self::build_limit(update_wrapper.get_limit());
        update_sql.push(limit_sql);
        update_prams.extend(limit_params);
        let (offset_sql, offset_params) = Self::build_offset(update_wrapper.get_offset());
        update_sql.push(offset_sql);
        update_prams.extend(offset_params);
        (update_sql.join("\n"), update_prams)
    }

    fn build_delete(delete_wrapper: &DeleteWrapper) -> (String, HashMap<String, RdbcValue>) {
        let mut delete_sql = vec![];
        let mut delete_params = HashMap::new();
        // DELETE
        delete_sql.push("DELETE".to_string());
        // from
        let (from_table_sql, from_table_params) = Self::build_table(delete_wrapper.get_table());
        delete_sql.push(from_table_sql);
        delete_params.extend(from_table_params);

        // join
        let (join_table_sql, join_table_params) =
            Self::build_table_join(delete_wrapper.get_join().unwrap_or(&vec![]));
        delete_sql.push(join_table_sql);
        delete_params.extend(join_table_params);
        // where
        let (filter_sql, filter_params) = Self::build_filter(delete_wrapper.get_filter());
        delete_sql.push(filter_sql);
        delete_params.extend(filter_params);
        // group by
        let (group_sql, group_params) = Self::build_group(delete_wrapper.get_group_by());
        delete_sql.push(group_sql);
        delete_params.extend(group_params);
        // having
        let (having_sql, having_params) = Self::build_having(delete_wrapper.get_having());
        delete_sql.push(having_sql);
        delete_params.extend(having_params);
        // order by
        let (order_sql, order_params) = Self::build_order(delete_wrapper.get_order());
        delete_sql.push(order_sql);
        delete_params.extend(order_params);
        // limit sql
        let (limit_sql, limit_params) = Self::build_limit(delete_wrapper.get_limit());
        delete_sql.push(limit_sql);
        delete_params.extend(limit_params);
        let (offset_sql, offset_params) = Self::build_offset(delete_wrapper.get_offset());
        delete_sql.push(offset_sql);
        delete_params.extend(offset_params);
        (delete_sql.join("\n"), delete_params)
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
    fn build_table_by_slice(
        from_tables: &[&RdbcTableInner],
    ) -> (String, HashMap<String, RdbcValue>) {
        let mut table_vec = vec![];
        let mut join_table_vec = vec![];
        for table in from_tables {
            if table.get_join().is_some() {
                join_table_vec.push(table);
            } else {
                table_vec.push(table);
            }
        }

        let mut table_sql_vec = vec![];
        let mut table_sql_params = HashMap::new();
        let (table_sql, table_params) = Self::build_table_vec_script(table_vec);
        if !table_sql.is_empty() {
            table_sql_vec.push(table_sql);
            table_sql_params.extend(table_params);
        }
        let (table_join_sql, table_join_params) = Self::build_table_join_vec_script(join_table_vec);
        if table_join_sql.is_empty() {
            table_sql_vec.push(table_join_sql);
            table_sql_params.extend(table_join_params);
        }

        (table_sql_vec.join("\n"), table_sql_params)
    }

    fn build_table(from_tables: &Vec<RdbcTableInner>) -> (String, HashMap<String, RdbcValue>) {
        let mut table_slice = vec![];
        for item in from_tables {
            table_slice.push(item);
        }
        Self::build_table_by_slice(table_slice.as_slice())
    }

    fn build_table_vec_script(
        from_tables: Vec<&&RdbcTableInner>,
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
        from_tables: Vec<&&RdbcTableInner>,
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
    fn build_filter(filter_op: Option<&QueryFilter>) -> (String, HashMap<String, RdbcValue>) {
        if filter_op.is_none() {
            return ("".to_string(), HashMap::new());
        }
        let filter = filter_op.as_ref().unwrap();

        let concat = match filter.get_concat() {
            crate::RdbcConcatType::And => " AND ".to_string(),
            crate::RdbcConcatType::Or => " OR ".to_string(),
        };

        let mut filter_vec: Vec<String> = vec![];
        let mut filter_params = HashMap::new();

        for item in filter.get_item() {
            let (item_sql, item_params) = Self::build_filter_item(item);
            filter_vec.push(item_sql);
            filter_params.extend(item_params);
        }

        if let Some(params) = filter.get_params() {
            filter_params.extend(params.clone());
        }
        (filter_vec.join(concat.as_str()), filter_params)
    }

    fn build_filter_item(
        filter_item: &crate::QueryFilterItem,
    ) -> (String, HashMap<String, RdbcValue>) {
        match filter_item {
            crate::QueryFilterItem::Value(v) => Self::build_filter_value_item(v),
            crate::QueryFilterItem::Column(v) => Self::build_filter_column_item(v),
            crate::QueryFilterItem::Filter(v) => Self::build_filter_table_item(v),
            crate::QueryFilterItem::Query(v) => Self::build_filter_query_item(v),
        }
    }

    fn build_filter_value_item(
        value_filter: &crate::RdbcValueFilterItem,
    ) -> (String, HashMap<String, RdbcValue>) {
        let column = value_filter.get_column();
        let (mut col_sql, mut col_params) = PgScriptColumnBuilder::build_column(column);
        let compare = value_filter.get_compare();
        let value = value_filter.get_value();
        let ignore = value_filter.get_ignore_null();
        match compare {
            crate::RdbcCompareType::Eq => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} = #{{{}}}", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::NotEq => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} != #{{{}}}", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NOT NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::Gt => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} > #{{{}}}", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::GtEq => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} >= #{{{}}}", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::Lt => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} < #{{{}}}", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::LtEq => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} <= #{{{}}}", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::Like => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!(
                            "{} LIKE CONCAT('%',#{{{}}}::text,'%')",
                            col_sql,
                            col_id.clone()
                        );
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::LikeLeft => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql =
                            format!("{} LIKE CONCAT(#{{{}}}::text,'%')", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::LikeRight => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql =
                            format!("{} LIKE CONCAT('%',#{{{}}}::text)", col_sql, col_id.clone());
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::NotLike => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!(
                            "{} NOT LIKE CONCAT('%',#{{{}}}::text,'%')",
                            col_sql,
                            col_id.clone()
                        );
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NOT NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::NotLikeLeft => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!(
                            "{} NOT LIKE CONCAT(#{{{}}}::text,'%')",
                            col_sql,
                            col_id.clone()
                        );
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NOT NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::NotLikeRight => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!(
                            "{} NOT LIKE CONCAT('%',#{{{}}}::text)",
                            col_sql,
                            col_id.clone()
                        );
                        col_params.insert(col_id, v.clone());
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NOT NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::In => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} = ANY (#{{{}}})", col_sql, col_id.clone());
                        col_params.insert(col_id, RdbcValue::Vec(v.convert_to_vec()));
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::NotIn => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id = Uuid::new_v4().simple().to_string();
                        col_sql = format!("{} NOT ANY (#{{{}}})", col_sql, col_id.clone());

                        col_params.insert(col_id, RdbcValue::Vec(v.convert_to_vec()));
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NOT NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::IsNull => {
                col_sql = format!("{} IS NULL", col_sql);
            }
            crate::RdbcCompareType::IsNotNull => {
                col_sql = format!("{} IS NOT NULL", col_sql);
            }
            crate::RdbcCompareType::Exists => {
                let col_id = Uuid::new_v4().simple().to_string();
                col_sql = format!("{} EXISTS (${{{}}})", col_id, col_sql);
            }
            crate::RdbcCompareType::NotExists => {
                let col_id = Uuid::new_v4().simple().to_string();
                col_sql = format!("{} NOT EXISTS (${{{}}})", col_id, col_sql);
            }
            crate::RdbcCompareType::Between => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id_start = Uuid::new_v4().simple().to_string();
                        let col_id_end = Uuid::new_v4().simple().to_string();
                        let col_value = v.convert_to_vec();
                        if col_value.len().clone() >= 2 {
                            col_sql = format!(
                                "{} BETWEEN #{{{}}} AND #{{{}}} ",
                                col_sql,
                                col_id_start.clone(),
                                col_id_end.clone()
                            );
                            col_params.insert(col_id_start, col_value[0].clone());
                            col_params.insert(col_id_end, col_value[1].clone());
                        } else if col_value.len().clone() == 1 {
                            col_sql = format!("{} BETWEEN #{{{}}} ", col_sql, col_id_start.clone());
                            col_params.insert(col_id_start, col_value[0].clone());
                        } else {
                            col_sql = format!("{} BETWEEN  ", col_sql);
                        }
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS  NULL", col_sql);
                    }
                }
            }
            crate::RdbcCompareType::NotBetween => {
                if let Some(v) = value {
                    if (v.is_null() && !ignore) || !v.is_null() {
                        let col_id_start = Uuid::new_v4().simple().to_string();
                        let col_id_end = Uuid::new_v4().simple().to_string();
                        let col_value = v.convert_to_vec();
                        if col_value.len().clone() >= 2 {
                            col_sql = format!(
                                "{} NOT BETWEEN #{{{}}} AND #{{{}}} ",
                                col_sql,
                                col_id_start.clone(),
                                col_id_end.clone()
                            );
                            col_params.insert(col_id_start, col_value[0].clone());
                            col_params.insert(col_id_end, col_value[1].clone());
                        } else if col_value.len().clone() == 1 {
                            col_sql =
                                format!("{} NOT BETWEEN #{{{}}} ", col_sql, col_id_start.clone());
                            col_params.insert(col_id_start, col_value[0].clone());
                        } else {
                            col_sql = format!("{} NOT BETWEEN  ", col_sql);
                        }
                    }
                } else {
                    if !ignore {
                        col_sql = format!("{} IS NOT NULL", col_sql);
                    }
                }
            }
        }
        (col_sql, col_params)
    }

    fn build_filter_column_item(
        filter_column: &crate::RdbcColumnFilterItem,
    ) -> (String, HashMap<String, RdbcValue>) {
        let (filter_sql, mut filter_params) =
            PgScriptColumnBuilder::build_column(filter_column.get_column());
        let compare = filter_column.get_compare().name();
        if let Some(v) = filter_column.get_value() {
            let (value_sql, value_params) = PgScriptColumnBuilder::build_column(v);
            filter_params.extend(value_params);
            (
                format!("{} {} {}", filter_sql, compare, value_sql),
                filter_params,
            )
        } else {
            (format!("{} {} {}", filter_sql, compare, ""), filter_params)
        }
    }

    fn build_filter_table_item(filter_value: &QueryFilter) -> (String, HashMap<String, RdbcValue>) {
        let (filter_sql, filter_params) = Self::build_filter(Some(filter_value));
        (format!("({})", filter_sql), filter_params)
    }

    fn build_filter_query_item(
        query_filter: &crate::RdbcQueryFilterItem,
    ) -> (String, HashMap<String, RdbcValue>) {
        let (filter_sql, mut filter_params) =
            PgScriptColumnBuilder::build_column(query_filter.get_column());
        let compare = query_filter.get_compare().name();
        if let Some(query) = query_filter.get_value() {
            let (query_sql, query_params) = PgScriptBuilder::build_query(query);
            filter_params.extend(query_params);
            (
                format!("{} {} ({})", filter_sql, compare, query_sql),
                filter_params,
            )
        } else {
            (format!("{} {} {}", filter_sql, compare, ""), filter_params)
        }
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
        let (column_sql, column_params) = PgScriptColumnBuilder::build_table_column(column);
        let old_value = func_column.get_old_value();
        let new_value = func_column.get_new_value();
        (
            format!("REPLACE({},'{}','{}')", column_sql, old_value, new_value),
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
    fn build_having(having: Option<&QueryFilter>) -> (String, HashMap<String, RdbcValue>) {
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

struct PgRawValueBuilder;
impl PgRawValueBuilder {
    pub fn raw_value(value: &RdbcValue) -> String {
        let value_index = match value {
            RdbcValue::Int(v) => v.to_string(),
            RdbcValue::BigInt(v) => v.to_string(),
            RdbcValue::Float(v) => v.to_string(),
            RdbcValue::BigFloat(v) => v.to_string(),
            RdbcValue::String(v) => format!("'{}'", v),
            RdbcValue::DateTime(v) => v.to_string(),
            RdbcValue::Bool(v) => v.to_string(),
            RdbcValue::Vec(v) => {
                let mut v_vec: Vec<String> = vec![];
                for item in v {
                    v_vec.push(PgRawValueBuilder::raw_value(item));
                }
                v_vec.join(",")
            }
            RdbcValue::Map(v) => {
                let mut v_vec: Vec<String> = vec![];
                for item in v.values() {
                    v_vec.push(PgRawValueBuilder::raw_value(item));
                }
                v_vec.join(",")
            }
            RdbcValue::Null => " NULL ".to_string(),
        };
        value_index
    }
}
