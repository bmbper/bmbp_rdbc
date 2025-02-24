use crate::{
    RdbcColumn, RdbcFuncColumn, RdbcGroupColumn, RdbcQuery, RdbcQueryColumn, RdbcRawColumn,
    RdbcSelectColumn, RdbcSimpleColumn, RdbcTableColumn, RdbcValueColumn,
};
use bmbp_rdbc_type::{RdbcIdent, RdbcValue, RdbcValueIdent};

pub trait RdbcGroupBuilder {
    fn group_mut(&mut self) -> &mut Vec<RdbcGroupColumn>;
    fn group_by<T>(&mut self, column: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                column: column.name(),
            }),
        });
        self
    }
    fn group_by_vec<T>(&mut self, column_vec: Vec<T>) -> &mut Self
    where
        T: RdbcIdent,
    {
        for column in column_vec.iter() {
            self.group_mut().push(RdbcGroupColumn {
                column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                    column: column.name(),
                }),
            });
        }
        self
    }

    fn group_by_slice<T>(&mut self, column_vec: &[T]) -> &mut Self
    where
        T: RdbcIdent,
    {
        for column in column_vec.iter() {
            self.group_mut().push(RdbcGroupColumn {
                column: RdbcColumn::SimpleColumn(RdbcSimpleColumn {
                    column: column.name(),
                }),
            });
        }
        self
    }

    fn group_by_query(&mut self, query: RdbcQuery) -> &mut Self {
        self.group_by_query_column(RdbcQueryColumn { column: query })
    }

    fn group_by_raw<T>(&mut self, value: T) -> &mut Self
    where
        T: RdbcValueIdent,
    {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::RawColumn(RdbcRawColumn {
                column: value.value(),
            }),
        });
        self
    }

    fn group_by_value<T>(&mut self, value: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.group_by_rdbc_value(RdbcValue::Varchar(value.name()));
        self
    }

    fn group_by_rdbc_value(&mut self, value: RdbcValue) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::ValueColumn(RdbcValueColumn { column: value }),
        });
        self
    }

    fn group_by_rdbc_column(&mut self, column: RdbcColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn { column });
        self
    }

    fn group_by_simple_column(&mut self, column: RdbcSimpleColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::SimpleColumn(column),
        });
        self
    }

    fn group_by_table_column(&mut self, column: RdbcTableColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::TableColumn(column),
        });
        self
    }

    fn group_by_query_column(&mut self, column: RdbcQueryColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::QueryColumn(column),
        });
        self
    }

    fn group_by_value_column(&mut self, column: RdbcValueColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::ValueColumn(column),
        });
        self
    }

    fn group_by_func_column(&mut self, column: RdbcFuncColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::FuncColumn(column),
        });
        self
    }

    fn group_by_raw_column(&mut self, column: RdbcRawColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::RawColumn(column),
        });
        self
    }

    fn group_by_column(&mut self, column: RdbcGroupColumn) -> &mut Self {
        self.group_mut().push(column);
        self
    }

    fn group_by_func(&mut self, func: RdbcFuncColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: RdbcColumn::FuncColumn(func),
        });
        self
    }
    fn group_by_select_column(&mut self, column: RdbcSelectColumn) -> &mut Self {
        self.group_mut().push(RdbcGroupColumn {
            column: column.column,
        });
        self
    }
}
