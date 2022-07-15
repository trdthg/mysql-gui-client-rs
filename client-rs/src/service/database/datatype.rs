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
    Real,
    // f64
    Double,
    // deciml(2,6)
    Decimal { scale: u16, precision: u16 },
    Date,
    Time,
    DateTime,
    TimeStamp,
    Unknown,
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
    // bool
    Boolean(bool),
    // f32
    Real(f32),
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
            DataType::Boolean => 50.,
            DataType::Real => 50.,
            DataType::Double => 60.,
            DataType::Decimal { scale, precision } => (scale + precision) as f32 * 10.,
            DataType::Date => 80.,
            DataType::Time => 80.,
            DataType::DateTime => 120.,
            DataType::TimeStamp => 50.,
            DataType::Unknown => 50.,
        }
    }
}
use sqlx::Row;
impl DataCell {
    pub fn to_string(&self) -> String {
        match self {
            DataCell::BigInt(i) => i.to_string(),
            DataCell::TinyInt(i) => i.to_string(),
            DataCell::SmallInt(i) => i.to_string(),
            DataCell::Integer(i) => i.to_string(),
            DataCell::Varchar(i) => i.to_string(),
            DataCell::Char(i) => i.to_string(),
            DataCell::Boolean(i) => i.to_string(),
            DataCell::Real(i) => i.to_string(),
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
        field: &crate::apps::database::Field,
    ) -> DataCell {
        let cell = match field.datatype {
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
            DataType::Varchar => {
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
            DataType::Real => {
                let data: f32 = mysql_row.try_get(col).unwrap_or_default();
                DataCell::Real(data)
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
