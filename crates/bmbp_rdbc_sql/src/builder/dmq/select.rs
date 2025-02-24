use crate::{
    RdbcColumn, RdbcFunc, RdbcFuncColumn, RdbcQuery, RdbcQueryColumn, RdbcRawColumn,
    RdbcSelectColumn, RdbcSimpleColumn, RdbcTableColumn, RdbcValueColumn,
};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue, RdbcValueIdent};

pub trait RdbcSelectBuilder {
    fn select_mut(&mut self) -> &mut Vec<RdbcSelectColumn>;
    fn select<T>(&mut self, column: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            alias: "".to_string(),
        });
        self
    }
    fn select_as<C, A>(&mut self, column: C, alias: A) -> &mut Self
    where
        C: RdbcIdent,
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
            alias: alias.name(),
        });
        self
    }
    fn select_vec<T>(&mut self, column_vec: Vec<T>) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.select_slice(column_vec.as_slice())
    }
    fn select_vec_as<C, A>(&mut self, column_vec: Vec<(C, A)>) -> &mut Self
    where
        C: RdbcIdent,
        A: RdbcIdent,
    {
        self.select_slice_as(column_vec.as_slice())
    }

    fn select_slice<T>(&mut self, column_vec: &[T]) -> &mut Self
    where
        T: RdbcIdent,
    {
        for column in column_vec.iter() {
            self.select_mut().push(RdbcSelectColumn {
                column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                    column: column.name(),
                }),
                alias: "".to_string(),
            });
        }
        self
    }
    fn select_slice_as<C, A>(&mut self, column_vec: &[(C, A)]) -> &mut Self
    where
        C: RdbcIdent,
        A: RdbcIdent,
    {
        for (column, alias) in column_vec.iter() {
            self.select_mut().push(RdbcSelectColumn {
                column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                    column: column.name(),
                }),
                alias: alias.name(),
            });
        }
        self
    }

    fn select_query(&mut self, query: RdbcQuery) -> &mut Self {
        self.select_query_column(RdbcQueryColumn { column: query })
    }

    fn select_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_query_column_as(RdbcQueryColumn { column: query }, alias)
    }
    fn select_raw<T>(&mut self, value: T) -> &mut Self
    where
        T: RdbcValueIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::RawColumn(RdbcRawColumn {
                column: value.value(),
            }),
            alias: "".to_string(),
        });
        self
    }

    fn select_raw_as<T, A>(&mut self, value: T, alias: A) -> &mut Self
    where
        T: RdbcValueIdent,
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::RawColumn(RdbcRawColumn {
                column: value.value(),
            }),
            alias: alias.name(),
        });
        self
    }

    fn select_value<T>(&mut self, value: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.select_rdbc_value(RdbcValue::Varchar(value.name()));
        self
    }

    fn select_value_as<T, A>(&mut self, value: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.select_rdbc_value_as(RdbcValue::Varchar(value.name()), alias.name());
        self
    }
    fn select_rdbc_value(&mut self, value: RdbcValue) -> &mut Self {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(RdbcValueColumn { column: value }),
            alias: "".to_string(),
        });
        self
    }

    fn select_rdbc_value_as<A>(&mut self, value: RdbcValue, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(RdbcValueColumn { column: value }),
            alias: alias.name(),
        });
        self
    }
    fn select_rdbc_column(&mut self, column: RdbcColumn) -> &mut Self {
        self.select_mut().push(RdbcSelectColumn {
            column,
            alias: "".to_string(),
        });
        self
    }

    fn select_rdbc_column_as<A>(&mut self, column: RdbcColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column,
            alias: alias.name(),
        });
        self
    }

    fn select_simple_column(&mut self, column: RdbcSimpleColumn) -> &mut Self {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(column),
            alias: "".to_string(),
        });
        self
    }

    fn select_simple_column_as<A>(&mut self, column: RdbcSimpleColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::SimpleColumn(column),
            alias: alias.name(),
        });
        self
    }
    fn select_table_column(&mut self, column: RdbcTableColumn) -> &mut Self {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::TableColumn(column),
            alias: "".to_string(),
        });
        self
    }

    fn select_table_column_as<A>(&mut self, column: RdbcTableColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::TableColumn(column),
            alias: alias.name(),
        });
        self
    }
    fn select_query_column(&mut self, column: RdbcQueryColumn) -> &mut Self {
        self.select_query_column_as(column, "")
    }

    fn select_query_column_as<A>(&mut self, column: RdbcQueryColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::QueryColumn(column),
            alias: alias.name(),
        });
        self
    }

    fn select_value_column(&mut self, column: RdbcValueColumn) -> &mut Self {
        self.select_value_column_as(column, "")
    }

    fn select_value_column_as<A>(&mut self, column: RdbcValueColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::ValueColumn(column),
            alias: alias.name(),
        });
        self
    }

    fn select_func_column(&mut self, column: RdbcFuncColumn) -> &mut Self {
        self.select_func_column_as(column, "")
    }

    fn select_func_column_as<A>(&mut self, column: RdbcFuncColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::FuncColumn(column),
            alias: alias.name(),
        });
        self
    }

    fn select_raw_column(&mut self, column: RdbcRawColumn) -> &mut Self {
        self.select_raw_column_as(column, "")
    }

    fn select_raw_column_as<A>(&mut self, column: RdbcRawColumn, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        self.select_mut().push(RdbcSelectColumn {
            column: RdbcColumn::RawColumn(column),
            alias: alias.name(),
        });
        self
    }

    fn select_select_column(&mut self, column: RdbcSelectColumn) -> &mut Self {
        self.select_mut().push(column);
        self
    }

    fn select_func(&mut self, func: RdbcFunc) -> &mut Self {
        self.select_func_column(RdbcFuncColumn { func })
    }
    fn select_func_as<T>(&mut self, func: RdbcFunc, alias: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.select_func_column_as(RdbcFuncColumn { func }, alias)
    }
}
