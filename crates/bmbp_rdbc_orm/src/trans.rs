use bmbp_rdbc_type;

pub trait RdbcTransaction{}
pub struct RdbcTransactionManager {
    inner: Box<dyn RdbcTransaction>,
}