use crate::ds::RdbcDbConfig;
use crate::pool::{RdbcPool, RdbcPoolBuilder};
use bmbp_rdbc_sql::{RdbcDelete, RdbcInsert, RdbcQuery, RdbcUpdate};
use bmbp_rdbc_type::{Executor, RdbcError, RdbcPage, RdbcRow, RdbcValue};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

pub struct RdbcOrm {}
impl RdbcOrm {
    pub async fn connect(db_config: RdbcDbConfig) -> Result<Arc<RdbcPool>, RdbcError> {
        RdbcPoolBuilder::connect(db_config).await
    }
}

impl RdbcOrm {
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

    pub async fn execute_batch_slice<E>(
        executor: &E,
        execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError>
    where
        E: Executor,
    {
        executor.execute_batch_slice(execute_sql_params).await
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
    pub async fn query_list_as<E, T>(executor: &E, query: &RdbcQuery) -> Result<Vec<T>, RdbcError>
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
