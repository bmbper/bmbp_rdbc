/// RdbcIdent get name which use for table name,column name,schema name
pub trait RdbcIdent {
    fn get_ident(&self) -> String;
}

impl<T> RdbcIdent for T where T: ToString {
    fn get_ident(&self) -> String {
        return self.to_string();
    }
}

/// RdbcTable which use for declare a struct to  table
pub trait RdbcTable {
    fn get_table() -> impl RdbcIdent;
    fn get_columns() -> Vec<impl RdbcIdent>;
}