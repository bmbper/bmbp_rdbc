use crate::{DeleteWrapper, InsertWrapper, QueryWrapper, UpdateWrapper};
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;

pub trait RdbcSQLBuilder {
    fn build_query_script(_query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_insert_script(
        _insert_wrapper: &InsertWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_update_script(
        _update_wrapper: &UpdateWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_delete_script(
        _delete_wrapper: &DeleteWrapper,
    ) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_query_sql(_query_wrapper: &QueryWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_insert_sql(_insert_wrapper: &InsertWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_update_sql(_update_wrapper: &UpdateWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_delete_sql(_delete_wrapper: &DeleteWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_raw_query(_query_wrapper: &QueryWrapper) -> String {
        "".to_string()
    }
    fn build_raw_insert(_insert_wrapper: &InsertWrapper) -> String {
        "".to_string()
    }
    fn build_raw_update(_update_wrapper: &UpdateWrapper) -> String {
        "".to_string()
    }
    fn build_raw_delete(_delete_wrapper: &DeleteWrapper) -> String {
        "".to_string()
    }
}
