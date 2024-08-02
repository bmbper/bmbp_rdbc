use bmbp_rdbc_sql::{RdbcIdent, RdbcTable};

#[test]
fn test_ident() {
    let name = "test_ident";
    assert_eq!("test_ident".to_string(), name.get_ident())
}

#[test]
fn test_rdbc_table() {
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

    assert_eq!("DEMO".to_string(), Demo::get_table().get_ident());
    assert_eq!("name".to_string(), DemoVars::Name.get_ident());
    assert_eq!(
        vec![
            "name".to_string(),
            "title".to_string(),
        ],
        Demo::get_columns().iter().map(|x| x.get_ident()).collect::<Vec<String>>()
    );

}
