use std::collections::HashMap;
use std::fmt::Debug;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;
use crate::RdbcValue;

/// 定义返回值类型
/// RdbcOrmRow 数据库查询结果 实现各个数据库的FromRow
/// RdbcPage 分页返回值

#[derive(Debug)]
pub struct RdbcOrmRow {
    columns: Vec<String>,
    data: HashMap<String, RdbcValue>,
}

impl RdbcOrmRow {
    pub fn new() -> Self {
        RdbcOrmRow {
            columns: vec![],
            data: HashMap::new(),
        }
    }
    pub fn get_columns(&self) -> &Vec<String> {
        &self.columns
    }
    pub fn get_columns_mut(&mut self) -> &mut Vec<String> {
        &mut self.columns
    }
    pub fn get_data(&self) -> &HashMap<String, RdbcValue> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut HashMap<String, RdbcValue> {
        &mut self.data
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcPage<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
{
    page_size: usize,
    page_num: usize,
    total: usize,
    data: Option<Vec<T>>,
}

impl<T> RdbcPage<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
{
    pub fn new() -> Self {
        RdbcPage {
            page_size: 10,
            page_num: 1,
            total: 0,
            data: None,
        }
    }
    pub fn new_with_page(page_size: usize, page_num: usize) -> Self {
        RdbcPage {
            page_size,
            page_num,
            total: 0,
            data: None,
        }
    }
}

impl<T> RdbcPage<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
{
    pub fn page_num(&self) -> &usize {
        &self.page_num
    }
    pub fn page_size(&self) -> &usize {
        &self.page_size
    }
    pub fn total(&self) -> &usize {
        &self.total
    }
    pub fn data(&self) -> &Option<Vec<T>> {
        &self.data
    }
    pub fn data_take(&mut self) -> Option<Vec<T>> {
        self.data.take()
    }
    pub fn set_page_num(&mut self, page_num: usize) -> &mut Self {
        self.page_num = page_num;
        self
    }
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }
    pub fn set_total(&mut self, total: usize) -> &mut Self {
        self.total = total;
        self
    }
    pub fn set_data(&mut self, data: Option<Vec<T>>) -> &mut Self {
        self.data = data;
        self
    }
}

impl From<Row> for RdbcOrmRow {
    fn from(row: Row) -> Self {
        let mut orm_row = RdbcOrmRow::new();
        let columns = row.columns();
        for col in columns {
            let col_name = col.name().to_string();
            orm_row.get_columns_mut().push(col_name.clone());
            let col_type = col.type_().name().to_string();
            match col_type.as_str() {
                "text" | "varchar" | "char" | "json" | "xml" => {
                    let col_value: Option<String> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::String(value));
                    }
                }
                "int2" => {
                    let col_value: Option<i16> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Int(value));
                    }
                }
                "int4" => {
                    let col_value: Option<i32> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::BigInt(value as i64));
                    }
                }
                "int8" => {
                    let col_value: Option<i64> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::BigInt(value));
                    }
                }
                "float4" | "float8" => {
                    let col_value: Option<f32> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Float(value));
                    }
                }
                "date" | "time" | "timestamp" => {
                    let col_value: Option<chrono::DateTime<chrono::Utc>> =
                        row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::DateTime(value));
                    }
                }
                "bool" => {
                    let col_value: Option<bool> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Bool(value));
                    }
                }
                _ => {
                    tracing::warn!("postgres数据库暂未支持的列类型: {:#?}", col_type);
                    orm_row.get_data_mut().insert(col_name, RdbcValue::Null);
                }
            }
        }
        orm_row
    }
}
