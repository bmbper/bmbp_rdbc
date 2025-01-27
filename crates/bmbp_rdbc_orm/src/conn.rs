use crate::client::RdbcPgConnection;
use crate::ds::{RdbcDbConfig, RdbcDbType};
use crate::pool::RdbcPool;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError};
use std::fmt::Debug;
use std::sync::{Arc, Weak};

pub trait RdbcConnectionTrait {}
pub struct RdbcConnection {
    pub(crate) id: String,
    db_config: Arc<RdbcDbConfig>,
    inner: Box<dyn RdbcConnectionTrait>,
    pub(crate) pool: Weak<RdbcPool>,
}

impl RdbcConnection {
    pub async fn connect(pool_conn: Arc<RdbcPool>) -> Result<RdbcConnection, RdbcError> {
        let conn_inner: Box<dyn RdbcConnectionTrait> =
            Self::connect_db(pool_conn.db_config.clone()).await?;
        let conn = RdbcConnection {
            id: uuid::Uuid::new_v4().to_string(),
            db_config: pool_conn.db_config.clone(),
            pool: Arc::downgrade(&pool_conn),
            inner: conn_inner,
        };
        Ok(conn)
    }

    async fn connect_db(
        config: Arc<RdbcDbConfig>,
    ) -> Result<Box<dyn RdbcConnectionTrait>, RdbcError> {
        return match &config.database_type {
            RdbcDbType::Postgres => {
                let conn = RdbcPgConnection::connect(config.clone()).await?;
                let box_conn: Box<dyn RdbcConnectionTrait> = Box::new(conn);
                Ok(box_conn)
            }
            _ => Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "不支持的数据库类型".to_string(),
            )),
        };
    }
}

