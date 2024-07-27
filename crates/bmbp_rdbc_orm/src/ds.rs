use crate::err::{RdbcError, RdbcErrorType, RdbcResult};

pub enum RdbcDataBaseDriver {
    Postgres,
    Mysql,
}

impl RdbcDataBaseDriver {
    pub fn value(&self) -> &str {
        match self {
            RdbcDataBaseDriver::Postgres => "Postgres",
            RdbcDataBaseDriver::Mysql => "Mysql",
        }
    }
    pub fn value_of(driver: String) -> RdbcResult<Self> {
        match driver.to_lowercase().as_str() {
            "postgres" => Ok(RdbcDataBaseDriver::Postgres),
            "mysql" => Ok(RdbcDataBaseDriver::Mysql),
            _ => Err(RdbcError::new(
                RdbcErrorType::NotSupportDatabase,
                "暂不支持的数据库类型",
            )),
        }
    }
}

pub struct RdbcDataSource {
    driver: RdbcDataBaseDriver,
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
    schema: String,
    ignore_case: bool,
    use_ssl: bool,
    init_conn_size: Option<usize>,
    grow_conn_size: Option<usize>,
    max_conn_size: Option<usize>,
    max_idle_conn: Option<usize>,
    max_wait_time: Option<usize>,
    max_idle_time: Option<usize>,
}

impl RdbcDataSource {
    pub fn new() -> Self {
        RdbcDataSource {
            driver: RdbcDataBaseDriver::Postgres,
            host: "".to_string(),
            port: 0,
            user: "".to_string(),
            password: "".to_string(),
            database: "".to_string(),
            schema: "".to_string(),
            ignore_case: false,
            use_ssl: false,

            init_conn_size: None,
            grow_conn_size: None,
            max_conn_size: None,
            max_idle_conn: None,
            max_wait_time: None,
            max_idle_time: None,
        }
    }
    pub fn driver(&self) -> &RdbcDataBaseDriver {
        &self.driver
    }
    pub fn set_driver(&mut self, driver: RdbcDataBaseDriver) -> &mut Self {
        self.driver = driver;
        self
    }
    pub fn host(&self) -> &String {
        &self.host
    }
    pub fn set_host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }
    pub fn port(&self) -> &u16 {
        &self.port
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    pub fn user(&self) -> &String {
        &self.user
    }
    pub fn set_user(&mut self, user: String) -> &mut Self {
        self.user = user;
        self
    }
    pub fn password(&self) -> &String {
        &self.password
    }
    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }
    pub fn database(&self) -> &String {
        &self.database
    }
    pub fn set_database(&mut self, database: String) -> &mut Self {
        self.database = database;
        self
    }
    pub fn schema(&self) -> &String {
        &self.schema
    }
    pub fn set_schema(&mut self, schema: String) -> &mut Self {
        self.schema = schema;
        self
    }
    pub fn ignore_case(&self) -> &bool {
        &self.ignore_case
    }
    pub fn set_ignore_case(&mut self, ignore_case: bool) -> &mut Self {
        self.ignore_case = ignore_case;
        self
    }
    pub fn use_ssl(&self) -> &bool {
        &self.use_ssl
    }
    pub fn set_use_ssl(&mut self, use_ssl: bool) -> &mut Self {
        self.use_ssl = use_ssl;
        self
    }
    pub fn init_conn_size(&self) -> &Option<usize> {
        &self.init_conn_size
    }
    pub fn set_init_conn_size(&mut self, init_conn_size: usize) -> &mut Self {
        self.init_conn_size = Some(init_conn_size);
        self
    }
    pub fn grow_conn_size(&self) -> &Option<usize> {
        &self.grow_conn_size
    }
    pub fn set_grow_conn_size(&mut self, grow_conn_size: usize) -> &mut Self {
        self.grow_conn_size = Some(grow_conn_size);
        self
    }
    pub fn max_conn_size(&self) -> &Option<usize> {
        &self.max_conn_size
    }

    pub fn set_max_conn_size(&mut self, max_conn_size: usize) -> &mut Self {
        self.max_conn_size = Some(max_conn_size);
        self
    }
    pub fn max_idle_conn(&self) -> &Option<usize> {
        &self.max_idle_conn
    }
    pub fn set_max_idle_conn(&mut self, max_idle_conn: usize) -> &mut Self {
        self.max_idle_conn = Some(max_idle_conn);
        self
    }
    pub fn max_wait_time(&self) -> &Option<usize> {
        &self.max_wait_time
    }
    pub fn set_max_wait_time(&mut self, max_wait_time: usize) -> &mut Self {
        self.max_wait_time = Some(max_wait_time);
        self
    }
    pub fn max_idle_time(&self) -> &Option<usize> {
        &self.max_idle_time
    }
    pub fn set_max_idle_time(&mut self, max_idle_time: usize) -> &mut Self {
        self.max_idle_time = Some(max_idle_time);
        self
    }
}
