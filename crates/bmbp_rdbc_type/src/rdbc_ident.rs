pub trait RdbcTableTrait {
    fn table_name(&self) -> String;
    fn table_columns(&self) -> Vec<String>;
    fn table_primary_key(&self) -> String;
}

pub trait RdbcColumnTrait {
    fn table_name(&self) -> String;
    fn column_name(&self) -> String;
}
