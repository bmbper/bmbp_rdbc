#[derive(Debug)]
pub struct RdbcError {
    kind: RdbcErrKind,
    message: String,
}

#[derive(Debug)]
pub enum RdbcErrKind {
    CONNECTION,
}

impl RdbcError {
    pub fn new(kind: RdbcErrKind, message: String) -> Self {
        Self { kind, message }
    }
}

