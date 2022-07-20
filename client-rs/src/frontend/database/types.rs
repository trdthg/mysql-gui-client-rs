use crate::backend::database::{datatype::DataType, message};

use std::{
    cell::RefCell,
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Clone, Debug, Default)]
pub struct Conn {
    pub config: message::ConnectionConfig,
    pub conn: Option<usize>,
    pub databases: Option<Databases>,
}

#[derive(Clone, Debug, Default)]
pub struct Conns {
    inner: Rc<RefCell<ConnTree>>,
}

impl Deref for Conns {
    type Target = Rc<RefCell<ConnTree>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Conns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConnTree {
    inner: BTreeMap<String, Conn>,
}
impl Deref for ConnTree {
    type Target = BTreeMap<String, Conn>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ConnTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ConnTree {
    pub fn get_conn_mut(&mut self, conn: &str) -> Option<&mut Conn> {
        self.get_mut(conn)
    }

    pub fn get_db_mut(&mut self, conn: &str, db: &str) -> Option<&mut DB> {
        self.get_mut(conn)
            .and_then(|conn| conn.databases.as_mut())
            .and_then(|database| database.get_mut(db))
    }

    pub fn get_db(&self, conn: &str, db: &str) -> Option<&DB> {
        self.get(conn)
            .and_then(|conn| conn.databases.as_ref())
            .and_then(|database| database.get(db))
    }

    pub fn get_tables(&self, conn: &str, db: &str) -> Option<&Tables> {
        self.get_db(conn, db).and_then(|db| db.tables.as_ref())
    }

    pub fn get_fields(&self, conn: &str, db: &str, table: &str) -> Option<&Vec<Field>> {
        self.get_tables(conn, db)
            .and_then(|tables| tables.get(table))
    }
}
#[derive(Clone, Debug)]
pub struct DB {
    pub name: String,
    pub tables: Option<Tables>,
}

pub type Databases = BTreeMap<String, DB>;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub r#type: DataType,
    pub column_type: String,
    pub column_key: ColumnKey,
    pub is_nullable: bool,
}

impl Field {
    pub fn default_width(&self) -> f32 {
        self.r#type.get_default_width()
    }
}

#[derive(Debug, Clone)]
pub enum ColumnKey {
    Primary,
    Foreign,
    None,
}

pub type Tables = Box<BTreeMap<String, Vec<Field>>>;

pub type TableRows = Box<Vec<Vec<Option<String>>>>;
