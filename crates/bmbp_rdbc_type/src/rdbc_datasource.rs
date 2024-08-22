use crate::RdbcDataBase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RdbcDataSource {
    typ: RdbcDataBase,
    host: Option<String>,
    port: Option<u32>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
    ignore_case: Option<bool>,
    use_ssl: Option<bool>,
    init_conn_size: Option<usize>,
    grow_conn_size: Option<usize>,
    max_conn_size: Option<usize>,
    max_idle_conn: Option<usize>,
    max_wait_time: Option<usize>,
    max_idle_time: Option<usize>,
}
impl RdbcDataSource {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_typ(&mut self, value: RdbcDataBase) -> &mut Self {
        self.typ = value;
        self
    }
    pub fn get_typ(&self) -> &RdbcDataBase {
        &self.typ
    }
    pub fn get_mut_typ(&mut self) -> &mut RdbcDataBase {
        &mut self.typ
    }
    pub fn set_host(&mut self, value: Option<String>) -> &mut Self {
        self.host = value;
        self
    }
    pub fn get_host(&self) -> &Option<String> {
        &self.host
    }
    pub fn get_mut_host(&mut self) -> &mut Option<String> {
        &mut self.host
    }
    pub fn set_port(&mut self, value: Option<u32>) -> &mut Self {
        self.port = value;
        self
    }
    pub fn get_port(&self) -> &Option<u32> {
        &self.port
    }
    pub fn get_mut_port(&mut self) -> &mut Option<u32> {
        &mut self.port
    }
    pub fn set_username(&mut self, value: Option<String>) -> &mut Self {
        self.username = value;
        self
    }
    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }
    pub fn get_mut_username(&mut self) -> &mut Option<String> {
        &mut self.username
    }
    pub fn set_password(&mut self, value: Option<String>) -> &mut Self {
        self.password = value;
        self
    }
    pub fn get_password(&self) -> &Option<String> {
        &self.password
    }
    pub fn get_mut_password(&mut self) -> &mut Option<String> {
        &mut self.password
    }
    pub fn set_database(&mut self, value: Option<String>) -> &mut Self {
        self.database = value;
        self
    }
    pub fn get_database(&self) -> &Option<String> {
        &self.database
    }
    pub fn get_mut_database(&mut self) -> &mut Option<String> {
        &mut self.database
    }
    pub fn set_schema(&mut self, value: Option<String>) -> &mut Self {
        self.schema = value;
        self
    }
    pub fn get_schema(&self) -> &Option<String> {
        &self.schema
    }
    pub fn get_mut_schema(&mut self) -> &mut Option<String> {
        &mut self.schema
    }
    pub fn set_ignore_case(&mut self, value: Option<bool>) -> &mut Self {
        self.ignore_case = value;
        self
    }
    pub fn get_ignore_case(&self) -> &Option<bool> {
        &self.ignore_case
    }
    pub fn get_mut_ignore_case(&mut self) -> &mut Option<bool> {
        &mut self.ignore_case
    }
    pub fn set_use_ssl(&mut self, value: Option<bool>) -> &mut Self {
        self.use_ssl = value;
        self
    }
    pub fn get_use_ssl(&self) -> &Option<bool> {
        &self.use_ssl
    }
    pub fn get_mut_use_ssl(&mut self) -> &mut Option<bool> {
        &mut self.use_ssl
    }
    pub fn set_init_conn_size(&mut self, value: Option<usize>) -> &mut Self {
        self.init_conn_size = value;
        self
    }
    pub fn get_init_conn_size(&self) -> &Option<usize> {
        &self.init_conn_size
    }
    pub fn get_mut_init_conn_size(&mut self) -> &mut Option<usize> {
        &mut self.init_conn_size
    }
    pub fn set_grow_conn_size(&mut self, value: Option<usize>) -> &mut Self {
        self.grow_conn_size = value;
        self
    }
    pub fn get_grow_conn_size(&self) -> &Option<usize> {
        &self.grow_conn_size
    }
    pub fn get_mut_grow_conn_size(&mut self) -> &mut Option<usize> {
        &mut self.grow_conn_size
    }
    pub fn set_max_conn_size(&mut self, value: Option<usize>) -> &mut Self {
        self.max_conn_size = value;
        self
    }
    pub fn get_max_conn_size(&self) -> &Option<usize> {
        &self.max_conn_size
    }
    pub fn get_mut_max_conn_size(&mut self) -> &mut Option<usize> {
        &mut self.max_conn_size
    }
    pub fn set_max_idle_conn(&mut self, value: Option<usize>) -> &mut Self {
        self.max_idle_conn = value;
        self
    }
    pub fn get_max_idle_conn(&self) -> &Option<usize> {
        &self.max_idle_conn
    }
    pub fn get_mut_max_idle_conn(&mut self) -> &mut Option<usize> {
        &mut self.max_idle_conn
    }
    pub fn set_max_wait_time(&mut self, value: Option<usize>) -> &mut Self {
        self.max_wait_time = value;
        self
    }
    pub fn get_max_wait_time(&self) -> &Option<usize> {
        &self.max_wait_time
    }
    pub fn get_mut_max_wait_time(&mut self) -> &mut Option<usize> {
        &mut self.max_wait_time
    }
    pub fn set_max_idle_time(&mut self, value: Option<usize>) -> &mut Self {
        self.max_idle_time = value;
        self
    }
    pub fn get_max_idle_time(&self) -> &Option<usize> {
        &self.max_idle_time
    }
    pub fn get_mut_max_idle_time(&mut self) -> &mut Option<usize> {
        &mut self.max_idle_time
    }
}
