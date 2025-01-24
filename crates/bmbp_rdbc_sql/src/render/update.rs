use crate::render::render::RdbcSqlRender;
use crate::types::RdbcUpdate;
use bmbp_rdbc_type::RdbcValue;
use std::collections::HashMap;

pub struct RdbcUpdateRender {
    update: RdbcUpdate,
}
impl RdbcSqlRender for RdbcUpdateRender {
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
