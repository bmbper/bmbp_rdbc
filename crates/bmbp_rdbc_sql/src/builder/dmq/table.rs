use bmbp_rdbc_type::RdbcIdent;

use crate::filter::RdbcWhereFilterBuilder;
use crate::JoinType::{FullJoin, InnerJoin, LeftJoin, RightJoin};
use crate::{
    JoinType, RdbcFilterType, RdbcJoinTable, RdbcQuery, RdbcQueryTable, RdbcRawTable
    , RdbcSimpleTable, RdbcTable, RdbcWhereFilter,
};

pub trait RdbcTableBuilder {
    fn table_mut(&mut self) -> &mut Vec<RdbcTable>;

    fn table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.table_as(table.name(), "")
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
        let table = RdbcTable::SimpleTable(RdbcSimpleTable {
            schema: "".to_string(),
            table: table.name(),
            alias: alias.name(),
        });
        self.table_mut().push(table);
        self
    }
    fn schema_table<S, T>(&mut self, schema: S, table: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.schema_table_as(schema, table, "")
    }
    fn schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcTable::from(RdbcTable::SimpleTable(RdbcSimpleTable {
            schema: schema.name(),
            table: table.name(),
            alias: alias.name(),
        }));
        self.table_mut().push(table);
        self
    }

    fn raw_table<T>(&mut self, table: T) -> &mut Self
    where
        T: RdbcIdent,
    {
        self.raw_table_as(table, "")
    }
    fn raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut Self
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcTable::RawTable(RdbcRawTable {
            table: table.name(),
            alias: alias.name(),
        });
        self.table_mut().push(table);
        self
    }

    fn table_query(&mut self, query: RdbcQuery) -> &mut Self {
        self.table_query_as(query, "")
    }
    fn table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut Self
    where
        A: RdbcIdent,
    {
        let table = RdbcTable::QueryTable(RdbcQueryTable {
            query,
            alias: alias.name(),
        });
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

pub trait RdbcJoinTableBuilder {
    fn table_join_mut(&mut self) -> &mut Vec<RdbcJoinTable>;
    fn join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.join_table_as(table, "")
    }

    fn join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.join_schema_table_as("", table, alias)
    }
    fn join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.join_schema_table_as(schema.name(), table.name(), "")
    }
    fn join_schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.with_join_schema_table_as(schema.name(), table.name(), alias.name(), InnerJoin)
    }

    fn join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.join_raw_table_as(table.name(), "")
    }
    fn join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.with_join_raw_table_as(table, alias, InnerJoin)
    }

    fn join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        self.join_table_query_as(query, "")
    }
    fn join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        self.with_join_table_query_as(query, alias.name(), InnerJoin)
    }
    fn join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
        self.with_join_rdbc_table(table, InnerJoin)
    }
    fn join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_simple_table(table, InnerJoin)
    }
    fn join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_query_table(query, InnerJoin)
    }
    fn join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
        self.with_join_rdbc_raw_table(table,InnerJoin)
    }

    fn left_join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.left_join_table_as(table, "")
    }

    fn left_join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.left_join_schema_table_as("", table, alias)
    }
    fn left_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.left_join_schema_table_as(schema, table, "")
    }
    fn left_join_schema_table_as<S, T, A>(
        &mut self,
        schema: S,
        table: T,
        alias: A,
    ) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.with_join_schema_table_as(schema.name(), table.name(), alias.name(), LeftJoin)
    }

    fn left_join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.left_join_raw_table_as(table, "")
    }
    fn left_join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
       self.with_join_raw_table_as(table, alias, LeftJoin)
    }

    fn left_join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        self.left_join_table_query_as(query, "")
    }
    fn left_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
       self.with_join_table_query_as(query, alias.name(), LeftJoin)
    }
    fn left_join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_table(table, LeftJoin)
    }
    fn left_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_simple_table(table,LeftJoin)
    }
    fn left_join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
        self.with_join_rdbc_query_table(query,LeftJoin)
    }
    fn left_join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_raw_table(table,LeftJoin)
    }

    fn right_join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.right_join_table_as(table, "")
    }

    fn right_join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.right_join_schema_table_as("", table, alias)
    }
    fn right_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.right_join_schema_table_as(schema, table, "")
    }
    fn right_join_schema_table_as<S, T, A>(
        &mut self,
        schema: S,
        table: T,
        alias: A,
    ) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
       self.with_join_schema_table_as(schema.name(), table.name(), alias.name(), RightJoin)
    }

    fn right_join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.right_join_raw_table_as(table, "")
    }
    fn right_join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.with_join_raw_table_as(table, alias, RightJoin)
    }

    fn right_join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        self.right_join_table_query_as(query, "")
    }
    fn right_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        self.with_join_table_query_as(query, alias.name(), RightJoin)
    }
    fn right_join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
        self.with_join_rdbc_table(table, RightJoin)
    }
    fn right_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
        self.with_join_rdbc_simple_table(table, RightJoin)
    }
    fn right_join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_query_table(query,RightJoin)
    }
    fn right_join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_raw_table(table,RightJoin)
    }
    fn full_join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.full_join_table_as(table, "")
    }

    fn full_join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.full_join_schema_table_as("", table, alias)
    }
    fn full_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.full_join_schema_table_as(schema, table, "")
    }
    fn full_join_schema_table_as<S, T, A>(
        &mut self,
        schema: S,
        table: T,
        alias: A,
    ) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.with_join_schema_table_as(schema.name(), table.name(), alias.name(), FullJoin)
    }

    fn full_join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.full_join_raw_table_as(table, "")
    }
    fn full_join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
       self.with_join_raw_table_as(table, alias, FullJoin)
    }

    fn full_join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        self.full_join_table_query_as(query, "")
    }
    fn full_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
       self.with_join_table_query_as(query, alias.name(), FullJoin)
    }
    fn full_join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_table(table, FullJoin)
    }
    fn full_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_simple_table(table,FullJoin)
    }
    fn full_join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_query_table(query,FullJoin)
    }
    fn full_join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
       self.with_join_rdbc_raw_table(table,FullJoin)
    }

    fn with_join_table<T, A>(&mut self, table: T, join_type: JoinType) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.with_join_table_as(table, "", join_type)
    }

    fn with_join_table_as<T, A>(
        &mut self,
        table: T,
        alias: A,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        self.with_join_schema_table_as("", table, alias, join_type)
    }
    fn with_join_schema_table<S, T>(
        &mut self,
        schema: S,
        table: T,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        self.with_join_schema_table_as(schema, table, "", join_type)
    }
    fn with_join_schema_table_as<S, T, A>(
        &mut self,
        schema: S,
        table: T,
        alias: A,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(RdbcSimpleTable {
                schema: schema.name(),
                table: table.name(),
                alias: alias.name(),
            }),
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn with_join_raw_table<T>(&mut self, table: T, join_type: JoinType) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        self.with_join_raw_table_as(table, "", join_type)
    }
    fn with_join_raw_table_as<S, T>(
        &mut self,
        table: S,
        alias: T,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable {
                table: table.name(),
                alias: alias.name(),
            }),
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn with_join_table_query(
        &mut self,
        query: RdbcQuery,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable {
        self.with_join_table_query_as(query, "", join_type)
    }
    fn with_join_table_query_as<A>(
        &mut self,
        query: RdbcQuery,
        alias: A,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(RdbcQueryTable {
                query,
                alias: alias.name(),
            }),
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_table(
        &mut self,
        table: RdbcTable,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table,
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_simple_table(
        &mut self,
        table: RdbcSimpleTable,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(table),
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_query_table(
        &mut self,
        query: RdbcQueryTable,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(query),
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_raw_table(
        &mut self,
        table: RdbcRawTable,
        join_type: JoinType,
    ) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(table),
            join_type,
            filter: None,
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
}

impl RdbcWhereFilterBuilder for RdbcJoinTable {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        self.filter.get_or_insert(RdbcWhereFilter {
            type_: RdbcFilterType::And,
            conditions: vec![],
            distinct: false,
        })
    }
    fn filter_take(&mut self) -> Option<RdbcWhereFilter> {
        self.filter.take()
    }
}
