#[derive(Default, Clone, Debug)]
pub struct ConnectionConfig {
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: String,
}

impl ConnectionConfig {
    pub fn get_name(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn get_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            &self.username, &self.password, &self.ip, &self.port
        )
    }
}
