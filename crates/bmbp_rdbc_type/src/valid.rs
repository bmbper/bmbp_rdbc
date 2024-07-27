use bmbp_rdbc_sql::RdbcValue;

pub trait RdbcValid {
    fn valid(&self, value: RdbcValue) -> Option<String>;
}

pub struct NotNone {
    msg: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct RdbcValidMeta {
    pub name: Option<String>,
    pub save: Vec<RdbcValidRule>,
    pub insert: Vec<RdbcValidRule>,
    pub update: Vec<RdbcValidRule>,
}

#[derive(Default, Debug, Clone)]
pub struct RdbcValidRule {
    pub _type: Option<String>,
    pub _msg: Option<String>,
}
