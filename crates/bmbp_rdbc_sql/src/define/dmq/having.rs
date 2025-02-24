use crate::{RdbcWhereFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdbcHaving {
    pub(crate) filter: Option<RdbcWhereFilter>,
}
