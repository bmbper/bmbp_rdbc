use std::collections::HashMap;
use bmbp_rdbc_type::RdbcValue;
use crate::render::render::RdbcSqlRender;
use crate::types::RdbcInsert;

pub struct RdbcInsertRender {
    insert: RdbcInsert
}
impl RdbcSqlRender for RdbcInsertRender {
    fn render_script(&self) -> (String, HashMap<String, RdbcValue>) {
        todo!()
    }

    fn render_sql(&self) -> (String, Vec<RdbcValue>) {
        todo!()
    }

    fn render_raw(&self) -> String {
        todo!()
    }
}