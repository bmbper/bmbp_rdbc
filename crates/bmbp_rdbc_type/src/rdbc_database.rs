use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RdbcDataBase {
    #[default]
    Postgres,
    MySQL,
    SQLLite,
}
