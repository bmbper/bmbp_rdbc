use crate::RdbcQuery;

pub trait RdbcUnionBuilder{
    fn union(&mut self, query: RdbcQuery) -> &mut Self;
}
pub trait RdbcUnionAllBuilder{
    fn union_all(&mut self, query: RdbcQuery) -> &mut Self;
}