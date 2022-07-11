#[derive(Default, Clone, Debug)]
pub struct ConnectionConfig {
    pub name: String,
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: String,
    pub db: String,
}
