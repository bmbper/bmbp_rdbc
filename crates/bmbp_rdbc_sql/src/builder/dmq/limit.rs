pub trait  RdbcLimitBuilder {
    fn limit(&mut self, limit: u64) -> &mut Self;
}
pub trait  RdbcOffsetBuilder {
    fn offset(&mut self, limit: u64) -> &mut Self;
}