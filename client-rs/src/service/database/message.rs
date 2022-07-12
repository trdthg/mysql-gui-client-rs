use super::entity::ConnectionConfig;

pub enum Message {
    Connect {
        config: ConnectionConfig,
        save: bool,
    },
    Select {
        key: String,
        db: Option<String>,
        r#type: SelectType,
        sql: String,
    },
}

pub enum SelectType {
    Database,
}

pub enum Response {
    NewConn {
        config: ConnectionConfig,
        save: bool,
        result: Option<usize>,
    },
    Databases {
        key: String,
        data: Vec<sqlx::mysql::MySqlRow>,
    },
}
