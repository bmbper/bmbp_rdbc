use crate::conn::RdbcTransaction;
use crate::ds::RdbcDbConfig;
use crate::orm::RdbcOrmExecutor;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError, RdbcPage, RdbcRow, RdbcValue};
use chrono::Duration;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, NoTls, Transaction};

pub struct RdbcPgConnection {
    id: String,
    db_config: Arc<RdbcDbConfig>,
    client: Client,
}

impl RdbcPgConnection {
    pub(crate) async fn connect(
        db_config: Arc<RdbcDbConfig>,
    ) -> Result<RdbcPgConnection, RdbcError> {
        let time_out = {
            if let Some(config) = &db_config.pool_config {
                config.wait_timeout
            } else {
                Duration::seconds(10)
            }
        };
        let config_str = format!(
            "host={} port={} user={} password={} dbname={} connect_timeout={}",
            &db_config.host,
            &db_config.port,
            &db_config.user,
            &db_config.password,
            &db_config.database_name,
            time_out.num_seconds(),
        );
        let connect_rs = tokio_postgres::connect(config_str.as_str(), NoTls).await;
        match connect_rs {
            Ok((client, pg_connection)) => {
                tokio::spawn(async {
                    if let Err(e) = pg_connection.await {
                        return Err(RdbcError::new(RdbcErrKind::CONNECTION, e.to_string()));
                    }
                    Ok(())
                });
                let conn = RdbcPgConnection {
                    id: uuid::Uuid::new_v4().to_string(),
                    db_config: db_config.clone(),
                    client,
                };
                Ok(conn)
            }
            Err(e) => Err(RdbcError::new(RdbcErrKind::CONNECTION, e.to_string())),
        }
    }

    pub async fn get_transaction(&mut self) -> Result<RdbcTransaction, RdbcError> {
        let mut client = &mut self.client;
        let trans_rs = client.transaction().await;
        if trans_rs.is_err() {
            return Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                trans_rs.err().unwrap().to_string(),
            ));
        }
        let pg_trans = RdbcPgTransaction {
            trans: Some(trans_rs.unwrap()),
        };
        Ok(RdbcTransaction::Pg(pg_trans))
    }
}

impl RdbcOrmExecutor for RdbcPgConnection {
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
        Ok(0)
    }

    async fn execute_batch(
        &self,
        _execute_sql: String,
        _params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_raw(&self, _execute_sql: String) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch_raw(&self, _execute_sql: &[String]) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch_slice(
        &self,
        _execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }
}

pub struct RdbcPgTransaction<'a> {
    trans: Option<Transaction<'a>>,
}
impl<'a> RdbcPgTransaction<'a> {
    pub async fn commit(&mut self) -> Result<usize, RdbcError> {
        if let Some(mut trans) = self.trans.take() {
            let commit_rs = trans.commit().await;
            if commit_rs.is_err() {
                return Err(RdbcError::new(
                    RdbcErrKind::CONNECTION,
                    commit_rs.err().unwrap().to_string(),
                ));
            }
        }
        Ok(0)
    }
}

impl RdbcOrmExecutor for RdbcPgTransaction<'_> {
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
        Ok(0)
    }

    async fn execute_batch(
        &self,
        _execute_sql: String,
        _params: &[&[RdbcValue]],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_raw(&self, _execute_sql: String) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch_raw(&self, _execute_sql: &[String]) -> Result<usize, RdbcError> {
        Ok(0)
    }

    async fn execute_batch_slice(
        &self,
        _execute_sql_params: &[(&String, &[&RdbcValue])],
    ) -> Result<usize, RdbcError> {
        Ok(0)
    }
}
