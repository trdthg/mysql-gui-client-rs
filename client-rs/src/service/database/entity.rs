#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            username: "tiangong2008".to_string(),
            password: "tiangong2008".to_string(),
            ip: "www.91iedu.com".to_string(),
            port: "3391".to_string(),
        }
    }
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
