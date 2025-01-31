use crate::{
    RdbcColumn, RdbcCompare, RdbcFilterValue, RdbcQuery, RdbcWhereCondition, RdbcWhereFilter,
    RdbcWhereNestCondition, RdbcWhereRawCondition, RdbcWhereSimpleCondition,
};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};

pub struct RdbcWhereConditionBuilder;

impl RdbcWhereConditionBuilder {
    pub(crate) fn new_simple_value<C, V>(
        column: C,
        compare: RdbcCompare,
        value: V,
    ) -> RdbcWhereCondition
    where
        RdbcColumn: From<C>,
        RdbcValue: From<V>,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::from(column),
            compare,
            value: RdbcFilterValue::Value(RdbcValue::from(value)),
        };
        let condition = RdbcWhereCondition::Simple(simple);
        return condition;
    }
    pub(crate) fn new_simple_column<C, V>(
        column: C,
        compare: RdbcCompare,
        value: V,
    ) -> RdbcWhereCondition
    where
        RdbcColumn: From<C>,
        RdbcColumn: From<V>,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::from(column),
            compare,
            value: RdbcFilterValue::Column(RdbcColumn::from(value)),
        };
        let condition = RdbcWhereCondition::Simple(simple);
        return condition;
    }
    pub(crate) fn new_simple_query<C>(
        column: C,
        compare: RdbcCompare,
        value: RdbcQuery,
    ) -> RdbcWhereCondition
    where
        RdbcColumn: From<C>,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::from(column),
            compare,
            value: RdbcFilterValue::Query(value),
        };
        let condition = RdbcWhereCondition::Simple(simple);
        return condition;
    }
    pub(crate) fn new_simple_script<C, V>(
        column: C,
        compare: RdbcCompare,
        value: V,
    ) -> RdbcWhereCondition
    where
        RdbcColumn: From<C>,
        V: RdbcIdent,
    {
        let script_value = value.name();
        let script = {
            if script_value.starts_with("#{") && script_value.ends_with("}") {
                script_value
            } else {
                format!("#{{{}}}", script_value)
            }
        };

        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::from(column),
            compare,
            value: RdbcFilterValue::Script(script),
        };
        let condition = RdbcWhereCondition::Simple(simple);
        return condition;
    }

    pub(crate) fn new_simple_raw<C, V>(
        column: C,
        compare: RdbcCompare,
        value: V,
    ) -> RdbcWhereCondition
    where
        RdbcColumn: From<C>,
        V: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::from(column),
            compare,
            value: RdbcFilterValue::Raw(value.name()),
        };
        let condition = RdbcWhereCondition::Simple(simple);
        return condition;
    }

    pub(crate) fn raw<C>(condition: C) -> RdbcWhereCondition
    where
        C: RdbcIdent,
    {
        let simple = RdbcWhereRawCondition {
            condition: condition.name(),
        };
        let condition = RdbcWhereCondition::Raw(simple);
        return condition;
    }
    pub(crate) fn nest(filter: RdbcWhereFilter) -> RdbcWhereCondition {
        let filter = RdbcWhereCondition::Nest(RdbcWhereNestCondition { condition: filter });
        return filter;
    }
}
