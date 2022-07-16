use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub enum DataType {
    // i8
    TinyInt,
    // i16
    SmallInt,
    // i32
    Integer,
    // i64
    BigInt,
    // carchar
    Varchar,
    // char(10)
    Char { width: u16 },
    // bool
    Boolean,
    // f32
    Float,
    // f64
    Double,
    // deciml(2,6)
    Decimal { scale: u16, precision: u16 },
    Date,
    Time,
    DateTime,
    TimeStamp,
    Unknown,
    Text,
}

#[derive(Debug, Clone)]
pub enum DataCell {
    // i8
    TinyInt(i8),
    // i16
    SmallInt(i16),
    // MiddleInt(i32),
    // i32
    Integer(i32),
    // i64
    BigInt(i64),
    // carchar
    Varchar(String),
    // char(10)
    Char(String),
    Text(String),
    // bool
    Boolean(bool),
    // f32
    Float(f32),
    // f64
    Double(f64),
    // deciml(2,6)
    Decimal(Decimal),
    Date(sqlx::types::chrono::NaiveDate),
    Time(sqlx::types::chrono::NaiveTime),
    DateTime(sqlx::types::chrono::NaiveDateTime),
    TimeStamp(chrono::DateTime<chrono::Utc>),
    Unknown(String),
    // Null = 0x06,
    // Year = 0x0d,
    // Bit = 0x10,
    // Json = 0xf5,
    // NewDecimal = 0xf6,
    // Enum = 0xf7,
    // Set = 0xf8,
    // TinyBlob = 0xf9,
    // MediumBlob = 0xfa,
    // LongBlob = 0xfb,
    // Blob = 0xfc,
    // VarString = 0xfd,
    // String = 0xfe,
    // Geometry = 0xff,
}

impl DataType {
    pub fn get_default_width(&self) -> f32 {
        match self {
            DataType::TinyInt => 50.,
            DataType::SmallInt => 50.,
            DataType::Integer => 60.,
            DataType::BigInt => 80.,
            DataType::Varchar => 100.,
            DataType::Char { width } => *width as f32 * 10.,
            DataType::Text => 180.,
            DataType::Boolean => 50.,
            DataType::Float => 50.,
            DataType::Double => 60.,
            DataType::Decimal { scale, precision } => (scale + precision) as f32 * 10.,
            DataType::Date => 80.,
            DataType::Time => 80.,
            DataType::DateTime => 120.,
            DataType::TimeStamp => 50.,
            DataType::Unknown => 50.,
        }
    }
    // |---------------------------------------|------------------------------------------------------|
    // | `bool`                                | TINYINT(1), BOOLEAN                                  |
    // | `i8`                                  | TINYINT                                              |
    // | `i16`                                 | SMALLINT                                             |
    // | `i32`                                 | INT                                                  |
    // | `i64`                                 | BIGINT                                               |
    // | `u8`                                  | TINYINT UNSIGNED                                     |
    // | `u16`                                 | SMALLINT UNSIGNED                                    |
    // | `u32`                                 | INT UNSIGNED                                         |
    // | `u64`                                 | BIGINT UNSIGNED                                      |
    // | `f32`                                 | FLOAT                                                |
    // | `f64`                                 | DOUBLE                                               |
    // | `&str`, [`String`]                    | VARCHAR, CHAR, TEXT                                  |
    // | `&[u8]`, `Vec<u8>`                    | VARBINARY, BINARY, BLOB                              |
    // |---------------------------------------|------------------------------------------------------|
    // | `chrono::DateTime<Utc>`               | TIMESTAMP                                            |
    // | `chrono::DateTime<Local>`             | TIMESTAMP                                            |
    // | `chrono::NaiveDateTime`               | DATETIME                                             |
    // | `chrono::NaiveDate`                   | DATE                                                 |
    // | `chrono::NaiveTime`                   | TIME                                                 |
    // |---------------------------------------|------------------------------------------------------|
    // | `time::PrimitiveDateTime`             | DATETIME                                             |
    // | `time::OffsetDateTime`                | TIMESTAMP                                            |
    // | `time::Date`                          | DATE                                                 |
    // | `time::Time`                          | TIME                                                 |
    // |---------------------------------------|------------------------------------------------------|
    // | `rust_decimal::Decimal`               | DECIMAL                                              |
    // |---------------------------------------|------------------------------------------------------|
    // | `uuid::Uuid`                          | BYTE(16), VARCHAR, CHAR, TEXT                        |
    // | `uuid::fmt::Hyphenated`               | CHAR(36)                                             |
    // |---------------------------------------|------------------------------------------------------|
    // | `json::JsonValue`             | JSON

    pub(crate) fn from_uppercase(name: &str) -> Self {
        match name {
            "BOOLEAN" => DataType::Boolean,
            "TINYINT" => DataType::TinyInt,
            "SMALLINT" => DataType::SmallInt,
            "INT" => DataType::Integer,
            "BIGINT" => DataType::BigInt,
            "FLOAT" => DataType::Float,
            "DOUBLE" => DataType::Double,
            "CHAR" => DataType::Char { width: 0 },
            "VARCHAR" => DataType::Varchar,
            "TEXT" => DataType::Text,
            "TEXT" => DataType::DateTime,
            "TEXT" => DataType::Date,
            "TEXT" => DataType::Time,
            "TEXT" => DataType::TimeStamp,
            "DECIMAL" => DataType::Decimal {
                scale: 0,
                precision: 0,
            },
            _ => DataType::Unknown,
        }
    }

    pub(crate) fn from_field(field: &FieldMeta) -> Self {
        match field.data_type.as_str() {
            "tinyint" => DataType::TinyInt,
            "smallint" => DataType::SmallInt,
            "int" => DataType::Integer,
            "bigint" => DataType::BigInt,

            "float" => DataType::Float,
            "double" => DataType::Double,

            "varchar" => DataType::Varchar,
            "char" => DataType::Char {
                width: field.character_maximum_length.unwrap(),
            },
            "text" => DataType::Text,
            "decimal" => DataType::Decimal {
                scale: field.numeric_precision.unwrap(),
                precision: field.numeric_scale.unwrap(),
            },
            "bool" | "bit" => DataType::Boolean,
            "date" => DataType::Date,
            "time" => DataType::Time,
            "datetime" => DataType::DateTime,
            "timestamp" => DataType::TimeStamp,
            _ => {
                tracing::error!("没有实现 {}", field.data_type.as_str());
                DataType::Unknown
            }
        }
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            DataType::TinyInt => "tinyint".to_string(),
            DataType::SmallInt => "smallint".to_string(),
            DataType::Integer => "int".to_string(),
            DataType::BigInt => "bigint".to_string(),

            DataType::Float => "float".to_string(),
            DataType::Double => "double".to_string(),

            DataType::Varchar => "varchar".to_string(),
            DataType::Char { width } => format!("char({})", width),
            DataType::Text => "text".to_string(),
            DataType::Decimal { scale, precision } => format!("decimal({}, {})", scale, precision),
            DataType::Boolean => "bool".to_string(),
            DataType::Date => "date".to_string(),
            DataType::Time => "time".to_string(),
            DataType::DateTime => "datetime".to_string(),
            DataType::TimeStamp => "timestamp".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
use sqlx::Row;

use super::sqls::FieldMeta;
impl DataCell {
    pub fn to_string(&self) -> String {
        match self {
            DataCell::BigInt(i) => i.to_string(),
            DataCell::TinyInt(i) => i.to_string(),
            DataCell::SmallInt(i) => i.to_string(),
            DataCell::Integer(i) => i.to_string(),
            DataCell::Varchar(i) => i.to_string(),
            DataCell::Char(i) => i.to_string(),
            DataCell::Text(i) => i.to_string(),
            DataCell::Boolean(i) => i.to_string(),
            DataCell::Float(i) => i.to_string(),
            DataCell::Double(i) => i.to_string(),
            DataCell::Decimal(i) => i.to_string(),
            DataCell::Date(i) => i.to_string(),
            DataCell::Time(i) => i.to_string(),
            DataCell::DateTime(i) => i.to_string(),
            DataCell::TimeStamp(i) => i.to_string(),
            DataCell::Unknown(i) => i.to_string(),
        }
    }

    pub fn from_mysql_row(
        mysql_row: &sqlx::mysql::MySqlRow,
        col: usize,
        field: &DataType,
    ) -> DataCell {
        let cell = match field {
            DataType::TinyInt => {
                let data: i8 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::TinyInt(data)
            }
            DataType::SmallInt => {
                let data: i16 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::SmallInt(data)
            }
            DataType::Integer => {
                let data: i32 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Integer(data)
            }
            DataType::BigInt => {
                let data: i64 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::BigInt(data)
            }
            DataType::Varchar | DataType::Text => {
                let data: String = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Varchar(data)
            }
            DataType::Char { .. } => {
                let data: String = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Char(data)
            }
            DataType::Boolean => {
                let data: bool = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Boolean(data)
            }
            DataType::Float => {
                let data: f32 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Float(data)
            }
            DataType::Double => {
                let data: f64 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Double(data)
            }
            DataType::Decimal { .. } => {
                let data: Decimal = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Decimal(data)
            }
            DataType::DateTime => {
                let data: Result<sqlx::types::chrono::NaiveDateTime, sqlx::Error> =
                    mysql_row.try_get(col);
                if let Ok(data) = data {
                    DataCell::DateTime(data)
                } else {
                    DataCell::Unknown("日期时间".to_string())
                }
            }
            DataType::Date => {
                let data: Result<sqlx::types::chrono::NaiveDate, sqlx::Error> =
                    mysql_row.try_get(col);
                if let Ok(data) = data {
                    DataCell::Date(data)
                } else {
                    DataCell::Unknown("日期".to_string())
                }
            }
            DataType::Time => {
                let data: Result<sqlx::types::chrono::NaiveTime, sqlx::Error> =
                    mysql_row.try_get(col);
                if let Ok(data) = data {
                    DataCell::Time(data)
                } else {
                    DataCell::Unknown("时间".to_string())
                }
            }
            DataType::TimeStamp => {
                let data: Result<chrono::DateTime<chrono::Utc>, sqlx::Error> =
                    mysql_row.try_get(col);
                if let Ok(data) = data {
                    DataCell::TimeStamp(data)
                } else {
                    DataCell::Unknown("时间戳".to_string())
                }
            }
            DataType::Unknown => DataCell::Unknown("未知类型".to_string()),
        };
        cell
    }
}
