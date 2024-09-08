use std::collections::HashMap;
use bmbp_rdbc_type::RdbcValue;
use crate::{DeleteWrapper, InsertWrapper, QueryWrapper, UpdateWrapper};

pub trait RdbcSQLBuilder {
    fn build_query_script(&self, query_wrapper: QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_insert_script(&self, insert_wrapper: InsertWrapper) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_update_script(&self, update_wrapper: UpdateWrapper) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_delete_script(&self, delete_wrapper: DeleteWrapper) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }
    fn build_query_sql(&self, query_wrapper: QueryWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_insert_sql(&self, insert_wrapper: InsertWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_update_sql(&self, update_wrapper: UpdateWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_delete_sql(&self, delete_wrapper: DeleteWrapper) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }
    fn build_raw_query(&self, query_wrapper: QueryWrapper) -> String {
        "".to_string()
    }
    fn build_raw_insert(&self, insert_wrapper: InsertWrapper) -> String {
        "".to_string()
    }
    fn build_raw_update(&self, update_wrapper: UpdateWrapper) -> String {
        "".to_string()
    }
    fn build_raw_delete(&self, delete_wrapper: DeleteWrapper) -> String {
        "".to_string()
    }
}
