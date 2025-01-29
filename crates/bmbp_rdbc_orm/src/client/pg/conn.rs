use crate::ds::RdbcDbConfig;
use crate::trans::RdbcTransactionInner;
use bmbp_rdbc_type::{RdbcErrKind, RdbcError};
use chrono::Duration;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio_postgres::{Client, NoTls, Transaction};

pub struct RdbcPgConnection {
    id: String,
    db_config: Arc<RdbcDbConfig>,
    client: Client,
}

pub struct RdbcPgTransaction<'a> {
    trans: Transaction<'a>,
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
                    client: client,
                };
                Ok(conn)
            }
            Err(e) => Err(RdbcError::new(RdbcErrKind::CONNECTION, e.to_string())),
        }
    }

    pub async fn get_transaction(&mut self) -> Result<RdbcTransactionInner, RdbcError> {
        let mut client = &mut self.client;
        let trans_rs = client.transaction().await;
        if trans_rs.is_err() {
            return Err(RdbcError::new(
                RdbcErrKind::CONNECTION,
                trans_rs.err().unwrap().to_string(),
            ));
        }
        let pg_trans = RdbcPgTransaction {
            trans: trans_rs.unwrap(),
        };
        Ok(RdbcTransactionInner::Pg(pg_trans))
    }
}
