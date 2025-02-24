use crate::filter::RdbcWhereFilterBuilder;
use crate::{RdbcFilterType, RdbcHaving, RdbcWhereFilter};

pub trait RdbcHavingBuilder{
    fn having_mut(&mut self) -> &mut RdbcHaving;
}


impl RdbcWhereFilterBuilder for RdbcHaving{
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.filter.get_or_insert(RdbcWhereFilter{
            type_: RdbcFilterType::And,
            conditions: vec![],
            distinct: false,
        })
    }
    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.filter.take()
    }
}
