use bmbp_rdbc_sql::{QueryWrapper, RdbcConcatType, RdbcSQL, RdbcTableFilter, RdbcTableFilterImpl, RdbcTableWrapper};
use bmbp_rdbc_type::{RdbcDataBase, RdbcIdent, RdbcTable};

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

        fn get_primary_key() -> impl RdbcIdent {
            DemoVars::Name
        }

        fn get_union_key() -> Vec<impl RdbcIdent> {
            vec![DemoVars::Name]
        }
    }

    let query = QueryWrapper::new_from::<Demo>();
    println!("=====>{:#?}", query.build_sql(RdbcDataBase::Postgres).0);

    let mut query2 = QueryWrapper::new();
    query2.table(Demo::get_table().get_ident());
    query2.column(DemoVars::Name).column(DemoVars::Title.as_alias("name2"));
    query2.column(DemoVars::Name.with_alias("t1")).column(DemoVars::Title.as_alias("name2"));
    println!("=====>{:#?}", query2.build_sql(RdbcDataBase::Postgres).0);

    let mut query3 = QueryWrapper::new();
    query3.select(DemoVars::Name);
    query3.eq_(DemoVars::Name, "1");
    println!("=====>{:#?}", query3.build_sql(RdbcDataBase::Postgres).0);
    let v = vec!["1".to_string(), "2".to_string()];
    query3.in_v_slice(DemoVars::Name, v.as_slice());
    let mut filter2 = RdbcTableFilterImpl::concat(RdbcConcatType::Or);
    filter2.in_v_slice(DemoVars::Name, v.as_slice());
    filter2.in_v_slice(DemoVars::Name, v.as_slice());
}
