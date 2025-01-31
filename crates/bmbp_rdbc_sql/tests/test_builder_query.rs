use bmbp_rdbc_sql::{RdbcDistinctFunc, RdbcQueryBuilder, RdbcTableBuilder, RdbcFilterBuilder};
use bmbp_rdbc_type::RdbcIdent;

#[test]
pub fn test_builder_simple_query() {
    let mut query_builder = RdbcQueryBuilder::new();

    query_builder
        .select("name")
        .select("age".to_string())
        .select_raw("3")
        .select_value("tt")
        .select_raw(5i8);
    query_builder.table("damp");
    query_builder.table_slice(&["a", "B"]);

    let v: Vec<Box<dyn RdbcIdent>> = vec![
        Box::new("name"),
        Box::new(2),
        Box::new("d"),
        Box::new("C".to_string()),
    ];
    for item in v {
        println!("{}", item.name());
    }

    query_builder.eq_v("name", "damp").eq_col("test","test").eq_script("dd","hello");

}
