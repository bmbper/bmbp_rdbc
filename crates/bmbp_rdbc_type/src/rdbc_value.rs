use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RdbcValue {
    Char(char),
    Varchar(String),
    Text(String),
    LongText(String),
    SmallInt(i8),
    Int(i32),
    BigInt(i64),
    Double(f32),
    BigDouble(f64),
    Date(chrono::NaiveDate),
    DateTime(chrono::NaiveDateTime),
    Time(chrono::NaiveTime),
    TimeStamp(u64),
    Bytes(Vec<u8>),
    Boolean(bool),
    Array(Vec<RdbcValue>),
    Object(HashMap<String, RdbcValue>),
    Null,
}
impl RdbcValue {
    pub fn is_object(&self) -> bool {
        match self {
            RdbcValue::Object(_) => true,
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            RdbcValue::Array(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            RdbcValue::Char(_)
            | RdbcValue::Varchar(_)
            | RdbcValue::Text(_)
            | RdbcValue::LongText(_) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            RdbcValue::SmallInt(_)
            | RdbcValue::Int(_)
            | RdbcValue::BigInt(_)
            | RdbcValue::Double(_)
            | RdbcValue::BigDouble(_) => true,
            _ => false,
        }
    }
    pub fn is_datetime(&self) -> bool {
        match self {
            RdbcValue::DateTime(_) => true,
            _ => false,
        }
    }
    pub fn is_date(&self) -> bool {
        match self {
            RdbcValue::Date(_) => true,
            _ => false,
        }
    }
    pub fn is_time(&self) -> bool {
        match self {
            RdbcValue::Time(_) => true,
            _ => false,
        }
    }
    pub fn is_bytes(&self) -> bool {
        match self {
            RdbcValue::Bytes(_) => true,
            _ => false,
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            RdbcValue::Null => true,
            _ => false,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, RdbcValue>> {
        match self {
            RdbcValue::Object(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&Vec<RdbcValue>> {
        match self {
            RdbcValue::Array(v) => Some(v),
            _ => None,
        }
    }
}

impl Display for RdbcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RdbcValue::Char(v) => v.to_string(),
            RdbcValue::Varchar(v) => v.to_string(),
            RdbcValue::Text(v) => v.to_string(),
            RdbcValue::LongText(v) => v.to_string(),
            RdbcValue::SmallInt(v) => v.to_string(),
            RdbcValue::Int(v) => v.to_string(),
            RdbcValue::BigInt(v) => v.to_string(),
            RdbcValue::Double(v) => v.to_string(),
            RdbcValue::BigDouble(v) => v.to_string(),
            RdbcValue::Date(v) => v.to_string(),
            RdbcValue::DateTime(v) => v.to_string(),
            RdbcValue::Time(v) => v.to_string(),
            RdbcValue::TimeStamp(v) => v.to_string(),
            RdbcValue::Bytes(v) => String::from_utf8(v.to_vec()).unwrap_or("".to_string()),
            RdbcValue::Boolean(v) => v.to_string(),
            RdbcValue::Array(v) => match serde_json::to_string(v) {
                Ok(v_s) => v_s.clone(),
                Err(e) => e.to_string(),
            },
            RdbcValue::Object(v) => match serde_json::to_string(v) {
                Ok(v_s) => v_s.clone(),
                Err(e) => e.to_string(),
            },
            RdbcValue::Null => "NULL".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<char> for RdbcValue {
    fn from(value: char) -> Self {
        RdbcValue::Char(value)
    }
}
impl From<&char> for RdbcValue {
    fn from(value: &char) -> Self {
        RdbcValue::Char(*value)
    }
}
impl From<Option<char>> for RdbcValue {
    fn from(value: Option<char>) -> Self {
        match value {
            Some(v) => RdbcValue::Char(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&char>> for RdbcValue {
    fn from(value: Option<&char>) -> Self {
        match value {
            Some(v) => RdbcValue::Char(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<char>> for RdbcValue {
    fn from(value: &Option<char>) -> Self {
        match value {
            Some(v) => RdbcValue::Char(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<String> for RdbcValue {
    fn from(value: String) -> Self {
        if value.len() < 4096 {
            return RdbcValue::Varchar(value);
        }
        if value.len() < 65535 {
            return RdbcValue::Text(value);
        }
        RdbcValue::LongText(value)
    }
}
impl From<&String> for RdbcValue {
    fn from(value: &String) -> Self {
        if value.len() < 4096 {
            return RdbcValue::Varchar(value.to_string());
        }
        if value.len() < 65535 {
            return RdbcValue::Text(value.to_string());
        }
        RdbcValue::LongText(value.to_string())
    }
}
impl From<Option<String>> for RdbcValue {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => {
                if v.len() < 4096 {
                    return RdbcValue::Varchar(v.to_string());
                }
                if v.len() < 65535 {
                    return RdbcValue::Text(v.to_string());
                }
                RdbcValue::LongText(v.to_string())
            }
            None => RdbcValue::Null,
        }
    }
}

impl From<Option<&String>> for RdbcValue {
    fn from(value: Option<&String>) -> Self {
        match value {
            Some(v) => {
                if v.len() < 4096 {
                    return RdbcValue::Varchar(v.to_string());
                }
                if v.len() < 65535 {
                    return RdbcValue::Text(v.to_string());
                }
                RdbcValue::LongText(v.to_string())
            }
            None => RdbcValue::Null,
        }
    }
}

impl From<&Option<String>> for RdbcValue {
    fn from(value: &Option<String>) -> Self {
        match value {
            Some(v) => {
                if v.len() < 4096 {
                    return RdbcValue::Varchar(v.to_string());
                }
                if v.len() < 65535 {
                    return RdbcValue::Text(v.to_string());
                }
                RdbcValue::LongText(v.to_string())
            }
            None => RdbcValue::Null,
        }
    }
}

impl From<&str> for RdbcValue {
    fn from(v: &str) -> Self {
        if v.len() < 4096 {
            return RdbcValue::Varchar(v.to_string());
        }
        if v.len() < 65535 {
            return RdbcValue::Text(v.to_string());
        }
        RdbcValue::LongText(v.to_string())
    }
}
impl From<Option<&str>> for RdbcValue {
    fn from(v: Option<&str>) -> Self {
        match v {
            Some(v) => {
                if v.len() < 4096 {
                    return RdbcValue::Varchar(v.to_string());
                }
                if v.len() < 65535 {
                    return RdbcValue::Text(v.to_string());
                }
                RdbcValue::LongText(v.to_string())
            }
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<&str>> for RdbcValue {
    fn from(value: &Option<&str>) -> Self {
        match value {
            Some(v) => {
                if v.len() < 4096 {
                    return RdbcValue::Varchar(v.to_string());
                }
                if v.len() < 65535 {
                    return RdbcValue::Text(v.to_string());
                }
                RdbcValue::LongText(v.to_string())
            }
            None => RdbcValue::Null,
        }
    }
}

impl From<i8> for RdbcValue {
    fn from(value: i8) -> Self {
        RdbcValue::SmallInt(value)
    }
}
impl From<&i8> for RdbcValue {
    fn from(value: &i8) -> Self {
        RdbcValue::SmallInt(value.clone())
    }
}
impl From<Option<i8>> for RdbcValue {
    fn from(value: Option<i8>) -> Self {
        match value {
            Some(v) => RdbcValue::SmallInt(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&i8>> for RdbcValue {
    fn from(value: Option<&i8>) -> Self {
        match value {
            Some(v) => RdbcValue::SmallInt(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<i8>> for RdbcValue {
    fn from(value: &Option<i8>) -> Self {
        match value {
            Some(v) => RdbcValue::SmallInt(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<i16> for RdbcValue {
    fn from(value: i16) -> Self {
        RdbcValue::Int(value as i32)
    }
}
impl From<&i16> for RdbcValue {
    fn from(value: &i16) -> Self {
        RdbcValue::Int(value.clone() as i32)
    }
}
impl From<Option<i16>> for RdbcValue {
    fn from(value: Option<i16>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&i16>> for RdbcValue {
    fn from(value: Option<&i16>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<i16>> for RdbcValue {
    fn from(value: &Option<i16>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}

impl From<i32> for RdbcValue {
    fn from(value: i32) -> Self {
        RdbcValue::Int(value)
    }
}
impl From<&i32> for RdbcValue {
    fn from(value: &i32) -> Self {
        RdbcValue::Int(value.clone())
    }
}
impl From<Option<i32>> for RdbcValue {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&i32>> for RdbcValue {
    fn from(value: Option<&i32>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<i32>> for RdbcValue {
    fn from(value: &Option<i32>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<i64> for RdbcValue {
    fn from(value: i64) -> Self {
        RdbcValue::BigInt(value)
    }
}
impl From<&i64> for RdbcValue {
    fn from(value: &i64) -> Self {
        RdbcValue::BigInt(value.clone())
    }
}
impl From<Option<i64>> for RdbcValue {
    fn from(value: Option<i64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&i64>> for RdbcValue {
    fn from(value: Option<&i64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<i64>> for RdbcValue {
    fn from(value: &Option<i64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<isize> for RdbcValue {
    fn from(value: isize) -> Self {
        RdbcValue::BigInt(value as i64)
    }
}
impl From<&isize> for RdbcValue {
    fn from(value: &isize) -> Self {
        RdbcValue::BigInt(value.clone() as i64)
    }
}
impl From<Option<isize>> for RdbcValue {
    fn from(value: Option<isize>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&isize>> for RdbcValue {
    fn from(value: Option<&isize>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<isize>> for RdbcValue {
    fn from(value: &Option<isize>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<Option<u8>> for RdbcValue {
    fn from(value: Option<u8>) -> Self {
        match value {
            Some(v) => RdbcValue::SmallInt(v as i8),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&u8>> for RdbcValue {
    fn from(value: Option<&u8>) -> Self {
        match value {
            Some(v) => RdbcValue::SmallInt(v.clone() as i8),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<u8>> for RdbcValue {
    fn from(value: &Option<u8>) -> Self {
        match value {
            Some(v) => RdbcValue::SmallInt(v.clone() as i8),
            None => RdbcValue::Null,
        }
    }
}

impl From<u16> for RdbcValue {
    fn from(value: u16) -> Self {
        RdbcValue::Int(value as i32)
    }
}
impl From<&u16> for RdbcValue {
    fn from(value: &u16) -> Self {
        RdbcValue::Int(value.clone() as i32)
    }
}
impl From<Option<u16>> for RdbcValue {
    fn from(value: Option<u16>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&u16>> for RdbcValue {
    fn from(value: Option<&u16>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<u16>> for RdbcValue {
    fn from(value: &Option<u16>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}

impl From<u32> for RdbcValue {
    fn from(value: u32) -> Self {
        RdbcValue::Int(value as i32)
    }
}
impl From<&u32> for RdbcValue {
    fn from(value: &u32) -> Self {
        RdbcValue::Int(value.clone() as i32)
    }
}
impl From<Option<u32>> for RdbcValue {
    fn from(value: Option<u32>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&u32>> for RdbcValue {
    fn from(value: Option<&u32>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<u32>> for RdbcValue {
    fn from(value: &Option<u32>) -> Self {
        match value {
            Some(v) => RdbcValue::Int(v.clone() as i32),
            None => RdbcValue::Null,
        }
    }
}

impl From<u64> for RdbcValue {
    fn from(value: u64) -> Self {
        RdbcValue::BigInt(value as i64)
    }
}
impl From<&u64> for RdbcValue {
    fn from(value: &u64) -> Self {
        RdbcValue::BigInt(value.clone() as i64)
    }
}
impl From<Option<u64>> for RdbcValue {
    fn from(value: Option<u64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&u64>> for RdbcValue {
    fn from(value: Option<&u64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<u64>> for RdbcValue {
    fn from(value: &Option<u64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<usize> for RdbcValue {
    fn from(value: usize) -> Self {
        RdbcValue::BigInt(value as i64)
    }
}
impl From<&usize> for RdbcValue {
    fn from(value: &usize) -> Self {
        RdbcValue::BigInt(value.clone() as i64)
    }
}
impl From<Option<usize>> for RdbcValue {
    fn from(value: Option<usize>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&usize>> for RdbcValue {
    fn from(value: Option<&usize>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<usize>> for RdbcValue {
    fn from(value: &Option<usize>) -> Self {
        match value {
            Some(v) => RdbcValue::BigInt(v.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<f32> for RdbcValue {
    fn from(value: f32) -> Self {
        RdbcValue::Double(value)
    }
}
impl From<&f32> for RdbcValue {
    fn from(value: &f32) -> Self {
        RdbcValue::Double(value.clone())
    }
}
impl From<Option<f32>> for RdbcValue {
    fn from(value: Option<f32>) -> Self {
        match value {
            Some(v) => RdbcValue::Double(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&f32>> for RdbcValue {
    fn from(value: Option<&f32>) -> Self {
        match value {
            Some(v) => RdbcValue::Double(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<f32>> for RdbcValue {
    fn from(value: &Option<f32>) -> Self {
        match value {
            Some(v) => RdbcValue::Double(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<f64> for RdbcValue {
    fn from(value: f64) -> Self {
        RdbcValue::BigDouble(value)
    }
}
impl From<&f64> for RdbcValue {
    fn from(value: &f64) -> Self {
        RdbcValue::BigDouble(value.clone())
    }
}
impl From<Option<f64>> for RdbcValue {
    fn from(value: Option<f64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigDouble(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&f64>> for RdbcValue {
    fn from(value: Option<&f64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigDouble(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<f64>> for RdbcValue {
    fn from(value: &Option<f64>) -> Self {
        match value {
            Some(v) => RdbcValue::BigDouble(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<chrono::NaiveDate> for RdbcValue {
    fn from(value: chrono::NaiveDate) -> Self {
        RdbcValue::Date(value)
    }
}
impl From<&chrono::NaiveDate> for RdbcValue {
    fn from(value: &chrono::NaiveDate) -> Self {
        RdbcValue::Date(value.clone())
    }
}
impl From<Option<chrono::NaiveDate>> for RdbcValue {
    fn from(value: Option<chrono::NaiveDate>) -> Self {
        match value {
            Some(v) => RdbcValue::Date(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&chrono::NaiveDate>> for RdbcValue {
    fn from(value: Option<&chrono::NaiveDate>) -> Self {
        match value {
            Some(v) => RdbcValue::Date(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<chrono::NaiveDate>> for RdbcValue {
    fn from(value: &Option<chrono::NaiveDate>) -> Self {
        match value {
            Some(v) => RdbcValue::Date(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<chrono::NaiveDateTime> for RdbcValue {
    fn from(value: chrono::NaiveDateTime) -> Self {
        RdbcValue::DateTime(value)
    }
}
impl From<&chrono::NaiveDateTime> for RdbcValue {
    fn from(value: &chrono::NaiveDateTime) -> Self {
        RdbcValue::DateTime(value.clone())
    }
}
impl From<Option<chrono::NaiveDateTime>> for RdbcValue {
    fn from(value: Option<chrono::NaiveDateTime>) -> Self {
        match value {
            Some(v) => RdbcValue::DateTime(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&chrono::NaiveDateTime>> for RdbcValue {
    fn from(value: Option<&chrono::NaiveDateTime>) -> Self {
        match value {
            Some(v) => RdbcValue::DateTime(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<chrono::NaiveDateTime>> for RdbcValue {
    fn from(value: &Option<chrono::NaiveDateTime>) -> Self {
        match value {
            Some(v) => RdbcValue::DateTime(v.clone()),
            None => RdbcValue::Null,
        }
    }
}

impl From<chrono::NaiveTime> for RdbcValue {
    fn from(value: chrono::NaiveTime) -> Self {
        RdbcValue::Time(value)
    }
}
impl From<&chrono::NaiveTime> for RdbcValue {
    fn from(value: &chrono::NaiveTime) -> Self {
        RdbcValue::Time(value.clone())
    }
}
impl From<Option<chrono::NaiveTime>> for RdbcValue {
    fn from(value: Option<chrono::NaiveTime>) -> Self {
        match value {
            Some(v) => RdbcValue::Time(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&chrono::NaiveTime>> for RdbcValue {
    fn from(value: Option<&chrono::NaiveTime>) -> Self {
        match value {
            Some(v) => RdbcValue::Time(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<chrono::NaiveTime>> for RdbcValue {
    fn from(value: &Option<chrono::NaiveTime>) -> Self {
        match value {
            Some(v) => RdbcValue::Time(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<Vec<u8>> for RdbcValue {
    fn from(value: Vec<u8>) -> Self {
        RdbcValue::Bytes(value)
    }
}
impl From<&Vec<u8>> for RdbcValue {
    fn from(value: &Vec<u8>) -> Self {
        RdbcValue::Bytes(value.to_vec())
    }
}
impl From<Option<Vec<u8>>> for RdbcValue {
    fn from(value: Option<Vec<u8>>) -> Self {
        match value {
            Some(v) => RdbcValue::Bytes(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<Vec<u8>>> for RdbcValue {
    fn from(value: &Option<Vec<u8>>) -> Self {
        match value {
            Some(v) => RdbcValue::Bytes(v.to_vec()),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&Vec<u8>>> for RdbcValue {
    fn from(value: Option<&Vec<u8>>) -> Self {
        match value {
            Some(v) => RdbcValue::Bytes(v.to_vec()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<&Vec<u8>>> for RdbcValue {
    fn from(value: &Option<&Vec<u8>>) -> Self {
        match value {
            Some(v) => RdbcValue::Bytes(v.to_vec()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&[u8]> for RdbcValue {
    fn from(value: &[u8]) -> Self {
        RdbcValue::Bytes(value.to_vec())
    }
}
impl From<Option<&[u8]>> for RdbcValue {
    fn from(value: Option<&[u8]>) -> Self {
        match value {
            Some(v) => RdbcValue::Bytes(v.to_vec()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<&[u8]>> for RdbcValue {
    fn from(value: &Option<&[u8]>) -> Self {
        match value {
            Some(v) => RdbcValue::Bytes(v.to_vec()),
            None => RdbcValue::Null,
        }
    }
}

impl From<bool> for RdbcValue {
    fn from(value: bool) -> Self {
        RdbcValue::Boolean(value)
    }
}
impl From<&bool> for RdbcValue {
    fn from(value: &bool) -> Self {
        RdbcValue::Boolean(value.clone())
    }
}
impl From<Option<bool>> for RdbcValue {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(v) => RdbcValue::Boolean(v),
            None => RdbcValue::Null,
        }
    }
}
impl From<Option<&bool>> for RdbcValue {
    fn from(value: Option<&bool>) -> Self {
        match value {
            Some(v) => RdbcValue::Boolean(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<bool>> for RdbcValue {
    fn from(value: &Option<bool>) -> Self {
        match value {
            Some(v) => RdbcValue::Boolean(v.clone()),
            None => RdbcValue::Null,
        }
    }
}
impl From<&Option<&bool>> for RdbcValue {
    fn from(value: &Option<&bool>) -> Self {
        match value {
            Some(v) => RdbcValue::Boolean(v.clone().clone()),
            None => RdbcValue::Null,
        }
    }
}

impl RdbcValue {
    pub fn time_from_i32(value: i32) -> Self {
        RdbcValue::TimeStamp(value as u64)
    }
    pub fn time_from_u32(value: u32) -> Self {
        RdbcValue::TimeStamp(value.clone() as u64)
    }
    pub fn time_from_u64(value: u64) -> Self {
        RdbcValue::TimeStamp(value)
    }
    pub fn time_from_i64(value: i64) -> Self {
        RdbcValue::TimeStamp(value.clone() as u64)
    }
    pub fn time_from_usize(value: usize) -> Self {
        RdbcValue::TimeStamp(value as u64)
    }
    pub fn time_from_isize(value: isize) -> Self {
        RdbcValue::TimeStamp(value.clone() as u64)
    }
}
