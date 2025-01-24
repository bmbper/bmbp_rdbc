use bmbp_rdbc_type::RdbcIdent;
use crate::builder::part::RdbcWhereFilterBuilder;
use crate::{
    JoinType,  RdbcJoinTable, RdbcQuery, RdbcQueryTable, RdbcRawTable, RdbcSimpleTable,
    RdbcTable, RdbcWhereFilter,
};

pub trait RdbcJoinTableBuilder {
    fn table_join_mut(&mut self) -> &mut Vec<RdbcJoinTable>;
    fn join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from(table.name()),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((table.name(), alias.name())),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::new_with_schema_table(schema.name(), table.name()),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((schema.name(), table.name(), alias.name())),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from(table.name())),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from((table.name(), alias.name()))),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::from(query),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((query, alias.name())),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table,
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(table),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(query),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(table),
            join_type: JoinType::InnerJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }




    fn left_join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from(table.name()),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn left_join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((table.name(), alias.name())),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::new_with_schema_table(schema.name(), table.name()),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((schema.name(), table.name(), alias.name())),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn left_join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from(table.name())),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from((table.name(), alias.name()))),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn left_join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::from(query),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((query, alias.name())),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table,
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(table),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(query),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn left_join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(table),
            join_type: JoinType::LeftJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }


    fn right_join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from(table.name()),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn right_join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((table.name(), alias.name())),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::new_with_schema_table(schema.name(), table.name()),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((schema.name(), table.name(), alias.name())),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn right_join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from(table.name())),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from((table.name(), alias.name()))),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn right_join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::from(query),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((query, alias.name())),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table,
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(table),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(query),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn right_join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(table),
            join_type: JoinType::RightJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_table<T, A>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from(table.name()),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn full_join_table_as<T, A>(&mut self, table: T, alias: A) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((table.name(), alias.name())),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_schema_table<S, T>(&mut self, schema: S, table: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::new_with_schema_table(schema.name(), table.name()),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((schema.name(), table.name(), alias.name())),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn full_join_raw_table<T>(&mut self, table: T) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from(table.name())),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_raw_table_as<S, T>(&mut self, table: S, alias: T) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from((table.name(), alias.name()))),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn full_join_table_query(&mut self, query: RdbcQuery) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::from(query),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((query, alias.name())),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_rdbc_table(&mut self, table: RdbcTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table,
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(table),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_rdbc_query_table(&mut self, query: RdbcQueryTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(query),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn full_join_rdbc_raw_table(&mut self, table: RdbcRawTable) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(table),
            join_type: JoinType::FullJoin,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn with_join_table<T, A>(&mut self, table: T,join_type: JoinType) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from(table.name()),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn with_join_table_as<T, A>(&mut self, table: T, alias: A,join_type: JoinType) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((table.name(), alias.name())),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_schema_table<S, T>(&mut self, schema: S, table: T,join_type: JoinType) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::new_with_schema_table(schema.name(), table.name()),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_schema_table_as<S, T, A>(&mut self, schema: S, table: T, alias: A,join_type: JoinType) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((schema.name(), table.name(), alias.name())),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn with_join_raw_table<T>(&mut self, table: T,join_type: JoinType) -> &mut RdbcJoinTable
    where
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from(table.name())),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_raw_table_as<S, T>(&mut self, table: S, alias: T,join_type: JoinType) -> &mut RdbcJoinTable
    where
        S: RdbcIdent,
        T: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(RdbcRawTable::from((table.name(), alias.name()))),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }

    fn with_join_table_query(&mut self, query: RdbcQuery,join_type: JoinType) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::from(query),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_table_query_as<A>(&mut self, query: RdbcQuery, alias: A,join_type: JoinType) -> &mut RdbcJoinTable
    where
        A: RdbcIdent,
    {
        let table = RdbcJoinTable {
            table: RdbcTable::from((query, alias.name())),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_table(&mut self, table: RdbcTable,join_type: JoinType) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table,
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_simple_table(&mut self, table: RdbcSimpleTable,join_type: JoinType) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::SimpleTable(table),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_query_table(&mut self, query: RdbcQueryTable,join_type: JoinType) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::QueryTable(query),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
    fn with_join_rdbc_raw_table(&mut self, table: RdbcRawTable,join_type: JoinType) -> &mut RdbcJoinTable {
        let table = RdbcJoinTable {
            table: RdbcTable::RawTable(table),
            join_type,
            filter: RdbcWhereFilter::new(),
        };
        self.table_join_mut().push(table);
        self.table_join_mut().last_mut().unwrap()
    }
}
impl RdbcWhereFilterBuilder for RdbcJoinTable {
    fn filter_mut(&mut self) -> &mut RdbcWhereFilter {
        &mut self.filter
    }
}
