use serde::Serialize;
use std::fmt::Debug;
use bmbp_rdbc_type::{RdbcError, RdbcPage, RdbcRow, RdbcValue};
pub trait Executor where Self:Sized {
    async fn query_page(
        &self,
        _page_num: usize,
        _page_size: usize,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<RdbcPage<RdbcRow>, RdbcError>;
    async fn query_list(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Vec<RdbcRow>, RdbcError>;
    async fn query_one_option(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Option<RdbcRow>, RdbcError>;
    async fn query_page_as<T>(
        &self,
        _page_num: usize,
        _page_size: usize,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<RdbcPage<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone;
    async fn query_list_as<T>(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Vec<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone;
    async fn query_one_option_as<T>(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<Option<T>, RdbcError>
    where
        T: From<RdbcRow> + Debug + Default + Serialize + Clone;
    async fn execute(
        &self,
        _execute_sql: String,
        _params: &[RdbcValue],
    ) -> Result<usize, RdbcError>;
    async fn execute_batch(
        &self,
        _execute_sql: String,
        _params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError>;
    async fn execute_raw(&self, _execute_sql: String) -> Result<usize, RdbcError>;
    async fn execute_batch_raw(&self, _execute_sql: &[String]) -> Result<usize, RdbcError>;
    async fn execute_batch_slice(
        &self,
        _execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError>;
}