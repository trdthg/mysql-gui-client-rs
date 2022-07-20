use crate::app::database::{Databases, Field, TableRows, Tables};

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

pub enum SelectType {
    Databases,
    Tables,
    Table,
    Customed,
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
