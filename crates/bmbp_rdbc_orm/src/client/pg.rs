use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tokio_postgres::types::ToSql;
use tokio_postgres::{connect, Client, NoTls};
use tracing::info;

use bmbp_rdbc_sql::{
    DeleteWrapper, InsertWrapper, PgRdbcSQLBuilder, QueryWrapper, RdbcSQLBuilder, UpdateWrapper,
};
use bmbp_rdbc_type::{RdbcOrmRow, RdbcValue};

use crate::err::{RdbcError, RdbcErrorType, RdbcResult};
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;

pub struct PgDbClient {
    data_source: Arc<RdbcDataSource>,
    client: RwLock<Client>,
}

impl PgDbClient {
    pub(crate) async fn new(data_source: Arc<RdbcDataSource>) -> RdbcResult<Self> {
        let url = Self::build_url(data_source.clone())?;
        match connect(url.as_str(), NoTls).await {
            Ok((client, conn)) => {
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                Ok(PgDbClient {
                    data_source: data_source.clone(),
                    client: RwLock::new(client),
                })
            }
            Err(e) => Err(RdbcError::new(RdbcErrorType::ConnectError, &e.to_string())),
        }
    }
    fn build_url(ds: Arc<RdbcDataSource>) -> RdbcResult<String> {
        Ok(format!(
            "postgresql://{}:{}@{}:{}/{}?connect_timeout={}",
            ds.get_username().as_ref().unwrap_or(&"".to_string()),
            ds.get_password().as_ref().unwrap_or(&"".to_string()),
            ds.get_host().as_ref().unwrap_or(&"".to_string()),
            ds.get_port().as_ref().unwrap_or(&5432),
            ds.get_database().as_ref().unwrap_or(&"PUBLIC".to_string()),
            ds.get_max_wait_time().unwrap_or(5_000),
        ))
    }
}

impl PgDbClient {
    async fn execute(&self, sql: &str, params: &[RdbcValue]) -> RdbcResult<u64> {
        info!("sql=>{}; \n params={:#?}", sql, params);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        match self
            .client
            .read()
            .await
            .execute(sql, pg_prams.as_slice())
            .await
        {
            Ok(row_count) => Ok(row_count),
            Err(e) => Err(RdbcError::new(RdbcErrorType::SQLError, &e.to_string())),
        }
    }
}

#[async_trait]
impl RdbcConnInner for PgDbClient {
    async fn valid(&self) -> RdbcResult<bool> {
        let test_url = "select 1";
        let valid_result = self
            .client
            .read()
            .await
            .execute(test_url, &[])
            .await
            .is_ok();
        Ok(valid_result)
    }

    async fn select_page_by_query(
        &self,
        page_no: usize,
        page_size: usize,
        query: &QueryWrapper,
    ) -> RdbcResult<(usize, Option<Vec<RdbcOrmRow>>)> {
        let (pg_sql, page_prams) = PgRdbcSQLBuilder::build_query_sql(query);
        let count_sql = format!("SELECT COUNT(1) AS count FROM ({}) ", pg_sql);
        let query_sql = format!(
            "SELECT * FROM ({}) OFFSET {} LIMIT {} ",
            pg_sql,
            ((page_no - 1) * page_size),
            page_size
        );
        let total_row = self
            .select_one_by_sql(count_sql.as_str(), page_prams.as_slice())
            .await?;
        let row_data = self
            .select_list_by_sql(query_sql.as_str(), page_prams)
            .await?;
        let mut row_total = 0usize;
        if let Some(total) = total_row {
            if let Some(tv) = total.get_data().get("count") {
                if let Some(row_count) = tv.get_usize() {
                    row_total = row_count;
                }
            }
        }
        Ok((row_total, row_data))
    }

    async fn select_list_by_query(
        &self,
        query: &QueryWrapper,
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        let (pg_sql, page_prams) = PgRdbcSQLBuilder::build_query_sql(query);
        self.select_list_by_sql(pg_sql.as_str(), page_prams).await
    }

    async fn select_one_by_query(&self, query: &QueryWrapper) -> RdbcResult<Option<RdbcOrmRow>> {
        let (sql, params) = PgRdbcSQLBuilder::build_query_sql(query);
        self.select_one_by_sql(sql.as_str(), params.as_slice())
            .await
    }
    async fn select_one_by_sql(
        &self,
        sql: &str,
        params: &[RdbcValue],
    ) -> RdbcResult<Option<RdbcOrmRow>> {
        info!("sql=>{}; \n params={:#?}", sql, params);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();

        match self
            .client
            .read()
            .await
            .query_opt(sql, pg_prams.as_slice())
            .await
        {
            Ok(row_op) => {
                if let Some(row) = row_op {
                    Ok(Some(RdbcOrmRow::from(row)))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(RdbcError::new(RdbcErrorType::SQLError, &e.to_string())),
        }
    }
    async fn select_list_by_sql(
        &self,
        query: &str,
        params: Vec<RdbcValue>,
    ) -> RdbcResult<Option<Vec<RdbcOrmRow>>> {
        info!("sql=>{}; \n params={:#?}", query, params);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        match self
            .client
            .read()
            .await
            .query(query, pg_prams.as_slice())
            .await
        {
            Ok(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    let orm_row = RdbcOrmRow::from(row);
                    list.push(orm_row);
                }
                Ok(Some(list))
            }
            Err(e) => Err(RdbcError::new(RdbcErrorType::SQLError, &e.to_string())),
        }
    }

    async fn execute_insert(&self, insert: &InsertWrapper) -> RdbcResult<u64> {
        let (sql, params) = PgRdbcSQLBuilder::build_insert_sql(insert);
        self.execute(sql.as_str(), params.as_slice()).await
    }

    async fn execute_update(&self, update: &UpdateWrapper) -> RdbcResult<u64> {
        let (sql, params) = PgRdbcSQLBuilder::build_update_sql(update);
        self.execute(sql.as_str(), params.as_slice()).await
    }

    async fn execute_delete(&self, delete: &DeleteWrapper) -> RdbcResult<u64> {
        let (sql, params) = PgRdbcSQLBuilder::build_delete_sql(delete);
        self.execute(sql.as_str(), params.as_slice()).await
    }
}
