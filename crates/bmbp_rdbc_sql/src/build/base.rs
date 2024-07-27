use std::collections::HashMap;

use crate::RdbcValue;

pub(crate) fn base_build_sql(
    tag: &str,
    sql: String,
    params: HashMap<String, RdbcValue>,
) -> (String, Vec<RdbcValue>) {
    let (mut pg_sql, mut page_params) = (sql.clone(), vec![]);
    for (index, (key, value)) in params.iter().enumerate() {
        let old = format!("${{{}}}", key);
        let new_ = format!("{}{}", tag, index + 1);
        pg_sql = pg_sql.replace(old.as_str(), new_.as_str());
        page_params.push(value.clone());
    }
    (pg_sql, page_params)
}
