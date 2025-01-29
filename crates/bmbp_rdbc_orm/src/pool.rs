use crate::conn::RdbcConnection;
use crate::ds::RdbcDbConfig;
use crate::exec::Executor;
use crate::trans::{RdbcTransaction, RdbcTransactionInner};
use bmbp_rdbc_sql::RdbcFilterType;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError, RdbcPage, RdbcRow, RdbcValue};
use chrono::Duration;
use serde::Serialize;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::time::timeout;
use tracing::info;

pub struct RdbcPool {
    pub db_config: Arc<RdbcDbConfig>,
    pub conn_pool: Arc<Mutex<VecDeque<RdbcConnection>>>,
}

impl RdbcPool {
    pub async fn connect(db_config: Arc<RdbcDbConfig>) -> Result<Arc<RdbcPool>, RdbcError> {
        let pool = RdbcPool {
            db_config: db_config.clone(),
            conn_pool: Arc::new(Mutex::new(VecDeque::new())),
        };
        let arc_pool = Arc::new(pool);
        Self::init_pool(arc_pool.clone()).await?;
        Ok(arc_pool)
    }
    async fn init_pool(arc: Arc<RdbcPool>) -> Result<(), RdbcError> {
        let pool_size = {
            if let Some(config) = &arc.db_config.clone().pool_config {
                config.int_size.clone()
            } else {
                10u16
            }
        };
        info!("初始化连接池大小：{}", pool_size);
        for _ in 0..pool_size {
            let mut conn = RdbcConnection::connect(arc.clone()).await?;
            conn.pool = Arc::downgrade(&arc);
            arc.conn_pool.lock().unwrap().push_back(conn);
        }
        Ok(())
    }
    pub fn get_connection(&self) -> Result<RdbcConnection, RdbcError> {
        let timeout = {
            if let Some(config) = &self.db_config.pool_config {
                config.wait_timeout.clone()
            } else {
                Duration::seconds(60)
            }
        };
        let start = chrono::Utc::now();
        loop {
            let end = chrono::Utc::now();
            if end.signed_duration_since(start).ge(&timeout) {
                return Err(RdbcError::new(
                    RdbcErrKind::CONNECTION,
                    "获取连接超时".to_string(),
                ));
            }
            if let Ok(mut lock) = self.conn_pool.lock() {
                if let Some(conn) = lock.pop_front() {
                    return Ok(conn);
                }
            }
        }
    }

    pub fn idle_conn_size(&self) -> usize {
        self.conn_pool.lock().unwrap().len()
    }
}

impl Executor for Arc<RdbcPool> {
    async fn query_page(
        &self,
        _page_num: usize,
        _page_size: usize,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<RdbcPage<RdbcRow>, RdbcError> {
        self.get_connection()?
            .query_page(_page_num, _page_size, _execute_sql, _params)
            .await
    }

    async fn query_list(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Vec<RdbcRow>, RdbcError> {
        self.get_connection()?
            .query_list(_execute_sql, _params)
            .await
    }

    async fn query_one_option(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Option<RdbcRow>, RdbcError> {
        self.get_connection()?
            .query_one_option(_execute_sql, _params)
            .await
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
        self.get_connection()?
            .query_page_as(_page_num, _page_size, _execute_sql, _params)
            .await
    }

    async fn query_list_as<T>(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Vec<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        self.get_connection()?
            .query_list_as(_execute_sql, _params)
            .await
    }

    async fn query_one_option_as<T>(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Option<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        self.get_connection()?
            .query_one_option_as(_execute_sql, _params)
            .await
    }

    async fn execute(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<usize, RdbcError> {
        self.get_connection()?.execute(_execute_sql, _params).await
    }

    async fn execute_batch(
        &self,
        _execute_sql: String,
        _params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        self.get_connection()?
            .execute_batch(_execute_sql, _params)
            .await
    }

    async fn execute_raw(&self, _execute_sql: String) -> Result<usize, RdbcError> {
        self.get_connection()?.execute_raw(_execute_sql).await
    }

    async fn execute_batch_raw(&self, _execute_sql: &[String]) -> Result<usize, RdbcError> {
        self.get_connection()?.execute_batch_raw(_execute_sql).await
    }

    async fn execute_batch_slice(
        &self,
        _execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        self.get_connection()?
            .execute_batch_slice(_execute_sql_params)
            .await
    }
}
#[cfg(test)]
mod test {
    use crate::ds::{RdbcDbConfig, RdbcDbType};
    use crate::pool::RdbcPool;
    use std::sync::Arc;
    use tracing::{error, info};
    use tracing_subscriber::fmt;

    #[tokio::test]
    async fn test_pool() {
        fmt().init();
        let db_config = RdbcDbConfig::new(
            RdbcDbType::Postgres,
            "localhost",
            5432,
            "bmbp",
            "zgk0130!",
            "bmbp",
            "public",
            None,
        );
        let arc_db_config = Arc::new(db_config);
        let pool = RdbcPool::connect(arc_db_config.clone()).await;
        match pool {
            Ok(pool) => {
                info!("创建连接池成功");
                let size = pool.idle_conn_size();
                info!("空闲连接数：{}", size);
                assert!(size > 0);
                {
                    let conn_rs = pool.get_connection();
                    match conn_rs {
                        Ok(mut conn) => {
                            info!("获取连接成功");
                            info!("空闲连接数{}", pool.idle_conn_size());
                            match conn.get_transaction().await {
                                Ok(trans) => {
                                    info!("获取事务成功");
                                }
                                Err(err) => {
                                    error!("获取事务事变：err:{:?}", err);
                                    assert!(false);
                                }
                            }
                        }
                        Err(err) => {
                            error!("获取连接事变：err:{:?}", err);
                            assert!(false)
                        }
                    }
                }
                info!("空闲连接数{}", pool.idle_conn_size())
            }
            Err(err) => {
                error!("err:{:?}", err);
                assert!(false)
            }
        }
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(100)).await;
        }
    }
}
