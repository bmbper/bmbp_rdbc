pub trait RdbcIdent {
    fn name(&self) -> String;
}
impl<T> RdbcIdent for T
where
    T: ToString,
{
    fn name(&self) -> String {
        self.to_string()
    }
}
