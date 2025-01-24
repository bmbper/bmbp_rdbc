use crate::RdbcWhereFilter;

pub trait RdbcWhereFilterBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter;
}
