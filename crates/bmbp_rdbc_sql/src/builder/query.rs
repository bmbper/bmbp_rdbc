use crate::builder::part::{RdbcJoinTableBuilder, RdbcTableBuilder, RdbcWhereFilterBuilder};
use crate::types::{RdbcQuery, RdbcUpdate};

pub struct RdbcQueryBuilder {
    query: RdbcQuery,
}
impl RdbcQueryBuilder{
    pub fn build(&self) -> &RdbcQuery {
        &self.query
    }
}
impl RdbcTableBuilder for RdbcQueryBuilder {

}
impl RdbcJoinTableBuilder for RdbcQueryBuilder {

}
impl RdbcWhereFilterBuilder for RdbcQueryBuilder {

}