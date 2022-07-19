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

macro_rules! datatype_match_pattern {
    ($match_pattern:pat,  $datatype:ty, $scalar:ty) => {
        $match_pattern
    };
}
macro_rules! datatype_basictype {
    ($match_pattern:pat,  $datatype:ty, $scalar:ty) => {
        $scalar
    };
}
macro_rules! datatype_name {
    ($match_pattern:pat,  $datatype:ty, $scalar:ty) => {
        $datatype
    };
}
pub(crate) use datatype_basictype;
pub(crate) use datatype_match_pattern;
pub(crate) use datatype_name;

/// Association information for `Boolean` logical type.
macro_rules! boolean {
    ($macro:ident) => {
        $macro! {
            DataType::Boolean,
            Boolean,
            bool
        }
    };
}

pub(crate) use boolean;

macro_rules! int8 {
    ($macro:ident) => {
        $macro! {
            DataType::TinyInt,
            TinyInt,
            i8
        }
    };
}
pub(crate) use int8;

macro_rules! int16 {
    ($macro:ident) => {
        $macro! {
            DataType::SmallInt,
            SmallInt,
            i16
        }
    };
}
pub(crate) use int16;

macro_rules! int32 {
    ($macro:ident) => {
        $macro! {
            DataType::Integer,
            Integer,
            i32
        }
    };
}
pub(crate) use int32;

macro_rules! int64 {
    ($macro:ident) => {
        $macro! {
            DataType::BigInt,
            BigInt,
            i64
        }
    };
}
pub(crate) use int64;

macro_rules! varchar {
    ($macro:ident) => {
        $macro! {
            DataType::Varchar,
            Varchar,
            String
        }
    };
}
pub(crate) use varchar;

macro_rules! fwchar {
    ($macro:ident) => {
        $macro! {
            DataType::Char { .. },
            Char,
            String
        }
    };
}

macro_rules! text {
    ($macro:ident) => {
        $macro! {
            DataType::Text,
            Text,
            String
        }
    };
}

pub(crate) use fwchar;
macro_rules! float {
    ($macro:ident) => {
        $macro! {
            DataType::Float,
            Float,
            f32
        }
    };
}
pub(crate) use float;

macro_rules! double {
    ($macro:ident) => {
        $macro! {
            DataType::Double,
            Double,
            f64
        }
    };
}

pub(crate) use double;

/// Association information for `Decimal` logical type.
macro_rules! decimal {
    ($macro:ident) => {
        $macro! {
            DataType::Decimal { .. },
            Decimal,
            Decimal
        }
    };
}
pub(crate) use decimal;

macro_rules! date {
    ($macro:ident) => {
        $macro! {
            DataType::Date,
            Date,
            sqlx::types::chrono::NaiveDate
        }
    };
}
pub(crate) use date;

macro_rules! time {
    ($macro:ident) => {
        $macro! {
            DataType::Time,
            Time,
            sqlx::types::chrono::NaiveTime
        }
    };
}
pub(crate) use time;

macro_rules! datetime {
    ($macro:ident) => {
        $macro! {
            DataType::DateTime,
            Time,
            sqlx::types::chrono::NaiveDateTime
        }
    };
}
pub(crate) use datetime;

macro_rules! timestamp {
    ($macro:ident) => {
        $macro! {
            DataType::TimeStamp,
            Time,
            chrono::DateTime<chrono::Utc>
        }
    };
}
pub(crate) use timestamp;

macro_rules! unknown {
    ($macro:ident) => {
        $macro! {
            DataType::Unknown,
            Unknown,
            String
        }
    };
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
            "CHAR" => DataType::Char { width: 10 },
            "VARCHAR" => DataType::Varchar,
            "TEXT" => DataType::Text,
            "DATETIME" => DataType::DateTime,
            "DATE" => DataType::Date,
            "TIME" => DataType::Time,
            "TIMESTAMP" => DataType::TimeStamp,
            "DECIMAL" => DataType::Decimal {
                scale: 6,
                precision: 2,
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

macro_rules! datacell_to_string {
    ([], $({ $Variant:ident, $BasicType:ident }),*) => {
        #[derive(Debug, Clone)]
        pub enum DataCell {
            $(
                $BasicType(Option<$Variant! { datatype_basictype }>),
            )*
        }

        impl DataCell {
            pub fn to_string(&self) -> Option<String> {
                match self {
                    $(
                        // DataCell::$Variant(i) => i.as_ref().and_then(|x| Some(x.to_string())),
                        DataCell::$BasicType(i) => i.as_ref().and_then(|x| Some(x.to_string())),
                    )*
                }
            }

            pub fn from_mysql_row(
                mysql_row: &sqlx::mysql::MySqlRow,
                col: usize,
                field: &DataType,
                is_nullable: bool,
            ) -> DataCell {
                let cell = match field {
                    $(
                        // DataType::TinyInt => {
                        //     let data: Option<i8> = mysql_row.try_get(col).unwrap_or_default();
                        //     DataCell::TinyInt(data)
                        // }
                        $Variant! { datatype_match_pattern } => {
                            let data: Option<$Variant! { datatype_basictype } > = mysql_row.try_get(col).unwrap_or_default();
                            DataCell::$BasicType(data)
                        }
                    )*
                };
                cell
            }
        }
    };
}

macro_rules! get_all_datatype {
    ( $macro:ident ) => {
        $macro! {
            [],
            { int8, TinyInt },
            { int16, SmallInt },
            { int32, Integer },
            { int64, BigInt},
            { fwchar, Char},
            { varchar, Varchar},
            { text, Text},
            { boolean, Boolean},
            { float,  Float},
            { double, Double},
            { decimal, Decimal},
            { date, Date},
            { time, Time},
            { datetime, DateTime},
            { timestamp, TimeStamp},
            { unknown, Unknown}
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
    };
}

get_all_datatype!(datacell_to_string);
