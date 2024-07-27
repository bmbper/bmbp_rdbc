use crate::{RdbcColumn, RdbcFuncColumn, RdbcTableColumn, RdbcValue};

pub enum RdbcFunc {
    CONCAT(RdbcConcatFunc),
    REPLACE(RdbcReplaceFunc),
}

impl RdbcFunc {
    pub fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcFunc {
        RdbcFunc::CONCAT(RdbcConcatFunc::concat_split(columns, split_))
    }
    pub fn concat(columns: Vec<RdbcColumn>) -> RdbcFunc {
        RdbcFunc::CONCAT(RdbcConcatFunc::concat(columns))
    }
    pub fn replace(column: RdbcTableColumn, old_value: String, new_value: String) -> RdbcFunc {
        RdbcFunc::REPLACE(RdbcReplaceFunc::replace(column, old_value, new_value))
    }
}

pub struct RdbcConcatFunc {
    liter_: Option<String>,
    columns_: Vec<RdbcColumn>,
}

impl RdbcConcatFunc {
    pub fn get_liter(&self) -> Option<&String> {
        self.liter_.as_ref()
    }
    pub fn get_columns(&self) -> &Vec<RdbcColumn> {
        &self.columns_
    }
}

impl RdbcConcatFunc {
    pub fn concat_split(columns: Vec<RdbcColumn>, split_: Option<String>) -> RdbcConcatFunc {
        RdbcConcatFunc {
            liter_: split_,
            columns_: columns,
        }
    }
    pub fn concat(columns: Vec<RdbcColumn>) -> RdbcConcatFunc {
        RdbcConcatFunc {
            liter_: None,
            columns_: columns,
        }
    }
}

pub struct RdbcReplaceFunc {
    column: RdbcTableColumn,
    old_value: String,
    new_value: String,
}

impl RdbcReplaceFunc {
    pub fn get_column(&self) -> &RdbcTableColumn {
        &self.column
    }
    pub fn get_old_value(&self) -> &String {
        &self.old_value
    }
    pub fn get_new_value(&self) -> &String {
        &self.new_value
    }
}

impl RdbcReplaceFunc {
    pub fn replace(
        column: RdbcTableColumn,
        old_value: String,
        new_value: String,
    ) -> RdbcReplaceFunc {
        RdbcReplaceFunc {
            column,
            old_value,
            new_value,
        }
    }
}
