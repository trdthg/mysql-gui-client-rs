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
        }
    }
}
