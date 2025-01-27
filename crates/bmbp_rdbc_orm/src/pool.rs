use crate::conn::RdbcConnection;
use crate::ds::RdbcDbConfig;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError};
use chrono::Duration;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
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
                10
            }
        };
        info!("初始化连接池大小：{}", pool_size);
        for i in 0..pool_size {
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
                        Ok(conn) => {
                            info!("获取连接成功");
                            info!("空闲连接数{}", pool.idle_conn_size())
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
