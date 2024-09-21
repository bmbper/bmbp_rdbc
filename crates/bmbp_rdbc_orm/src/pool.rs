use async_trait::async_trait;
use bmbp_rdbc_sql::{DeleteWrapper, InsertWrapper, QueryWrapper, UpdateWrapper};
use bmbp_rdbc_type::{RdbcDataSource, RdbcOrmRow, RdbcValue};
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use crate::client;
use crate::err::{RdbcError, RdbcErrorType, RdbcResult};

/// RdbcConnInner 定义数据库连接抽象
#[async_trait]
pub trait RdbcConnInner {
    async fn valid(&self) -> RdbcResult<bool> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn select_page_by_query(
        &self,
        _page_no: usize,
        _page_size: usize,
        _query: &QueryWrapper,
    ) -> RdbcResult<(usize, Option<Vec<RdbcOrmRow>>)> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn select_list_by_query(
        &self,
        _query: &QueryWrapper,
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn select_one_by_query(&self, _query: &QueryWrapper) -> RdbcResult<Option<RdbcOrmRow>> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn select_one_by_sql(
        &self,
        _sql: &str,
        _params: &[RdbcValue],
    ) -> RdbcResult<Option<RdbcOrmRow>> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn select_list_by_sql(
        &self,
        _query: &str,
        _params: Vec<RdbcValue>,
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn execute_insert(&self, _delete: &InsertWrapper) -> RdbcResult<u64> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn execute_update(&self, _delete: &UpdateWrapper) -> RdbcResult<u64> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
    async fn execute_delete(&self, _delete: &DeleteWrapper) -> RdbcResult<u64> {
        Err(RdbcError::new(RdbcErrorType::SQLError, "接口未实现"))
    }
}

/// RdbcTransConnInner 定义数据库事务连接抽象
#[async_trait]
pub trait RdbcTransConnInner {
    async fn valid(&self) -> bool;
}

#[async_trait]
pub trait RdbcPreparedStatementInner {}

///RdbcConn 定义数据库连接池链接
pub struct RdbcConn<'a> {
    datasource: Arc<RdbcDataSource>,
    pool: &'a RdbcConnPool,
    inner: Option<Box<dyn RdbcConnInner + Send + Sync + 'static>>,
}

///RdbcTransConn 定义数据库连接池链接
pub struct RdbcTransConn<'a> {
    datasource: Arc<RdbcDataSource>,
    pool: &'a RdbcConnPool,
    inner: Option<Box<dyn RdbcConnInner + Send + Sync + 'static>>,
}

impl<'a> RdbcConn<'a> {
    pub async fn valid(&self) -> RdbcResult<bool> {
        if let Some(con) = &self.inner {
            return con.valid().await;
        } else {
            return Ok(false);
        }
    }

    pub async fn select_page_by_query(
        &self,
        page_no: usize,
        page_size: usize,
        query: &QueryWrapper,
    ) -> RdbcResult<(usize, Option<Vec<RdbcOrmRow>>)> {
        if let Some(con) = &self.inner {
            con.select_page_by_query(page_no, page_size, query).await
        } else {
            Err(RdbcError::new(
                RdbcErrorType::ConnectError,
                "获取到有效的数据库连接",
            ))
        }
    }
    pub async fn select_list_by_query(
        &self,
        query: &QueryWrapper,
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        if let Some(con) = &self.inner {
            con.select_list_by_query(query).await
        } else {
            Err(RdbcError::new(
                RdbcErrorType::ConnectError,
                "获取到有效的数据库连接",
            ))
        }
    }
    pub async fn select_one_by_query(
        &self,
        query: &QueryWrapper,
    ) -> RdbcResult<Option<RdbcOrmRow>> {
        if let Some(con) = &self.inner {
            con.select_one_by_query(query).await
        } else {
            Err(RdbcError::new(
                RdbcErrorType::ConnectError,
                "获取到有效的数据库连接",
            ))
        }
    }
    pub async fn execute_insert(&self, insert: &InsertWrapper) -> RdbcResult<u64> {
        if let Some(con) = &self.inner {
            con.execute_insert(insert).await
        } else {
            Err(RdbcError::new(
                RdbcErrorType::ConnectError,
                "获取到有效的数据库连接",
            ))
        }
    }

    pub async fn execute_update(&self, update: &UpdateWrapper) -> RdbcResult<u64> {
        if let Some(con) = &self.inner {
            con.execute_update(update).await
        } else {
            Err(RdbcError::new(
                RdbcErrorType::ConnectError,
                "获取到有效的数据库连接",
            ))
        }
    }

    pub async fn execute_delete(&self, delete: &DeleteWrapper) -> RdbcResult<u64> {
        if let Some(con) = &self.inner {
            con.execute_delete(delete).await
        } else {
            Err(RdbcError::new(
                RdbcErrorType::ConnectError,
                "获取到有效的数据库连接",
            ))
        }
    }
}

impl<'a> Drop for RdbcConn<'a> {
    fn drop(&mut self) {
        if let Some(con) = self.inner.take() {
            self.pool.receive_conn(con);
        }
    }
}

/// RdbcConnPool 定义数据库连接池
pub struct RdbcConnPool {
    data_source: Arc<RdbcDataSource>,
    conn_size: RwLock<usize>,
    conn_map: RwLock<Vec<Box<dyn RdbcConnInner + Send + Sync + 'static>>>,
}

impl RdbcConnPool {
    pub fn new(data_source: Arc<RdbcDataSource>) -> RdbcConnPool {
        let pool = RdbcConnPool {
            data_source,
            conn_size: RwLock::new(0),
            conn_map: RwLock::new(vec![]),
        };
        tracing::info!(
            "创建数据库连接池=> 初始连接数: {} ,可用连接数：{} ",
            pool.conn_size().clone(),
            pool.conn_idle_size().clone()
        );
        pool
    }
}

impl RdbcConnPool {
    pub async fn init(&self) -> RdbcResult<()> {
        let ds = self.data_source.clone();
        tracing::info!(
            "初始化连接池 => 数据库类型：{:?}， 初始连接数: {} ",
            ds.get_typ(),
            ds.get_init_conn_size().unwrap_or(5),
        );
        let init_conn_size = ds.get_init_conn_size().unwrap_or(5);
        *self.conn_size.write().unwrap() = init_conn_size.clone();
        self.create_conn_by_size(init_conn_size).await?;
        tracing::info!(
            "数据库连接池初始化完成=> 初始连接数: {} ,可用连接数：{} ",
            self.conn_size().clone(),
            self.conn_idle_size().clone()
        );
        Ok(())
    }
    pub async fn get_conn(&self) -> RdbcResult<RdbcConn> {
        let mut conn_op = self.conn_map.write().unwrap().pop();
        let timer = Instant::now();
        while conn_op.is_none() {
            if self.conn_size() < self.data_source.get_max_conn_size().unwrap_or(10) {
                self.extend_conn_pool().await?;
            } else {
                let times = timer.elapsed().as_millis();
                let max_wait_time = self.data_source.get_max_wait_time().unwrap_or(1000) as u128;
                if times > max_wait_time {
                    tracing::info!("获取数据库连接超时，最大等待时间：{}ms", max_wait_time);
                    return Err(RdbcError::new(RdbcErrorType::TimeOut, "获取数据库连接超时"));
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            conn_op = self.conn_map.write().unwrap().pop();
        }
        let conn = conn_op.unwrap();
        Ok(RdbcConn {
            datasource: self.data_source.clone(),
            pool: self,
            inner: Some(conn),
        })
    }
    pub async fn valid(&self) -> RdbcResult<bool> {
        return match self.get_conn().await {
            Ok(conn) => conn.valid().await,
            Err(_) => Ok(false),
        };
    }
}

impl RdbcConnPool {
    fn conn_size(&self) -> usize {
        self.conn_size.read().unwrap().clone()
    }
    fn conn_used_size(&self) -> usize {
        self.conn_size.read().unwrap().clone() - self.conn_map.read().unwrap().len()
    }
    fn conn_idle_size(&self) -> usize {
        self.conn_map.read().unwrap().len()
    }

    async fn extend_conn_pool(&self) -> RdbcResult<()> {
        let ds = self.data_source.clone();
        let max_conn_size = ds.get_max_conn_size().unwrap_or(10);
        let mut grow_size = ds.get_grow_conn_size().unwrap_or(5);
        let conn_size = self.conn_size.read().unwrap().clone();
        if conn_size >= max_conn_size {
            tracing::info!(
                "数据库连接池=> 资源已满，最大连接数：{}，已用连接数：{} ， 空闲连接数：{}",
                self.conn_size().clone(),
                self.conn_used_size().clone(),
                self.conn_idle_size().clone()
            );
            return Ok(());
        }
        if conn_size + grow_size > max_conn_size {
            grow_size = max_conn_size - conn_size;
        }
        tracing::info!("数据库连接池=> 新创建连接数{}", grow_size);
        self.create_conn_by_size(grow_size).await?;
        *self.conn_size.write().unwrap() = conn_size + grow_size;
        Ok(())
    }

    async fn create_conn_by_size(&self, conn_size: usize) -> RdbcResult<()> {
        for _ in 0..conn_size {
            match client::build_conn(self.data_source.clone()).await {
                Ok(conn) => {
                    self.conn_map.write().unwrap().push(conn);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
    fn receive_conn(&self, conn: Box<dyn RdbcConnInner + Send + Sync + 'static>) {
        self.conn_map.write().unwrap().push(conn);
    }
}
