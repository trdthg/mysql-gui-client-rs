use crate::frontend::database::types::{Databases, Field, TableRows, Tables};

pub enum Request {
    Connect {
        config: ConnectionConfig,
        save: bool,
    },
    SelectDatabases {
        conn: String,
    },
    SelectTables {
        conn: String,
        db: String,
    },
    SelectTable {
        conn: String,
        db: String,
        table: String,
        page: usize,
        size: usize,
        fields: Box<Vec<Field>>,
        orders: Option<Box<Vec<Option<bool>>>>,
    },
    SelectCustomed {
        conn: String,
        sql: String,
    },
    Delete {
        conn: String,
        db: String,
        table: String,
        fields: Box<Vec<Field>>,
        datas: Box<Vec<Option<String>>>,
    },
    Insert {
        conn: String,
        db: String,
        table: String,
        fields: Box<Vec<Field>>,
        datas: Box<Vec<Option<String>>>,
    },
    Update {
        conn: String,
        db: String,
        table: String,
        fields: Box<Vec<Field>>,
        datas: Box<Vec<Option<String>>>,
        new_data_index: usize,
        new_data: Box<Option<String>>,
    },
}

pub enum Select {}
// orders: Option<Box<Vec<Option<bool>>>>,

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
        sql: String,
        table: String,
        datas: TableRows,
    },
    Customed {
        fields: Box<Vec<Field>>,
        datas: TableRows,
    },
    Delete {
        n: u64,
        msg: String,
        sql: String,
    },
    Insert {
        n: u64,
        msg: String,
        sql: String,
    },
    Update {
        n: u64,
        msg: String,
        sql: String,
    },
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectionConfig {
    pub username: String,
    pub password: String,
    pub ip: String,
    pub port: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
            ip: "".to_string(),
            port: "".to_string(),
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
