use crate::apps::database::Field;

use super::{datatype::DataCell, entity::ConnectionConfig};

pub enum Message {
    Connect {
        config: ConnectionConfig,
        save: bool,
    },
    Select {
        key: String,
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
        datas: Box<Vec<Vec<DataCell>>>,
    },
}
