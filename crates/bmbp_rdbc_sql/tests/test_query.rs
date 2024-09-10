use bmbp_rdbc_sql::{MysqlBuilder, PgSqlBuilder, QueryWrapper, RdbcTableFilter, SQLBuilder};
use bmbp_rdbc_type::{RdbcDataBase, RdbcIdent, RdbcTable};

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

#[test]
fn test_query_select() {}

#[test]
fn test_query_eq() {}

#[test]
fn test_query_like() {}

#[test]
fn test_query_in() {
    let mut query = QueryWrapper::new_from::<Demo>();
    query.select(DemoVars::Name);
    query.eq_(DemoVars::Name, "name");
    query.eq_col(DemoVars::Title, "title");
    query.ne_(DemoVars::Name, 2);
    query.ne_col(DemoVars::Title, "title");
    query.ge_(DemoVars::Name, 1);
    query.ge_col(DemoVars::Title, "title");
    query.gt_(DemoVars::Name, 1);
    query.gt_col(DemoVars::Title, "title");
    query.lt_(DemoVars::Name, 2);
    query.lt_col(DemoVars::Title, "title");
    query.le_(DemoVars::Name, 2);
    query.le_col(DemoVars::Title, "title");
    query.in_v(DemoVars::Name, vec!["1", "2"]);
    query.in_v_slice(DemoVars::Name, &["1", "2"]);
    query.like_(DemoVars::Name, "name");
    query.like_left_(DemoVars::Name, "name");
    query.like_right_(DemoVars::Name, "name");
    query.not_like_(DemoVars::Name, "name");
    query.not_like_left_(DemoVars::Name, "name");
    query.not_like_right_(DemoVars::Name, "name");
    query.between_(DemoVars::Name, 1, 2);
    query.not_between_(DemoVars::Name, 1, 2);
    query.null_(DemoVars::Name);
    query.not_null_(DemoVars::Name);
}


#[test]
fn test_query_left() {}

#[test]
fn test_query_right() {}


#[test]
fn test_query_exists() {}

#[test]
fn test_query_not_exists() {}

#[test]
fn test_query_group_by() {}

#[test]
fn test_query_having() {}

#[test]
fn test_query_order() {}

#[test]
fn test_build_query_script() {
    let query = QueryWrapper::new_from::<Demo>();
    let (sql, _) = SQLBuilder::build_query_script::<PgSqlBuilder>(&query);
    println!("sql:{}", sql);
}