use bmbp_rdbc_type::RdbcValue;

#[test]
fn test_rdbc_value() {
    let rdbc_value = RdbcValue::Vec(vec![]);
    assert_eq!(rdbc_value.get_string(), "".to_string());
}
