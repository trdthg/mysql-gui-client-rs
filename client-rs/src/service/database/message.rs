use super::entity::ConnectionConfig;

pub enum Message {
    Connect {
        config: ConnectionConfig,
        save: bool,
    },
    Select {
        key: String,
        db: Option<String>,
        table: Option<String>,
        r#type: SelectType,
        sql: String,
    },
}

pub enum SelectType {
    Databases,
    Tables,
    Table,
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
    Tables {
        key: String,
        db: String,
        data: Vec<sqlx::mysql::MySqlRow>,
    },
    DataRows {
        key: String,
        db: String,
        table: String,
        data: Box<Vec<sqlx::mysql::MySqlRow>>,
    },
}
