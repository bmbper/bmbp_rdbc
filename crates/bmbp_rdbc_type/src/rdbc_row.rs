use crate::RdbcValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// 定义返回值类型
/// RdbcOrmRow 数据库查询结果 实现各个数据库的FromRow
/// RdbcPage 分页返回值

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
pub struct RdbcRow {
    columns: Vec<String>,
    data: HashMap<String, RdbcValue>,
}

impl RdbcRow {
    pub fn new() -> Self {
        RdbcRow {
            columns: vec![],
            data: HashMap::new(),
        }
    }
    pub fn columns(&self) -> &Vec<String> {
        &self.columns
    }
    pub fn columns_mut(&mut self) -> &mut Vec<String> {
        &mut self.columns
    }
    pub fn data(&self) -> &HashMap<String, RdbcValue> {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut HashMap<String, RdbcValue> {
        &mut self.data
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RdbcPage<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcRow>,
{
    page_size: usize,
    page_num: usize,
    total: usize,
    data: Option<Vec<T>>,
}

impl<T> RdbcPage<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcRow>,
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
    T: Default + Debug + Clone + Serialize + From<RdbcRow>,
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
