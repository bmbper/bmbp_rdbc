pub use crate::err::RdbcResult;
use async_static::async_static;
pub use bmbp_rdbc_sql::*;
pub use bmbp_rdbc_type::*;
pub use ds::*;
pub use err::*;
pub use orm::RdbcOrm;
pub use orm::*;

mod client;
mod ds;
mod err;
mod orm;
mod pool;
mod val;

async_static! {
    pub static ref RdbcOrmIns:RdbcOrm = build_orm().await;
}
async fn build_orm() -> RdbcOrm {
    let datasource = RdbcDataSource::new();
    match RdbcOrm::new(datasource).await {
        Ok(orm) => orm,
        Err(err) => {
            panic!("连接数据库失败:{:#?}", err);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{RdbcDataBaseDriver, RdbcDataSource, RdbcOrm};

    fn build_datasource() -> RdbcDataSource {
        let mut ds = RdbcDataSource::new();
        ds.set_driver(RdbcDataBaseDriver::Postgres);
        ds.set_host("127.0.0.1".to_string())
            .set_port(5432)
            .set_user("bmbp".to_string())
            .set_password("zgk0130!".to_string())
            .set_database("bmbp".to_string())
            .set_schema("public".to_string())
            .set_ignore_case(true);
        ds.set_init_conn_size(5)
            .set_max_conn_size(10)
            .set_max_idle_conn(1);
        ds
    }

    #[tokio::test]
    async fn test_rom() {
        let ds = build_datasource();
        if let Ok(orm) = RdbcOrm::new(ds).await {
            if let Ok(conn) = orm.get_conn().await {
                assert!(conn.valid().await);
            };
        }
    }
}
