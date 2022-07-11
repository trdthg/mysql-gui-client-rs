#[derive(Default, Clone, Debug)]
pub struct ConnectionConfig {
    pub name: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: String,
    pub db: String,
}

impl ConnectionConfig {
    pub fn to_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            &self.username, &self.password, &self.ip, &self.port, &self.db
        )
    }
}
