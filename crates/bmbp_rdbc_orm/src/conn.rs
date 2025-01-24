use bmbp_rdbc_type::Executor;
use crate::client::{RdbcMysqlConnection, RdbcOracleConnection, RdbcPgConnection, RdbcSqliteConnection};

pub struct RdbcConnection {
    inner: RdbcConnectionInner
}

pub enum RdbcConnectionInner {
    Oracle(RdbcOracleConnection),
    Mysql(RdbcMysqlConnection),
    Pg(RdbcPgConnection),
    Sqlite(RdbcSqliteConnection)
}

impl Executor for RdbcConnection {
}