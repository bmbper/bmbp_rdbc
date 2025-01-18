use std::collections::HashMap;
use bmbp_rdbc_type::RdbcValue;
use crate::render::render::RdbcSqlRender;
use crate::types::RdbcQuery;

pub struct RdbcQueryRender{
    query: RdbcQuery
}
impl RdbcSqlRender for RdbcQueryRender{
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