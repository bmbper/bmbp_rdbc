use std::sync::{Arc, Mutex};
use tokio_postgres::Client;
use crate::ds::RdbcDbConfig;

pub struct RdbcPgTransaction {
    db_config: Arc<RdbcDbConfig>,
    client: Arc<Mutex<Client>>,
}