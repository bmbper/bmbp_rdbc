use crate::client::{RdbcMysqlPool, RdbcOraclePool, RdbcPgPool, RdbcSqlitePool};
use crate::ds::{RdbcDbConfig, RdbcDbType};
use std::sync::Arc;
use bmbp_rdbc_type::{Executor, RdbcError};

pub struct RdbcPool {
    pub inner: RdbcPoolInner,
}

pub struct RdbcPoolBuilder {}

impl RdbcPoolBuilder {
    pub async fn connect(db_config: RdbcDbConfig) -> Result<Arc<RdbcPool>, RdbcError> {
        let db_config = Arc::new(db_config);
        let pool_inner = match db_config.database_type {
            RdbcDbType::Mysql => {
                RdbcPoolInner::Mysql(RdbcMysqlPool::connect(db_config.clone()).await?)
            }
            RdbcDbType::Postgres => RdbcPoolInner::Pg(RdbcPgPool::connect(db_config.clone()).await?),
            RdbcDbType::Sqlite => {
                RdbcPoolInner::Sqlite(RdbcSqlitePool::connect(db_config.clone()).await?)
            }
            RdbcDbType::Oracle => {
                RdbcPoolInner::Oracle(RdbcOraclePool::connect(db_config.clone()).await?)
            }
        };
        let pool = RdbcPool { inner: pool_inner };
        Ok(Arc::new(pool))
    }
}

pub enum RdbcPoolInner {
    Oracle(RdbcOraclePool),
    Mysql(RdbcMysqlPool),
    Sqlite(RdbcSqlitePool),
    Pg(RdbcPgPool),
}


impl Executor for RdbcPool {}