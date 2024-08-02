use bmbp_rdbc_sql::{QueryWrapper, RdbcIdent, RdbcSQL, RdbcTable, RdbcTableWrapper};

#[test]
fn test_query_table() {
    pub struct Demo {
        name: String,
        title: String,
    }

    pub enum DemoVars {
        Name,
        Title,
    }
    impl RdbcIdent for DemoVars {
        fn get_ident(&self) -> String {
            match self {
                DemoVars::Name => "name".to_string(),
                DemoVars::Title => "title".to_string(),
            }
        }
    }

    impl RdbcTable for Demo {
        fn get_table() -> impl RdbcIdent {
            "DEMO".to_string()
        }

        fn get_columns() -> Vec<impl RdbcIdent> {
            vec![
                DemoVars::Name,
                DemoVars::Title,
            ]
        }
    }

    let query = QueryWrapper::new_from::<Demo>();
    println!("=====>{:#?}", query.build_sql(bmbp_rdbc_sql::RdbcDataBase::Postgres).0);

    let mut query2 = QueryWrapper::new();
    query2.table(Demo::get_table().get_ident());
    query2.column(DemoVars::Name).column(DemoVars::Title.as_alias("name2"));
    query2.column(DemoVars::Name.with_alias("t1")).column(DemoVars::Title.as_alias("name2"));
    println!("=====>{:#?}", query2.build_sql(bmbp_rdbc_sql::RdbcDataBase::Postgres).0);
}
