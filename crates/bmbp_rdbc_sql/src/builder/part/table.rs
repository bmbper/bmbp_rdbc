use bmbp_rdbc_type::RdbcIdent;
use crate::{
       RdbcQuery, RdbcQueryTable, RdbcRawTable, RdbcSimpleTable,
    RdbcTable,
};
pub trait RdbcTableBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable>;

    fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        let table = RdbcTable::from(table.name());
        self.table_mut().push(table);
        self
    }

    fn table_slice<T>(&mut self, table: &[T]) -> &mut Self
    where
        T: RdbcIdent,
    {
        for item in table {
            self.table(item.name());
        }
        self
    }

    fn table_as<T, A>(&mut self, table: T, alias: A) -> &mut Self
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcTable::from((table.name(), alias.name()));
        self.table_mut().push(table);
        self
    }
    fn schema_table<S, T>(&mut self, schema: S, table: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcTable::new_with_schema_table(schema.name(), table.name());
        self.table_mut().push(table);
        self
    }
    fn schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcTable::from((schema.name(), table.name(), alias.name()));
        self.table_mut().push(table);
        self
    }

    fn raw_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        let table = RdbcTable::RawTable(RdbcRawTable::from(table.name()));
        self.table_mut().push(table);
        self
    }
    fn raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcTable::RawTable(RdbcRawTable::from((table.name(), alias.name())));
        self.table_mut().push(table);
        self
    }

    fn table_query(&mut self, query: RdbcQuery) -> &mut Self {
        let table = RdbcTable::from(query);
        self.table_mut().push(table);
        self
    }
    fn table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        let table = RdbcTable::from((query, alias.name()));
        self.table_mut().push(table);
        self
    }
    fn rdbc_table(&mut self, table: RdbcTable) -> &mut Self {
        self.table_mut().push(table);
        self
    }
    fn rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut Self {
        self.table_mut().push(RdbcTable::SimpleTable(table));
        self
    }
    fn rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut Self {
        self.table_mut().push(RdbcTable::QueryTable(query));
        self
    }
    fn rdbc_raw_table(&mut self, query: RdbcRawTable) -> &mut Self {
        self.table_mut().push(RdbcTable::RawTable(query));
        self
    }
}
