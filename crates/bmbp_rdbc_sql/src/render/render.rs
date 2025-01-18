use std::collections::HashMap;
use bmbp_rdbc_type::RdbcValue;

pub trait RdbcSqlRender{
    fn render_script(&self) -> (String, HashMap<String, RdbcValue>);
    fn render_sql(&self) -> (String, Vec<RdbcValue>);
    fn render_raw(&self) -> String;
}