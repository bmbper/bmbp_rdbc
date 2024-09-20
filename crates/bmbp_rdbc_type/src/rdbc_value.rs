use chrono::Utc;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum RdbcValue {
    Int(i16),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
    String(String),
    DateTime(chrono::DateTime<Utc>),
    Bool(bool),
    Vec(Vec<RdbcValue>),
    Map(HashMap<String, RdbcValue>),
    Null,
}

impl RdbcValue {
    pub fn get_string(&self) -> String {
        match self {
            RdbcValue::Int(i) => i.to_string(),
            RdbcValue::BigInt(i) => i.to_string(),
            RdbcValue::Float(i) => i.to_string(),
            RdbcValue::BigFloat(i) => i.to_string(),
            RdbcValue::String(i) => i.to_string(),
            RdbcValue::DateTime(i) => i.to_string(),
            RdbcValue::Bool(i) => i.to_string(),
            RdbcValue::Null => "".to_string(),
            RdbcValue::Vec(_) => "".to_string(),
            RdbcValue::Map(_) => "".to_string(),
        }
    }
    pub fn get_isize(&self) -> Option<isize> {
        match self {
            RdbcValue::Int(i) => Some(i.clone() as isize),
            RdbcValue::BigInt(i) => Some(i.clone() as isize),
            RdbcValue::Float(i) => Some(i.clone() as isize),
            RdbcValue::BigFloat(i) => Some(i.clone() as isize),
            _ => None,
        }
    }
    pub fn get_usize(&self) -> Option<usize> {
        match self {
            RdbcValue::Int(i) => Some(i.clone() as usize),
            RdbcValue::BigInt(i) => Some(i.clone() as usize),
            RdbcValue::Float(i) => Some(i.clone() as usize),
            RdbcValue::BigFloat(i) => Some(i.clone() as usize),
            _ => None,
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            RdbcValue::Null => true,
            _ => false,
        }
    }
    pub fn is_vec(&self) -> bool {
        match self {
            RdbcValue::Vec(_) => true,
            _ => false,
        }
    }
    pub fn is_map(&self) -> bool {
        match self {
            RdbcValue::Map(_) => true,
            _ => false,
        }
    }

    pub fn convert_to_vec(&self) -> Vec<RdbcValue> {
        match self {
            RdbcValue::Int(_) => {
                vec![self.clone()]
            }
            RdbcValue::BigInt(_) => {
                vec![self.clone()]
            }
            RdbcValue::Float(_) => {
                vec![self.clone()]
            }
            RdbcValue::BigFloat(_) => {
                vec![self.clone()]
            }
            RdbcValue::String(_) => {
                vec![self.clone()]
            }
            RdbcValue::DateTime(_) => {
                vec![self.clone()]
            }
            RdbcValue::Bool(_) => {
                vec![self.clone()]
            }
            RdbcValue::Vec(v) => v.clone(),
            RdbcValue::Map(mv) => {
                let mut v_vec = vec![];
                for v in mv.values() {
                    v_vec.push(v.clone());
                }
                v_vec
            }
            RdbcValue::Null => {
                vec![]
            }
        }
    }
}

impl Display for RdbcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.get_string();
        write!(f, "{}", str)
    }
}

impl From<i8> for RdbcValue {
    fn from(i: i8) -> RdbcValue {
        RdbcValue::Int(i as i16)
    }
}

impl From<u8> for RdbcValue {
    fn from(i: u8) -> RdbcValue {
        RdbcValue::Int(i as i16)
    }
}

impl From<i16> for RdbcValue {
    fn from(i: i16) -> RdbcValue {
        RdbcValue::Int(i)
    }
}

impl From<u16> for RdbcValue {
    fn from(i: u16) -> RdbcValue {
        RdbcValue::Int(i as i16)
    }
}

impl From<i32> for RdbcValue {
    fn from(i: i32) -> RdbcValue {
        RdbcValue::BigInt(i as i64)
    }
}

impl From<&i32> for RdbcValue {
    fn from(i: &i32) -> RdbcValue {
        RdbcValue::BigInt(i.clone() as i64)
    }
}

impl From<Option<i32>> for RdbcValue {
    fn from(s: Option<i32>) -> RdbcValue {
        match s {
            Some(s) => RdbcValue::BigInt(s as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<Option<&i32>> for RdbcValue {
    fn from(i: Option<&i32>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt(s.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<&Option<i32>> for RdbcValue {
    fn from(i: &Option<i32>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt(s.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<&Option<&i32>> for RdbcValue {
    fn from(i: &Option<&i32>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt((*s).clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<u32> for RdbcValue {
    fn from(i: u32) -> RdbcValue {
        RdbcValue::BigInt(i as i64)
    }
}

impl From<Option<u32>> for RdbcValue {
    fn from(s: Option<u32>) -> RdbcValue {
        match s {
            Some(s) => RdbcValue::BigInt(s as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<&Option<u32>> for RdbcValue {
    fn from(i: &Option<u32>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt(s.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<&u32> for RdbcValue {
    fn from(i: &u32) -> RdbcValue {
        RdbcValue::BigInt(i.clone() as i64)
    }
}

impl From<Option<&u32>> for RdbcValue {
    fn from(i: Option<&u32>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt(s.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<&Option<&u32>> for RdbcValue {
    fn from(i: &Option<&u32>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt((*s).clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<i64> for RdbcValue {
    fn from(i: i64) -> RdbcValue {
        RdbcValue::BigInt(i)
    }
}

impl From<u64> for RdbcValue {
    fn from(i: u64) -> RdbcValue {
        RdbcValue::BigInt(i as i64)
    }
}

impl From<usize> for RdbcValue {
    fn from(i: usize) -> RdbcValue {
        RdbcValue::BigInt(i as i64)
    }
}

impl From<Option<usize>> for RdbcValue {
    fn from(s: Option<usize>) -> RdbcValue {
        match s {
            Some(s) => RdbcValue::BigInt(s as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<&usize> for RdbcValue {
    fn from(i: &usize) -> RdbcValue {
        RdbcValue::BigInt(i.clone() as i64)
    }
}

impl From<Option<&usize>> for RdbcValue {
    fn from(i: Option<&usize>) -> RdbcValue {
        match i {
            Some(s) => RdbcValue::BigInt(s.clone() as i64),
            None => RdbcValue::Null,
        }
    }
}

impl From<String> for RdbcValue {
    fn from(s: String) -> RdbcValue {
        RdbcValue::String(s)
    }
}

impl From<&String> for RdbcValue {
    fn from(s: &String) -> RdbcValue {
        RdbcValue::String(s.to_string())
    }
}

impl From<Option<&String>> for RdbcValue {
    fn from(s: Option<&String>) -> RdbcValue {
        match s {
            Some(s) => RdbcValue::String(s.to_string()),
            None => RdbcValue::Null,
        }
    }
}

impl From<Option<String>> for RdbcValue {
    fn from(s: Option<String>) -> RdbcValue {
        match s {
            Some(s) => RdbcValue::String(s.to_string()),
            None => RdbcValue::Null,
        }
    }
}

impl From<&Option<String>> for RdbcValue {
    fn from(s: &Option<String>) -> RdbcValue {
        match s {
            Some(s) => RdbcValue::String(s.to_string()),
            None => RdbcValue::Null,
        }
    }
}

impl From<&str> for RdbcValue {
    fn from(s: &str) -> RdbcValue {
        RdbcValue::String(s.to_string())
    }
}

impl From<&RdbcValue> for String {
    fn from(value: &RdbcValue) -> Self {
        value.get_string()
    }
}

impl From<&RdbcValue> for bool {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Bool(s) => s.clone(),
            RdbcValue::Int(s) => s.clone() > 0,
            RdbcValue::BigInt(s) => s.clone() > 0,
            RdbcValue::Float(s) => s.clone() > 0.0,
            RdbcValue::BigFloat(s) => s.clone() > 0.0,
            RdbcValue::String(s) => s.clone() != "",
            RdbcValue::DateTime(_) => false,
            RdbcValue::Null => false,
            RdbcValue::Vec(v) => v.is_empty(),
            RdbcValue::Map(v) => v.is_empty(),
        }
    }
}

impl From<&RdbcValue> for i8 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as i8,
            RdbcValue::BigInt(s) => s.clone() as i8,
            RdbcValue::Float(s) => s.clone() as i8,
            RdbcValue::BigFloat(s) => s.clone() as i8,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for i16 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as i16,
            RdbcValue::BigInt(s) => s.clone() as i16,
            RdbcValue::Float(s) => s.clone() as i16,
            RdbcValue::BigFloat(s) => s.clone() as i16,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for i32 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as i32,
            RdbcValue::BigInt(s) => s.clone() as i32,
            RdbcValue::Float(s) => s.clone() as i32,
            RdbcValue::BigFloat(s) => s.clone() as i32,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for i64 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as i64,
            RdbcValue::BigInt(s) => s.clone() as i64,
            RdbcValue::Float(s) => s.clone() as i64,
            RdbcValue::BigFloat(s) => s.clone() as i64,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for i128 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as i128,
            RdbcValue::BigInt(s) => s.clone() as i128,
            RdbcValue::Float(s) => s.clone() as i128,
            RdbcValue::BigFloat(s) => s.clone() as i128,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for isize {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as isize,
            RdbcValue::BigInt(s) => s.clone() as isize,
            RdbcValue::Float(s) => s.clone() as isize,
            RdbcValue::BigFloat(s) => s.clone() as isize,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for u8 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as u8,
            RdbcValue::BigInt(s) => s.clone() as u8,
            RdbcValue::Float(s) => s.clone() as u8,
            RdbcValue::BigFloat(s) => s.clone() as u8,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for u16 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as u16,
            RdbcValue::BigInt(s) => s.clone() as u16,
            RdbcValue::Float(s) => s.clone() as u16,
            RdbcValue::BigFloat(s) => s.clone() as u16,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for u32 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as u32,
            RdbcValue::BigInt(s) => s.clone() as u32,
            RdbcValue::Float(s) => s.clone() as u32,
            RdbcValue::BigFloat(s) => s.clone() as u32,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for u64 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as u64,
            RdbcValue::BigInt(s) => s.clone() as u64,
            RdbcValue::Float(s) => s.clone() as u64,
            RdbcValue::BigFloat(s) => s.clone() as u64,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for u128 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as u128,
            RdbcValue::BigInt(s) => s.clone() as u128,
            RdbcValue::Float(s) => s.clone() as u128,
            RdbcValue::BigFloat(s) => s.clone() as u128,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for usize {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as usize,
            RdbcValue::BigInt(s) => s.clone() as usize,
            RdbcValue::Float(s) => s.clone() as usize,
            RdbcValue::BigFloat(s) => s.clone() as usize,
            _ => 0,
        }
    }
}

impl From<&RdbcValue> for f32 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as f32,
            RdbcValue::BigInt(s) => s.clone() as f32,
            RdbcValue::Float(s) => s.clone() as f32,
            RdbcValue::BigFloat(s) => s.clone() as f32,
            _ => 0.0,
        }
    }
}

impl From<&RdbcValue> for f64 {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => s.clone() as f64,
            RdbcValue::BigInt(s) => s.clone() as f64,
            RdbcValue::Float(s) => s.clone() as f64,
            RdbcValue::BigFloat(s) => s.clone() as f64,
            _ => 0.0,
        }
    }
}

impl From<&RdbcValue> for Option<i8> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as i8),
            RdbcValue::BigInt(s) => Some(s.clone() as i8),
            RdbcValue::Float(s) => Some(s.clone() as i8),
            RdbcValue::BigFloat(s) => Some(s.clone() as i8),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<i16> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as i16),
            RdbcValue::BigInt(s) => Some(s.clone() as i16),
            RdbcValue::Float(s) => Some(s.clone() as i16),
            RdbcValue::BigFloat(s) => Some(s.clone() as i16),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<i32> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as i32),
            RdbcValue::BigInt(s) => Some(s.clone() as i32),
            RdbcValue::Float(s) => Some(s.clone() as i32),
            RdbcValue::BigFloat(s) => Some(s.clone() as i32),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<i64> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as i64),
            RdbcValue::BigInt(s) => Some(s.clone() as i64),
            RdbcValue::Float(s) => Some(s.clone() as i64),
            RdbcValue::BigFloat(s) => Some(s.clone() as i64),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<i128> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as i128),
            RdbcValue::BigInt(s) => Some(s.clone() as i128),
            RdbcValue::Float(s) => Some(s.clone() as i128),
            RdbcValue::BigFloat(s) => Some(s.clone() as i128),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<isize> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as isize),
            RdbcValue::BigInt(s) => Some(s.clone() as isize),
            RdbcValue::Float(s) => Some(s.clone() as isize),
            RdbcValue::BigFloat(s) => Some(s.clone() as isize),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<u8> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as u8),
            RdbcValue::BigInt(s) => Some(s.clone() as u8),
            RdbcValue::Float(s) => Some(s.clone() as u8),
            RdbcValue::BigFloat(s) => Some(s.clone() as u8),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<u16> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as u16),
            RdbcValue::BigInt(s) => Some(s.clone() as u16),
            RdbcValue::Float(s) => Some(s.clone() as u16),
            RdbcValue::BigFloat(s) => Some(s.clone() as u16),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<u32> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as u32),
            RdbcValue::BigInt(s) => Some(s.clone() as u32),
            RdbcValue::Float(s) => Some(s.clone() as u32),
            RdbcValue::BigFloat(s) => Some(s.clone() as u32),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<u64> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as u64),
            RdbcValue::BigInt(s) => Some(s.clone() as u64),
            RdbcValue::Float(s) => Some(s.clone() as u64),
            RdbcValue::BigFloat(s) => Some(s.clone() as u64),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<u128> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as u128),
            RdbcValue::BigInt(s) => Some(s.clone() as u128),
            RdbcValue::Float(s) => Some(s.clone() as u128),
            RdbcValue::BigFloat(s) => Some(s.clone() as u128),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<usize> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as usize),
            RdbcValue::BigInt(s) => Some(s.clone() as usize),
            RdbcValue::Float(s) => Some(s.clone() as usize),
            RdbcValue::BigFloat(s) => Some(s.clone() as usize),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<f32> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as f32),
            RdbcValue::BigInt(s) => Some(s.clone() as f32),
            RdbcValue::Float(s) => Some(s.clone() as f32),
            RdbcValue::BigFloat(s) => Some(s.clone() as f32),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<f64> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Int(s) => Some(s.clone() as f64),
            RdbcValue::BigInt(s) => Some(s.clone() as f64),
            RdbcValue::Float(s) => Some(s.clone() as f64),
            RdbcValue::BigFloat(s) => Some(s.clone() as f64),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<bool> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::Bool(s) => Some(s.clone()),
            RdbcValue::Int(s) => Some(s.clone() != 0),
            RdbcValue::BigInt(s) => Some(s.clone() != 0),
            RdbcValue::Float(s) => Some(s.clone() != 0.0),
            RdbcValue::BigFloat(s) => Some(s.clone() != 0.0),
            RdbcValue::String(s) => Some(!s.clone().is_empty()),
            _ => None,
        }
    }
}

impl From<&RdbcValue> for Option<String> {
    fn from(value: &RdbcValue) -> Self {
        match value {
            RdbcValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}
