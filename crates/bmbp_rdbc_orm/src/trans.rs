use crate::client::RdbcPgTransaction;
use crate::conn::{RdbcConnection, RdbcConnectionImpl};
use bmbp_rdbc_type;
use bmbp_rdbc_type::RdbcError;

pub struct RdbcTransaction<'a> {
    pub(crate) conn: RdbcConnection,
    pub(crate) inner: RdbcTransactionInner<'a>,
}

pub enum RdbcTransactionInner<'a> {
    Pg(RdbcPgTransaction<'a>),
}

impl<'a> RdbcTransactionInner<'a> {
    pub(crate) async fn build_trans(conn: &'a mut RdbcConnection) -> Result<Self, RdbcError> {
        conn.get_transaction().await
    }
}
