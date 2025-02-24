use crate::{RdbcColumn, RdbcOrderColumn, RdbcOrderType};

pub trait RdbcOrderBuilder{
    fn order_mut(&mut self) -> &mut Vec<RdbcOrderColumn>;
     fn order<C>(&mut self, column: C, order: RdbcOrderType) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        let rdbc_order_column = RdbcOrderColumn {
            column: RdbcColumn::from(column),
            order_type: order,
        };
        self.order_mut().push(rdbc_order_column);
        self
    }
    fn order_asc<C>(&mut self, column: C) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        self.order(column, RdbcOrderType::Asc)
    }
    fn order_vec_asc<C>(&mut self, column: Vec<C>) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        for item in column {
            self.order(item, RdbcOrderType::Asc);
        }
        self
    }

    fn order_desc<C>(&mut self, column: C) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        self.order(column, RdbcOrderType::Desc)
    }
    fn order_vec_desc<C>(&mut self, column: Vec<C>) -> &mut Self
    where
        RdbcColumn: From<C>,
    {
        for item in column {
            self.order(item, RdbcOrderType::Desc);
        }
        self
    }
}

