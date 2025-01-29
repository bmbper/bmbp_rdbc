use crate::client::RdbcPgConnection;
use crate::ds::{RdbcDbConfig, RdbcDbType};
use crate::exec::Executor;
use crate::pool::RdbcPool;
use crate::trans::RdbcTransactionInner;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError, RdbcPage, RdbcRow, RdbcValue};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::{Arc, Weak};

pub enum RdbcConnectionImpl {
    Pg(RdbcPgConnection),
}
impl RdbcConnectionImpl {
    pub async fn get_transaction(&mut self) -> Result<RdbcTransactionInner, RdbcError> {
        match self {
            RdbcConnectionImpl::Pg(conn) => conn.get_transaction().await,
        }
    }
}

pub struct RdbcConnection {
    pub(crate) id: String,
    db_config: Arc<RdbcDbConfig>,
    inner: Option<RdbcConnectionImpl>,
    pub(crate) pool: Weak<RdbcPool>,
}

impl RdbcConnection {
    pub async fn connect(pool_conn: Arc<RdbcPool>) -> Result<RdbcConnection, RdbcError> {
        let conn_inner = Self::connect_db(pool_conn.db_config.clone()).await?;
        let conn = RdbcConnection {
            id: uuid::Uuid::new_v4().to_string(),
            db_config: pool_conn.db_config.clone(),
            pool: Arc::downgrade(&pool_conn),
            inner: Some(conn_inner),
        };
        Ok(conn)
    }
    async fn connect_db(config: Arc<RdbcDbConfig>) -> Result<RdbcConnectionImpl, RdbcError> {
        return match &config.database_type {
            RdbcDbType::Postgres => {
                let conn = RdbcPgConnection::connect(config.clone()).await?;
                Ok(RdbcConnectionImpl::Pg(conn))
            }
            _ => Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "不支持的数据库类型".to_string(),
            )),
        };
    }

    pub(crate) async fn get_transaction(&mut self) -> Result<RdbcTransactionInner, RdbcError> {
        self.inner.as_mut().unwrap().get_transaction().await
    }
}

impl Drop for RdbcConnection {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.upgrade() {
            let conn = RdbcConnection {
                id: self.id.clone(),
                db_config: self.db_config.clone(),
                pool: self.pool.clone(),
                inner: self.inner.take(),
            };
            pool.conn_pool.lock().unwrap().push_back(conn);
        }
    }
}

impl Executor for RdbcConnection {
    async fn query_page(
        &self,
        _page_num: usize,
        _page_size: usize,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<RdbcPage<RdbcRow>, RdbcError> {
        Ok(RdbcPage::new())
    }

    async fn query_list(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Vec<RdbcRow>, RdbcError> {
        Ok(vec![])
    }

    async fn query_one_option(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Option<RdbcRow>, RdbcError> {
        Ok(None)
    }

    async fn query_page_as<T>(
        &self,
        _page_num: usize,
        _page_size: usize,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<RdbcPage<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        Ok(RdbcPage::new())
    }

    async fn query_list_as<T>(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Vec<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        Ok(vec![])
    }

    async fn query_one_option_as<T>(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Option<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        Ok(None)
    }

    async fn execute(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch(
        &self,
        _execute_sql: String,
        _params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_raw(&self, _execute_sql: String) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch_raw(&self, _execute_sql: &[String]) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch_slice(
        &self,
        _execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }
}
