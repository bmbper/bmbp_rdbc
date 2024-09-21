#[derive(Debug)]
pub enum RdbcErrorType {
    TimeOut,
    NotSupportDatabase,
    ConnectError,
    SQLError,
    PrimaryRequired,
}

#[derive(Debug)]
pub struct RdbcError {
    typ: RdbcErrorType,
    msg: String,
}

impl RdbcError {
    pub fn new(typ: RdbcErrorType, msg: &str) -> Self {
        RdbcError {
            typ,
            msg: msg.to_string(),
        }
    }
}

impl RdbcError {
    pub fn get_msg(&self) -> String {
        self.msg.to_string()
    }
    pub fn get_type(&self) -> &RdbcErrorType {
        &self.typ
    }
}

pub type RdbcResult<T> = Result<T, RdbcError>;
