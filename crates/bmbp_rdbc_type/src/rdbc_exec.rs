use crate::error::RdbcError;
use bmbp_rdbc_type::{RdbcPage, RdbcRow, RdbcValue};
use serde::Serialize;
use std::fmt::Debug;

pub trait Executor {
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
        Ok(0usize)
    }
    async fn execute_batch(
        &self,
        _execute_sql: String,
        _params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        Ok(0usize)
    }
    async fn execute_raw(&self, _execute_sql: String) -> Result<usize, RdbcError> {
        Ok(0usize)
    }
    async fn execute_batch_raw(&self, _execute_sql: &[String]) -> Result<usize, RdbcError> {
        Ok(0usize)
    }
    async fn execute_batch_slice(
        &self,
        _execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        Ok(0usize)
    }
}
