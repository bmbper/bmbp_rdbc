use crate::client::mysql::MysqlDbClient;
use crate::client::pg::PgDbClient;
use crate::client::sqlite::SqliteDbClient;
use crate::err::{RdbcError, RdbcErrorType, RdbcResult};
use crate::pool::RdbcConnInner;
use crate::RdbcDataSource;
use bmbp_rdbc_type::RdbcDataBase;
use std::sync::Arc;

mod mysql;
mod pg;
mod sqlite;

pub async fn build_conn(
    ds: Arc<RdbcDataSource>,
) -> RdbcResult<Box<dyn RdbcConnInner + Send + Sync + 'static>> {
    match ds.get_typ() {
        RdbcDataBase::MySQL => build_mysql_conn(ds).await,
        RdbcDataBase::Postgres => build_postgres_conn(ds).await,
        RdbcDataBase::SQLLite => build_sqlite_conn(ds).await,
        _ => Err(RdbcError::new(
            RdbcErrorType::NotSupportDatabase,
            "未指定数据库类型",
        )),
    }
}

async fn build_sqlite_conn(
    datasource: Arc<RdbcDataSource>,
) -> RdbcResult<Box<dyn RdbcConnInner + Send + Sync + 'static>> {
    let conn = SqliteDbClient::new(datasource.clone()).await?;
    Ok(Box::new(conn))
}

async fn build_postgres_conn(
    datasource: Arc<RdbcDataSource>,
) -> RdbcResult<Box<dyn RdbcConnInner + Send + Sync + 'static>> {
    let conn = PgDbClient::new(datasource.clone()).await?;
    Ok(Box::new(conn))
}

async fn build_mysql_conn(
    datasource: Arc<RdbcDataSource>,
) -> RdbcResult<Box<dyn RdbcConnInner + Send + Sync + 'static>> {
    let conn = MysqlDbClient::new(datasource.clone()).await?;
    Ok(Box::new(conn))
}
