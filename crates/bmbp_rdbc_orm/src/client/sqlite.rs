use std::sync::Arc;

use async_trait::async_trait;

use bmbp_rdbc_type::RdbcOrmRow;
use bmbp_rdbc_sql::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcValue, UpdateWrapper};

use crate::err::RdbcResult;
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;

pub struct SqliteDbClient {
    data_source: Arc<RdbcDataSource>,
}

impl SqliteDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> RdbcResult<Self> {
        Ok(SqliteDbClient {
            data_source: data_source.clone(),
        })
    }
}

#[async_trait]
impl RdbcConnInner for SqliteDbClient {
    async fn valid(&self) -> bool {
        return true;
    }

    async fn select_list_by_query(
        &self,
        query: &QueryWrapper,
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Ok(None)
    }
    async fn select_one_by_query(&self, query: &QueryWrapper) -> RdbcResult<Option<RdbcOrmRow>> {
        Ok(None)
    }
    async fn select_list_by_sql(
        &self,
        query: &str,
        params: &[RdbcValue],
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        Ok(None)
    }

    async fn execute_insert(&self, delete: &InsertWrapper) -> RdbcResult<u64> {
        Ok(0)
    }

    async fn execute_update(&self, delete: &UpdateWrapper) -> RdbcResult<u64> {
        Ok(0)
    }

    async fn execute_delete(&self, delete: &DeleteWrapper) -> RdbcResult<u64> {
        Ok(0)
    }
}
