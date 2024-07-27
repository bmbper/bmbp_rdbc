use std::collections::HashMap;

use crate::build::base::base_build_sql;
use crate::build::vars::MYSQL_PARAMS_TAG;
use crate::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcValue, UpdateWrapper};

pub fn mysql_build_sql(
    sql: String,
    params: HashMap<String, RdbcValue>,
) -> (String, Vec<RdbcValue>) {
    base_build_sql(MYSQL_PARAMS_TAG, sql, params)
}
pub fn mysql_build_query_script(_query: &QueryWrapper) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn mysql_build_insert_script(_insert: &InsertWrapper) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn mysql_build_update_script(_update: &UpdateWrapper) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
pub fn mysql_build_delete_script(_delete: &DeleteWrapper) -> (String, HashMap<String, RdbcValue>) {
    ("".to_string(), HashMap::new())
}
