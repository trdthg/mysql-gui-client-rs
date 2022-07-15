use crate::apps::database::{Databases, Field, Tables};

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
        data: Databases,
    },
    Tables {
        conn: String,
        db: String,
        data: Tables,
    },
    DataRows {
        conn: String,
        db: String,
        table: String,
        datas: Box<Vec<Vec<DataCell>>>,
    },
}
