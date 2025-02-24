use std::collections::HashMap;
use bmbp_rdbc_type::RdbcValue;

pub trait RdbcSqlRender{
    fn render_script(&self) -> (String, HashMap<String, RdbcValue>) {
        ("".to_string(), HashMap::new())
    }

    fn render_sql(&self) -> (String, Vec<RdbcValue>) {
        ("".to_string(), vec![])
    }

    fn render_raw(&self) -> String {
        "".to_string()
    }
}