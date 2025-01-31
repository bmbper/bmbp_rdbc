use crate::client::{RdbcPgConnection, RdbcPgTransaction};
use crate::ds::{RdbcDbConfig, RdbcDbType};
use crate::orm::RdbcOrmExecutor;
use crate::pool::RdbcPool;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError, RdbcPage, RdbcRow, RdbcValue};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::{Arc, Weak};
pub struct RdbcPooledConnection {
    pub(crate) id: String,
    db_config: Arc<RdbcDbConfig>,
    inner: Option<RdbcConnection>,
    pub(crate) pool: Weak<RdbcPool>,
}

impl RdbcPooledConnection {
    pub async fn connect(pool_conn: Arc<RdbcPool>) -> Result<RdbcPooledConnection, RdbcError> {
        let conn_inner = Self::connect_db(pool_conn.db_config.clone()).await?;
        let conn = RdbcPooledConnection {
            id: uuid::Uuid::new_v4().to_string(),
            db_config: pool_conn.db_config.clone(),
            pool: Arc::downgrade(&pool_conn),
            inner: Some(conn_inner),
        };
        Ok(conn)
    }
    async fn connect_db(config: Arc<RdbcDbConfig>) -> Result<RdbcConnection, RdbcError> {
        return match &config.database_type {
            RdbcDbType::Postgres => {
                let conn = RdbcPgConnection::connect(config.clone()).await?;
                Ok(RdbcConnection::Pg(conn))
            }
            _ => Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "不支持的数据库类型".to_string(),
            )),
        };
    }

    pub(crate) async fn get_transaction(&mut self) -> Result<RdbcTransaction, RdbcError> {
        self.inner.as_mut().unwrap().get_transaction().await
    }
}

impl Drop for RdbcPooledConnection {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.upgrade() {
            let conn = RdbcPooledConnection {
                id: self.id.clone(),
                db_config: self.db_config.clone(),
                pool: self.pool.clone(),
                inner: self.inner.take(),
            };
            pool.conn_pool.lock().unwrap().push_back(conn);
        }
    }
}

impl RdbcOrmExecutor for RdbcPooledConnection {
    async fn query_page(
        &self,
        page_num: usize,
        page_size: usize,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<RdbcPage<RdbcRow>, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.query_page(page_num, page_size, execute_sql, params)
                .await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn query_list(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Vec<RdbcRow>, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.query_list(execute_sql, params).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn query_one_option(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Option<RdbcRow>, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.query_one_option(execute_sql, params).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn query_page_as<T>(
        &self,
        page_num: usize,
        page_size: usize,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<RdbcPage<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        if let Some(conn) = self.inner.as_ref() {
            conn.query_page_as(page_num, page_size, execute_sql, params)
                .await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn query_list_as<T>(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Vec<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        if let Some(conn) = self.inner.as_ref() {
            conn.query_list_as(execute_sql, params).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn query_one_option_as<T>(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Option<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        if let Some(conn) = self.inner.as_ref() {
            conn.query_one_option_as(execute_sql, params).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn execute(&self, execute_sql: String, params: &[RdbcValue]) -> Result<usize, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.execute(execute_sql, params).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn execute_batch(
        &self,
        execute_sql: String,
        params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.execute_batch(execute_sql, params).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn execute_raw(&self, execute_sql: String) -> Result<usize, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.execute_raw(execute_sql).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn execute_batch_raw(&self, execute_sql: &[String]) -> Result<usize, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.execute_batch_raw(execute_sql).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }

    async fn execute_batch_slice(
        &self,
        execute_sqlparams: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        if let Some(conn) = self.inner.as_ref() {
            conn.execute_batch_slice(execute_sqlparams).await
        } else {
            Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                "连接已关闭".to_string(),
            ))
        }
    }
}
pub enum RdbcConnection {
    Pg(RdbcPgConnection),
}
impl RdbcConnection {
    pub async fn get_transaction(&mut self) -> Result<RdbcTransaction, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.get_transaction().await,
        }
    }
}

impl RdbcOrmExecutor for RdbcConnection {
    async fn query_page(
        &self,
        page_num: usize,
        page_size: usize,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<RdbcPage<RdbcRow>, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => {
                conn.query_page(page_num, page_size, execute_sql, params)
                    .await
            }
        }
    }

    async fn query_list(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Vec<RdbcRow>, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.query_list(execute_sql, params).await,
        }
    }

    async fn query_one_option(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Option<RdbcRow>, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.query_one_option(execute_sql, params).await,
        }
    }

    async fn query_page_as<T>(
        &self,
        page_num: usize,
        page_size: usize,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<RdbcPage<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        match self {
            RdbcConnection::Pg(conn) => {
                conn.query_page_as(page_num, page_size, execute_sql, params)
                    .await
            }
        }
    }

    async fn query_list_as<T>(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Vec<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        match self {
            RdbcConnection::Pg(conn) => conn.query_list_as(execute_sql, params).await,
        }
    }

    async fn query_one_option_as<T>(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Option<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        match self {
            RdbcConnection::Pg(conn) => conn.query_one_option_as(execute_sql, params).await,
        }
    }

    async fn execute(
        &self,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<usize, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.execute(execute_sql, params).await,
        }
    }

    async fn execute_batch(
        &self,
        execute_sql: String,
        params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.execute_batch(execute_sql, params).await,
        }
    }

    async fn execute_raw(&self, execute_sql: String) -> Result<usize, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.execute_raw(execute_sql).await,
        }
    }

    async fn execute_batch_raw(&self, execute_sql: &[String]) -> Result<usize, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.execute_batch_raw(execute_sql).await,
        }
    }

    async fn execute_batch_slice(
        &self,
        execute_sqlparams: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        match self {
            RdbcConnection::Pg(conn) => conn.execute_batch_slice(execute_sqlparams).await,
        }
    }
}
pub enum RdbcTransaction<'a> {
    Pg(RdbcPgTransaction<'a>),
}
impl<'a> RdbcTransaction<'a> {
    pub(crate) async fn commit(&mut self) -> Result<usize, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.commit().await,
        }
    }
    pub(crate) async fn rollback(&mut self) -> Result<usize, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.rollback().await,
        }
    }
}
impl RdbcOrmExecutor for RdbcTransaction<'_> {
    async fn query_page(&self, page_num: usize, page_size: usize, execute_sql: String, params: &[RdbcValue]) -> Result<RdbcPage<RdbcRow>, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.query_page(page_num, page_size, execute_sql, params).await,
        }
    }

    async fn query_list(&self, execute_sql: String, params: &[RdbcValue]) -> Result<Vec<RdbcRow>, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.query_list(execute_sql, params).await,
        }
    }

    async fn query_one_option(&self, execute_sql: String, params: &[RdbcValue]) -> Result<Option<RdbcRow>, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.query_one_option(execute_sql, params).await,
        }
    }

    async fn query_page_as<T>(&self, page_num: usize, page_size: usize, execute_sql: String, params: &[RdbcValue]) -> Result<RdbcPage<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone
    {
        match self {
            RdbcTransaction::Pg(trans) => trans.query_page_as(page_num, page_size, execute_sql, params).await,
        }
    }

    async fn query_list_as<T>(&self, execute_sql: String, params: &[RdbcValue]) -> Result<Vec<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone
    {
        match self {
            RdbcTransaction::Pg(trans) => trans.query_list_as(execute_sql, params).await,
        }
    }

    async fn query_one_option_as<T>(&self, execute_sql: String, params: &[RdbcValue]) -> Result<Option<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone
    {
        match self {
            RdbcTransaction::Pg(trans) => trans.query_one_option_as(execute_sql, params).await,
        }
    }

    async fn execute(&self, execute_sql: String, params: &[RdbcValue]) -> Result<usize, RdbcError> {
       match self {
           RdbcTransaction::Pg(trans) => trans.execute(execute_sql, params).await,
       }
    }

    async fn execute_batch(&self, execute_sql: String, params: &[&[RdbcValue]]) -> Result<usize, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.execute_batch(execute_sql, params).await,
        }
    }

    async fn execute_raw(&self, execute_sql: String) -> Result<usize, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.execute_raw(execute_sql).await,
        }
    }

    async fn execute_batch_raw(&self, execute_sql: &[String]) -> Result<usize, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.execute_batch_raw(execute_sql).await,
        }
    }

    async fn execute_batch_slice(&self, execute_sqlparams: &[(&String, &[&RdbcValue])]) -> Result<usize, RdbcError> {
        match self {
            RdbcTransaction::Pg(trans) => trans.execute_batch_slice(execute_sqlparams).await,
        }
    }
}