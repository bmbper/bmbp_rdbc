use std::collections::HashMap;
use bmbp_rdbc_type::RdbcValue;
use crate::dmq::render::RdbcSqlRender;
use crate::RdbcDelete;

pub struct RdbcDeleteRender {
    delete: RdbcDelete
}
impl RdbcSqlRender for RdbcDeleteRender {
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