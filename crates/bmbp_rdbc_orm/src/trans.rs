use crate::client::{
    RdbcMysqlTransaction, RdbcOracleTransaction, RdbcPgTransaction, RdbcSqliteTransaction,
};
use bmbp_rdbc_type::Executor;

pub struct RdbcTransaction {
    inner: RdbcTransactionInner,
}
pub enum RdbcTransactionInner {
    Oracle(RdbcOracleTransaction),
    Mysql(RdbcMysqlTransaction),
    Pg(RdbcPgTransaction),
    Sqlite(RdbcSqliteTransaction),
}
impl Executor for RdbcTransaction {}
