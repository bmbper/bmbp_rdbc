use crate::ds::RdbcDbConfig;
use crate::pool::{RdbcPool};
use bmbp_rdbc_sql::{RdbcDelete, RdbcInsert, RdbcQuery, RdbcUpdate};
use bmbp_rdbc_type::{ RdbcError, RdbcPage, RdbcRow, RdbcValue};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;
use tokio_postgres::connect;
use crate::exec::Executor;

pub struct RdbcOrm {}
impl RdbcOrm {
    pub async fn connect(db_config: RdbcDbConfig) -> Result<Arc<RdbcPool>, RdbcError> {
        RdbcPool::connect(Arc::new(db_config)).await
    }
}

impl RdbcOrm {
    pub async fn query_page<E>(
        executor: &E,
        page_num: usize,
        page_size: usize,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<RdbcPage<RdbcRow>, RdbcError>
    where
        E: Executor,
    {
        executor
            .query_page(page_num, page_size, execute_sql, params)
            .await
    }
    pub async fn query_page_as<E, T>(
        executor: &E,
        page_num: usize,
        page_size: usize,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<RdbcPage<T>, RdbcError>
    where
        E: Executor,
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        executor
            .query_page_as(page_num, page_size, execute_sql, params)
            .await
    }
    pub async fn query_list<E>(
        executor: &E,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Vec<RdbcRow>, RdbcError>
    where
        E: Executor,
    {
        executor.query_list(execute_sql, params).await
    }
    pub async fn query_list_as<E, T>(
        executor: &E,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Vec<T>, RdbcError>
    where
        E: Executor,
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        executor.query_list_as(execute_sql, params).await
    }
    pub async fn query_one_option<E>(
        executor: &E,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Option<RdbcRow>, RdbcError>
    where
        E: Executor,
    {
        executor.query_one_option(execute_sql, params).await
    }
    pub async fn query_one_option_as<E, T>(
        executor: &E,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<Option<T>, RdbcError>
    where
        E: Executor,
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        executor.query_one_option_as(execute_sql, params).await
    }
    pub async fn execute<E>(
        executor: &E,
        execute_sql: String,
        params: &[RdbcValue],
    ) -> Result<usize, RdbcError>
    where
        E: Executor,
    {
        executor.execute(execute_sql, params).await
    }
    pub async fn execute_batch<E>(
        executor: &E,
        execute_sql: String,
        params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError>
    where
        E: Executor,
    {
        executor.execute_batch(execute_sql, params).await
    }
    pub async fn execute_batch_slice<E>(
        executor: &E,
        execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError>
    where
        E: Executor,
    {
        executor.execute_batch_slice(execute_sql_params).await
    }

    pub async fn execute_raw<E>(executor: &E, execute_sql: String) -> Result<usize, RdbcError>
    where
        E: Executor,
    {
        executor.execute_raw(execute_sql).await
    }
    pub async fn execute_batch_raw<E>(
        executor: &E,
        execute_sql: &[String],
    ) -> Result<usize, RdbcError>
    where
        E: Executor,
    {
        executor.execute_batch_raw(execute_sql).await
    }
}

impl RdbcOrm {
    pub async fn query_pag_by_query<E>(
        executor: &E,
        page_num: usize,
        page_size: usize,
        query: &RdbcQuery,
    ) -> Result<RdbcPage<RdbcRow>, RdbcError>
    where
        E: Executor,
    {
        executor
            .query_page(page_num, page_size, "".to_string(), &[])
            .await
    }
    pub async fn query_pag_by_query_as<E, T>(
        executor: &E,
        page_num: usize,
        page_size: usize,
        query: &RdbcQuery,
    ) -> Result<RdbcPage<T>, RdbcError>
    where
        E: Executor,
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        executor
            .query_page_as(page_num, page_size, "".to_string(), &[])
            .await
    }
    pub async fn query_list_by_query<E>(
        executor: &E,
        query: &RdbcQuery,
    ) -> Result<Vec<RdbcRow>, RdbcError>
    where
        E: Executor,
    {
        executor.query_list("".to_string(), &[]).await
    }
    pub async fn query_list_by_query_as<E, T>(executor: &E, query: &RdbcQuery) -> Result<Vec<T>, RdbcError>
    where
        E: Executor,
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        executor.query_list_as("".to_string(), &[]).await
    }
    pub async fn query_one_option_by_query<E>(
        executor: &E,
        query: &RdbcQuery,
    ) -> Result<Option<RdbcRow>, RdbcError>
    where
        E: Executor,
    {
        executor.query_one_option("".to_string(), &[]).await
    }
    pub async fn query_one_option_by_query_as<E, T>(
        executor: &E,
        query: &RdbcQuery,
    ) -> Result<Option<T>, RdbcError>
    where
        E: Executor,
        T: From<RdbcRow> + Debug + Default + Serialize + Clone,
    {
        executor.query_one_option_as("".to_string(), &[]).await
    }

    pub async fn execute_insert<E>(executor: &E, insert: &RdbcInsert) -> Result<usize, RdbcError> {
        Ok(0)
    }
    pub async fn execute_batch_insert<E>(executor: &E, insert: &[RdbcInsert]) -> Result<usize, RdbcError> {
        Ok(0)
    }
    pub async fn execute_update<E>(executor: &E, insert: &RdbcUpdate) -> Result<usize, RdbcError> {
        Ok(0)
    }
    pub async fn execute_batch_update<E>(executor: &E, insert: &[RdbcUpdate]) -> Result<usize, RdbcError> {
        Ok(0)
    }
    pub async fn execute_delete<E>(executor: &E, insert: &RdbcDelete) -> Result<usize, RdbcError> {
        Ok(0)
    }
    pub async fn execute_batch_delete<E>(executor: &E, insert: &[RdbcDelete]) -> Result<usize, RdbcError> {
        Ok(0)
    }
}


#[cfg(test)]
mod tests{
    use serde_json::to_string;
    use tracing::info;
    use tracing_subscriber::fmt;
    use bmbp_rdbc_sql::RdbcQuery;
    use bmbp_rdbc_type::RdbcError;
    use crate::ds::{RdbcDbConfig, RdbcDbType};
    use crate::orm::RdbcOrm;

    #[tokio::test]
    async fn test_orm()->Result<(),RdbcError>{
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
        let pool = RdbcOrm::connect(db_config).await?;
        info!("传递pool");
        let rs = RdbcOrm::query_list(&pool, "select * from bmbp_user".to_string(), &[]).await?;
        info!("pool查询记录数：{}",rs.len());
        info!("传递connection");
        let conn = pool.get_connection()?;
        let rs = RdbcOrm::query_list(&conn, "select * from bmbp_user".to_string(), &[]).await?;
        info!("connection查询记录数：{}",rs.len());
        Ok(())

    }
}
