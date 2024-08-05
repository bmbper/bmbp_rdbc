use bmbp_rdbc_type::rdbc_datasource::RdbcDataSource;

#[test]
fn test_rdbc_datasource() {
    let ds = RdbcDataSource::new();
    assert_eq!(ds.get_host().is_none(), true);
}
