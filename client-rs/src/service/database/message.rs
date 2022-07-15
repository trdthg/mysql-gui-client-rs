use crate::apps::database::Field;

use super::{datatype::DataCell, entity::ConnectionConfig};

pub enum Message {
    Connect {
        config: ConnectionConfig,
        save: bool,
    },
    Select {
        conn: String,
        db: Option<String>,
        table: Option<String>,
        fields: Option<Box<Vec<Field>>>,
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
        conn: String,
        data: Vec<sqlx::mysql::MySqlRow>,
    },
    Tables {
        conn: String,
        db: String,
        data: Vec<sqlx::mysql::MySqlRow>,
    },
    DataRows {
        conn: String,
        db: String,
        table: String,
        datas: Box<Vec<Vec<DataCell>>>,
    },
}
