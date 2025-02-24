use crate::builder::dmq::filter::RdbcWhereFilterBuilder;
use crate::builder::dmq::group::RdbcGroupBuilder;
use crate::builder::dmq::having::RdbcHavingBuilder;
use crate::builder::dmq::limit::{RdbcLimitBuilder, RdbcOffsetBuilder};
use crate::builder::dmq::order::RdbcOrderBuilder;
use crate::builder::dmq::select::RdbcSelectBuilder;
use crate::builder::dmq::table::{RdbcJoinTableBuilder, RdbcTableBuilder};
use crate::builder::dmq::union::{RdbcUnionAllBuilder, RdbcUnionBuilder};
use crate::{
    RdbcFilterType, RdbcGroupColumn, RdbcHaving, RdbcJoinTable, RdbcOrderColumn, RdbcQuery,
    RdbcSelectColumn, RdbcTable, RdbcWhereFilter,
};

pub struct RdbcQueryBuilder {
    query: RdbcQuery,
}

impl RdbcQueryBuilder {
    pub fn new() -> Self {
        RdbcQueryBuilder {
            query: RdbcQuery {
                select: vec![],
                table: vec![],
                join_table: vec![],
                where_: None,
                order_by: vec![],
                group_by: vec![],
                having: None,
                limit: None,
                offset: None,
                union: vec![],
                union_all: vec![],
            },
        }
    }
}

impl RdbcSelectBuilder for RdbcQueryBuilder {
    fn select_mut(&mut self) -> &mut Vec<RdbcSelectColumn> {
        self.query.select.as_mut()
    }
}

impl RdbcTableBuilder for RdbcQueryBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable> {
        self.query.table.as_mut()
    }
}

impl RdbcJoinTableBuilder for RdbcQueryBuilder {
    fn table_join_mut(&mut self) -> &mut Vec<RdbcJoinTable> {
        self.query.join_table.as_mut()
    }
}

impl RdbcWhereFilterBuilder for RdbcQueryBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.query.where_.get_or_insert(RdbcWhereFilter {
            type_: RdbcFilterType::And,
            conditions: vec![],
            distinct: false,
        })
    }
    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.query.where_.take()
    }
}

impl RdbcOrderBuilder for RdbcQueryBuilder {
    fn order_mut(&mut self) -> &mut Vec<RdbcOrderColumn> {
        self.query.order_by.as_mut()
    }
}

impl RdbcGroupBuilder for RdbcQueryBuilder {
    fn group_mut(&mut self) -> &mut Vec<RdbcGroupColumn> {
        self.query.group_by.as_mut()
    }
}

impl RdbcHavingBuilder for RdbcQueryBuilder {
    fn having_mut(&mut self) -> &mut RdbcHaving {
        self.query.having.get_or_insert(RdbcHaving {
            filter: Some(RdbcWhereFilter {
                type_: RdbcFilterType::And,
                conditions: vec![],
                distinct: false,
            }),
        })
    }
}
impl RdbcLimitBuilder for RdbcQueryBuilder {
    fn limit(&mut self, limit: u64) -> &mut Self {
        self.query.limit = Some(limit);
        self
    }
}
impl RdbcOffsetBuilder for RdbcQueryBuilder {
    fn offset(&mut self, limit: u64) -> &mut Self {
        self.query.offset = Some(limit);
        self
    }
}
impl RdbcUnionBuilder for RdbcQueryBuilder {
    fn union(&mut self, query: RdbcQuery) -> &mut Self {
        self.query.union.push(query);
        self
    }
}
impl RdbcUnionAllBuilder for RdbcQueryBuilder {
    fn union_all(&mut self, query: RdbcQuery) -> &mut Self {
        self.query.union_all.push(query);
        self
    }
}
