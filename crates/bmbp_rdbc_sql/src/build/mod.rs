use bmbp_rdbc_type::{RdbcDataBase};
pub use mysql::*;
pub use pg::*;
pub use crate::build::types::{RdbcSQLBuilder};

mod mysql;
mod pg;
mod types;

pub struct SQLBuilder {}

impl SQLBuilder {
    pub fn get_builder(rdbc_data_base: RdbcDataBase) -> Box<dyn RdbcSQLBuilder> {
        return match rdbc_data_base {
            RdbcDataBase::Postgres => Box::new(PgSqlBuilder {}),
            RdbcDataBase::MySQL => Box::new(MysqlBuilder {}),
        };
    }
}

