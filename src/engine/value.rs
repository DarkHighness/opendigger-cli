use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Date(chrono::NaiveDate),
    Timestamp(chrono::NaiveDateTime),
    Time(chrono::NaiveTime),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Text(value) => write!(f, "{}", value),
            Value::Date(value) => write!(f, "{}", value),
            Value::Timestamp(value) => write!(f, "{}", value),
            Value::Time(value) => write!(f, "{}", value),
            Value::Null => write!(f, "NULL"),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Text(value.to_string())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<chrono::NaiveDate> for Value {
    fn from(value: chrono::NaiveDate) -> Self {
        Value::Date(value)
    }
}

impl From<chrono::NaiveDateTime> for Value {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Value::Timestamp(value)
    }
}

impl From<chrono::NaiveTime> for Value {
    fn from(value: chrono::NaiveTime) -> Self {
        Value::Time(value)
    }
}

impl TryInto<i64> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int(value) => Ok(value),
            Value::Float(value) => Ok(value as i64),
            Value::Text(value) => value.parse().map_err(|_| "Value is not an integer"),
            Value::Timestamp(value) => Ok(value.timestamp()),
            _ => Err("Value is not an integer"),
        }
    }
}

impl TryInto<f64> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Int(value) => Ok(value as f64),
            Value::Float(value) => Ok(value),
            Value::Text(value) => value.parse().map_err(|_| "Value is not a float"),
            _ => Err("Value is not a float"),
        }
    }
}

pub fn translate_from_gluesql_value(value: gluesql::prelude::Value) -> Option<Value> {
    match value {
        gluesql::prelude::Value::Bool(value) => Some(Value::Bool(value)),
        gluesql::prelude::Value::I8(value) => Some(Value::Int(value as i64)),
        gluesql::prelude::Value::I16(value) => Some(Value::Int(value as i64)),
        gluesql::prelude::Value::I32(value) => Some(Value::Int(value as i64)),
        gluesql::prelude::Value::I64(value) => Some(Value::Int(value)),
        gluesql::prelude::Value::I128(_) => None,
        gluesql::prelude::Value::U8(value) => Some(Value::Int(value as i64)),
        gluesql::prelude::Value::F64(value) => Some(Value::Float(value)),
        gluesql::prelude::Value::Decimal(_) => None,
        gluesql::prelude::Value::Str(value) => Some(Value::Text(value)),
        gluesql::prelude::Value::Bytea(_) => None,
        gluesql::prelude::Value::Date(value) => Some(Value::Date(value)),
        gluesql::prelude::Value::Timestamp(value) => Some(Value::Timestamp(value)),
        gluesql::prelude::Value::Time(value) => Some(Value::Time(value)),
        gluesql::prelude::Value::Interval(_) => None,
        gluesql::prelude::Value::Uuid(_) => None,
        gluesql::prelude::Value::Map(_) => None,
        gluesql::prelude::Value::List(_) => None,
        gluesql::prelude::Value::Null => Some(Value::Null),
    }
}
