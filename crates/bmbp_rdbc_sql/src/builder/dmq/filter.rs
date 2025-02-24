use crate::{
    RdbcColumn, RdbcCompare, RdbcFilterType, RdbcFilterValue, RdbcQuery, RdbcSimpleColumn,
    RdbcValueColumn, RdbcWhereCondition, RdbcWhereFilter, RdbcWhereNestCondition,
    RdbcWhereRawCondition, RdbcWhereSimpleCondition,
};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue};
use crate::RdbcCompare::{IsNotNull, IsNull};

pub trait RdbcWhereFilterBuilder {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter;
    fn filter_take(&mut self) -> Option<RdbcWhereFilter>;

    fn distinct(&mut self) -> &mut Self {
        self.filter_mut().distinct = true;
        self
    }

    fn simple_value<C, V>(&mut self, column: C, compare: RdbcCompare, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare,
            value: RdbcFilterValue::Value(RdbcValue::from(value)),
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }

    fn simple_col<C, V>(&mut self, column: C, compare: RdbcCompare, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare,
            value: RdbcFilterValue::Column(RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            })),
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }
    fn simple_query<C>(&mut self, column: C, compare: RdbcCompare, value: RdbcQuery) -> &mut Self
    where
        C: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare,
            value: RdbcFilterValue::Query(value),
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }
    fn simple_script<C, V>(&mut self, column: C, compare: RdbcCompare, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare,
            value: RdbcFilterValue::Value(RdbcValue::Varchar(value.name())),
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }

    fn simple_raw<C, V>(&mut self, column: C, compare: RdbcCompare, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare,
            value: RdbcFilterValue::Raw(value.name()),
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }
    fn eq_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::EQ, value)
    }
    fn eq_col<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_col(column, RdbcCompare::EQ, value)
    }

    fn eq_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::EQ, value)
    }
    fn eq_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::EQ, value)
    }

    fn ne_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::NE, value)
    }
    fn ne_col<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_col(column, RdbcCompare::NE, value)
    }

    fn ne_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::NE, value)
    }
    fn ne_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::NE, value)
    }

    fn ge_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::GE, value)
    }
    fn ge_col<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_col(column, RdbcCompare::GE, value)
    }

    fn ge_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::GE, value)
    }
    fn ge_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::GE, value)
    }

    fn gt_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::GT, value)
    }
    fn gt_col<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_col(column, RdbcCompare::GT, value)
    }

    fn gt_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::GT, value)
    }
    fn gt_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::GT, value)
    }
    fn le_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::LE, value)
    }
    fn le_col<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_col(column, RdbcCompare::LE, value)
    }

    fn le_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::GE, value)
    }
    fn le_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::GE, value)
    }

    fn lt_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::GT, value)
    }
    fn lt_col<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_col(column, RdbcCompare::GT, value)
    }

    fn lt_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::GT, value)
    }
    fn lt_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::GT, value)
    }

    fn like_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::Like, value)
    }

    fn like_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::Like, value)
    }

    fn like_left_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::LikeLeft, value)
    }

    fn like_left_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::LikeLeft, value)
    }
    fn like_right_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::LikeRight, value)
    }

    fn like_right_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::LikeRight, value)
    }

    fn not_like_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::NotLike, value)
    }
    fn not_like_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::NotLike, value)
    }

    fn not_like_left_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::NotLikeLeft, value)
    }

    fn not_like_left_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::NotLikeLeft, value)
    }
    fn not_like_right_v<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
    {
        self.simple_value(column, RdbcCompare::NotLikeRight, value)
    }

    fn not_like_right_script<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_script(column, RdbcCompare::NotLikeRight, value)
    }

    fn in_v<C, V>(&mut self, column: C, value: Vec<V>) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<Vec<V>>,
    {
        self.simple_value(column, RdbcCompare::IN, value)
    }

    fn in_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::IN, value.name())
    }

    fn in_query<C>(&mut self, column: C, value: RdbcQuery) -> &mut Self
    where
        C: RdbcIdent,
    {
        self.simple_query(column, RdbcCompare::IN, value)
    }
    fn not_in_v<C, V>(&mut self, column: C, value: Vec<V>) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<Vec<V>>,
    {
        self.simple_value(column, RdbcCompare::NotIn, value)
    }
    fn not_in_query<C>(&mut self, column: C, value: RdbcQuery) -> &mut Self
    where
        C: RdbcIdent,
    {
        self.simple_query(column, RdbcCompare::NotIn, value)
    }

    fn not_in_raw<C, V>(&mut self, column: C, value: V) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        self.simple_raw(column, RdbcCompare::NotIn, value.name())
    }

    fn exists<C>(&mut self, column: C, value: RdbcQuery) -> &mut Self
    where
        C: RdbcIdent,
    {
        self.simple_query(column, RdbcCompare::Exists, value)
    }
    fn not_exists<C>(&mut self, column: C, value: RdbcQuery) -> &mut Self
    where
        C: RdbcIdent,
    {
        self.simple_query(column, RdbcCompare::Exists, value)
    }

    fn null<C, V>(&mut self, column: C) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare:IsNull,
            value: RdbcFilterValue::None,
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }
    fn not_null<C, V>(&mut self, column: C) -> &mut Self
    where
        C: RdbcIdent,
        V: RdbcIdent,
    {
        let simple = RdbcWhereSimpleCondition {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            compare:IsNotNull,
            value: RdbcFilterValue::None,
        };
        let where_condition = RdbcWhereCondition::Simple(simple);
        self.filter_mut().conditions.push(where_condition);
        self
    }

    fn between_v<C, V, E>(&mut self, column: C, start: V, end: E) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
        RdbcValue: From<E>,
    {
        let rdbc_value = RdbcValue::Array(vec![RdbcValue::from(start), RdbcValue::from(end)]);
        self.simple_value(column, RdbcCompare::Between, rdbc_value)
    }
    fn not_between_v<C, V, E>(&mut self, column: C, start: V, end: V) -> &mut Self
    where
        C: RdbcIdent,
        RdbcValue: From<V>,
        RdbcValue: From<E>,
    {
        let rdbc_value = RdbcValue::Array(vec![RdbcValue::from(start), RdbcValue::from(end)]);
        self.simple_value(column, RdbcCompare::NotBetween, rdbc_value)
    }

    fn nest(&mut self, filter: RdbcWhereFilter) -> &mut Self {
        let simple_filter = RdbcWhereCondition::Nest(RdbcWhereNestCondition { condition: filter });
        self.filter_mut().conditions.push(simple_filter);
        self
    }

    fn or(&mut self) -> &mut Self {
        let filter_take = self.filter_take();
        self.filter_mut().type_ = RdbcFilterType::Or;
        if let Some(filter) = filter_take {
            self.filter_mut()
                .conditions
                .push(RdbcWhereCondition::Nest(RdbcWhereNestCondition {
                    condition: filter,
                }));
        }
        self
    }
    fn and(&mut self) -> &mut Self {
        let filter_take = self.filter_take();
        self.filter_mut().type_ = RdbcFilterType::And;
        if let Some(filter) = filter_take {
            self.filter_mut()
                .conditions
                .push(RdbcWhereCondition::Nest(RdbcWhereNestCondition {
                    condition: filter,
                }));
        }
        self
    }
    fn or_nest(&mut self, filter: RdbcWhereFilter) -> &mut Self {
        self.or().nest(filter)
    }
    fn and_nest(&mut self, filter: RdbcWhereFilter) -> &mut Self {
        self.and().nest(filter)
    }
    fn and_nest_builder(&mut self, builder: fn() -> RdbcWhereFilter) -> &mut Self {
        self.and().nest(builder())
    }
    fn or_nest_builder(&mut self, builder: fn() -> RdbcWhereFilter) -> &mut Self {
        self.or().nest(builder())
    }
    fn nest_builder(&mut self, builder: fn() -> RdbcWhereFilter) -> &mut Self {
        self.nest(builder())
    }
    fn raw<C>(&mut self, condition: C) -> &mut Self
    where
        C: RdbcIdent,
    {
        let simple = RdbcWhereCondition::Raw(RdbcWhereRawCondition {
            condition: condition.name(),
        });
        self.filter_mut().conditions.push(simple);
        self
    }
}
