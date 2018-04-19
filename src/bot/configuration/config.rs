

#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfig{
    telegram_apikey: String,
    db_config: DatabaseConfig,
}

impl ApplicationConfig{

    pub fn new(apikey: String,dbconfig: DatabaseConfig) -> ApplicationConfig{
        ApplicationConfig{telegram_apikey: apikey,db_config: dbconfig}
    }

    /// returns borrow of the telegram api key
    pub fn telegram_api_key(&self) -> &String{
        &self.telegram_apikey
    }

    /// returns an borrow on the DatabaseConfig
    pub fn db_conf(&self) -> &DatabaseConfig{
        &self.db_config
    }

}

/// Configuration for a Database connection
#[derive(Debug,Clone,Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig{
    port: u16,
    host: String,
    user: String,
    password: Option<String>,
    database: String,
}

impl DatabaseConfig{

    pub fn new(port: u16, host: String, user: String, password: Option<String>, database: String) -> DatabaseConfig{
        DatabaseConfig{port, host, user, password, database}
    }

    /// returns the port of the database
    pub fn port(&self) -> u16{
        self.port
    }

    /// Hostname/Address of the database
    pub fn host(&self) -> &String{
        &self.host
    }

    /// Returns the Database User
    pub fn user(&self) -> &String{
        &self.user
    }

    pub fn password(&self) -> Option<String>{
        match self.password {
            None => None,
            Some(ref p) => Some(p.clone()),
        }
    }

    /// returns the name of the database
    pub fn database(&self) -> &String{
        &self.database
    }
}

