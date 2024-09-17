/// RdbcIdent get name which use for table name,column name,schema name
pub trait RdbcIdent {
    fn get_ident(&self) -> String;
    fn with_alias(&self, alias: &str) -> String {
        format!("{}.{}", alias, self.get_ident())
    }
    fn with_as_alias(&self, with_alias: &str, as_alias: bool) -> String {
        format!("{}.{} AS {}", with_alias, self.get_ident(), as_alias)
    }
    fn as_alias(&self, alias: &str) -> String {
        format!("{} AS {}", self.get_ident(), alias)
    }
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
    fn get_primary_key() -> impl RdbcIdent {
        "".to_string()
    }
    fn get_union_key() -> Vec<impl RdbcIdent> {
        let v:Vec<String> = vec![];
        v
    }
}