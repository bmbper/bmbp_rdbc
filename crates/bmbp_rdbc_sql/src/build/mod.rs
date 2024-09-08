use std::collections::HashMap;
use bmbp_rdbc_type::{RdbcValue};
pub use mysql::*;
pub use pg::*;
pub use crate::build::types::{RdbcSQLBuilder};
use crate::{DeleteWrapper, InsertWrapper, QueryWrapper, UpdateWrapper};

mod mysql;
mod pg;
mod types;

pub struct SQLBuilder {}

impl SQLBuilder {
    pub fn build_query_script<T>(query_wrapper: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) where T: RdbcSQLBuilder {
        T::build_query_script(query_wrapper)
    }
    pub fn build_insert_script<T>(&self, insert_wrapper: &InsertWrapper) -> (String, HashMap<String, RdbcValue>) where T: RdbcSQLBuilder {
        T::build_insert_script(insert_wrapper)
    }
    pub fn build_update_script<T>(&self, update_wrapper: &UpdateWrapper) -> (String, HashMap<String, RdbcValue>) where T: RdbcSQLBuilder {
        T::build_update_script(update_wrapper)
    }
    pub fn build_delete_script<T>(&self, delete_wrapper: &DeleteWrapper) -> (String, HashMap<String, RdbcValue>) where T: RdbcSQLBuilder {
        T::build_delete_script(delete_wrapper)
    }
    pub fn build_query_sql<T>(&self, query_wrapper: &QueryWrapper) -> (String, Vec<RdbcValue>) where T: RdbcSQLBuilder {
        T::build_query_sql(query_wrapper)
    }
    pub fn build_insert_sql<T>(&self, insert_wrapper: &InsertWrapper) -> (String, Vec<RdbcValue>) where T: RdbcSQLBuilder {
        T::build_insert_sql(insert_wrapper)
    }
    pub fn build_update_sql<T>(&self, update_wrapper: &UpdateWrapper) -> (String, Vec<RdbcValue>) where T: RdbcSQLBuilder {
        T::build_update_sql(update_wrapper)
    }
    pub fn build_delete_sql<T>(&self, delete_wrapper: &DeleteWrapper) -> (String, Vec<RdbcValue>) where T: RdbcSQLBuilder {
        T::build_delete_sql(delete_wrapper)
    }
    pub fn build_raw_query<T>(&self, query_wrapper: &QueryWrapper) -> String where T: RdbcSQLBuilder {
        T::build_raw_query(query_wrapper)
    }
    pub fn build_raw_insert<T>(&self, insert_wrapper: &InsertWrapper) -> String where T: RdbcSQLBuilder {
        T::build_raw_insert(insert_wrapper)
    }
    pub fn build_raw_update<T>(&self, update_wrapper: &UpdateWrapper) -> String where T: RdbcSQLBuilder {
        T::build_raw_update(update_wrapper)
    }
    pub fn build_raw_delete<T>(&self, delete_wrapper: &DeleteWrapper) -> String where T: RdbcSQLBuilder {
        T::build_raw_delete(delete_wrapper)
    }
}

