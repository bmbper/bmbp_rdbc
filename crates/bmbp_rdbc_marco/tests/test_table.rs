use case_style::CaseStyle;
use bmbp_rdbc_marco::table;
use bmbp_rdbc_type::RdbcIdent;
use bmbp_rdbc_type::RdbcTable;
#[test]
pub fn test_table() {
    #[table("DemoAttrs")]
    pub struct Demo {
        #[id]
        name: String,
        #[id]
        title: String,
    }
    assert_eq!("DEMO_ATTRS".to_string(), Demo::get_table().get_ident());
}