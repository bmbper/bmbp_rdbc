use std::sync::Arc;

use crate::client::mysql::MysqlDbClient;
use crate::client::pg::PgDbClient;
use crate::client::sqlite::SqliteDbClient;
use crate::err::{RdbcError, RdbcErrorType, RdbcResult};
use crate::pool::RdbcConnInner;
use crate::RdbcDataBaseDriver::*;
use crate::RdbcDataSource;

mod mysql;
mod pg;
mod sqlite;
mod sql;

pub async fn build_conn(
    ds: Arc<RdbcDataSource>,
) -> RdbcResult<Box<dyn RdbcConnInner + Send + Sync + 'static>> {
    return match ds.driver() {
        Mysql => build_mysql_conn(ds.clone()).await,
        Postgresql => build_postgres_conn(ds.clone()).await,
        Sqlite => build_sqlite_conn(ds.clone()).await,
        _ => Err(RdbcError::new(
            RdbcErrorType::NotSupportDatabase,
            "不支持的数据库类型",
        )),
    };
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
