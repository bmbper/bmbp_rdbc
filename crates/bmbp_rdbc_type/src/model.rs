use std::collections::HashMap;
use std::fmt::Debug;

use chrono::Utc;
use log::__private_api::Value;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use bmbp_rdbc_sql::{DeleteWrapper, InsertWrapper, QueryWrapper, RdbcTableWrapper, RdbcValue};

use crate::{
    RDBC_DATA_CREATE_TIME, RDBC_DATA_CREATE_USER, RDBC_DATA_FLAG, RDBC_DATA_ID, RDBC_DATA_LEVEL,
    RDBC_DATA_OWNER_ORG, RDBC_DATA_REMARK, RDBC_DATA_SIGN, RDBC_DATA_SORT, RDBC_DATA_STATUS,
    RDBC_DATA_UPDATE_TIME, RDBC_DATA_UPDATE_USER, RDBC_ENABLE, RDBC_NEW_FLAG, RDBC_TREE_CODE,
    RDBC_TREE_CODE_PATH, RDBC_TREE_NAME, RDBC_TREE_NAME_PATH, RDBC_TREE_NODE_LEAF,
    RDBC_TREE_NODE_LEVEL, RDBC_TREE_PARENT_CODE,
};

/// RdbcModel 定义数据库表标记
pub trait RdbcModel {
    fn get_table_name() -> String;
    fn get_table_fields() -> Vec<String>;
    fn get_table_primary_key() -> String {
        RDBC_DATA_ID.to_string()
    }
    fn get_table_union_primary_key() -> Vec<String> {
        return vec![];
    }
}

/// 定义平台提供的数据库表实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRdbcModel<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    /// 记录主键
    data_id: Option<String>,
    /// 记录密级
    data_level: Option<String>,
    /// 记录状态
    data_status: Option<String>,
    /// 记录标识
    data_flag: Option<String>,
    /// 记录显示顺序
    data_sort: Option<usize>,
    /// 记录备注
    data_remark: Option<String>,
    /// 记录创建时间
    data_create_time: Option<String>,
    /// 记录创建人
    data_create_user: Option<String>,
    /// 记录更新时间
    data_update_time: Option<String>,
    /// 记录更新用户
    data_update_user: Option<String>,
    /// 记录所属组织
    data_owner_org: Option<String>,
    /// 记录防串改标识
    data_sign: Option<String>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> BmbpRdbcModel<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn get_data_level(&self) -> Option<&String> {
        self.data_level.as_ref()
    }
    pub fn set_data_level(&mut self, data_level: String) -> &mut Self {
        self.data_level = Some(data_level);
        self
    }
    pub fn get_data_status(&self) -> Option<&String> {
        self.data_status.as_ref()
    }
    pub fn set_data_status(&mut self, data_status: String) -> &mut Self {
        self.data_status = Some(data_status);
        self
    }
    pub fn get_data_flag(&self) -> Option<&String> {
        self.data_flag.as_ref()
    }
    pub fn set_data_flag(&mut self, data_flag: String) -> &mut Self {
        self.data_flag = Some(data_flag);
        self
    }
    pub fn get_data_sort(&self) -> Option<&usize> {
        self.data_sort.as_ref()
    }
    pub fn set_data_sort(&mut self, data_sort: usize) -> &mut Self {
        self.data_sort = Some(data_sort);
        self
    }
    pub fn get_data_remark(&self) -> Option<&String> {
        self.data_remark.as_ref()
    }
    pub fn set_data_remark(&mut self, data_remark: String) -> &mut Self {
        self.data_remark = Some(data_remark);
        self
    }
    pub fn get_data_create_time(&self) -> Option<&String> {
        self.data_create_time.as_ref()
    }
    pub fn set_data_create_time(&mut self, data_create_time: String) -> &mut Self {
        self.data_create_time = Some(data_create_time);
        self
    }
    pub fn get_data_create_user(&self) -> Option<&String> {
        self.data_create_user.as_ref()
    }
    pub fn set_data_create_user(&mut self, data_create_user: String) -> &mut Self {
        self.data_create_user = Some(data_create_user);
        self
    }
    pub fn get_data_update_time(&self) -> Option<&String> {
        self.data_update_time.as_ref()
    }
    pub fn set_data_update_time(&mut self, data_update_time: String) -> &mut Self {
        self.data_update_time = Some(data_update_time);
        self
    }
    pub fn get_data_update_user(&self) -> Option<&String> {
        self.data_update_user.as_ref()
    }
    pub fn set_data_update_user(&mut self, data_update_user: String) -> &mut Self {
        self.data_update_user = Some(data_update_user);
        self
    }
    pub fn get_data_owner_org(&self) -> Option<&String> {
        self.data_owner_org.as_ref()
    }
    pub fn set_data_owner_org(&mut self, data_owner_org: String) -> &mut Self {
        self.data_owner_org = Some(data_owner_org);
        self
    }
    pub fn get_data_sign(&self) -> Option<&String> {
        self.data_sign.as_ref()
    }
    pub fn set_data_sign(&mut self, data_sign: String) -> &mut Self {
        self.data_sign = Some(data_sign);
        self
    }
    pub fn get_ext_props(&self) -> &T {
        &self.ext_props
    }
    pub fn get_mut_ext_props(&mut self) -> &mut T {
        &mut self.ext_props
    }
    pub fn set_ext_props(&mut self, ext_props: T) -> &mut Self {
        self.ext_props = ext_props;
        self
    }
}

impl<T> BmbpRdbcModel<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    pub fn build_insert(&self) -> InsertWrapper {
        let mut insert = InsertWrapper::new();
        insert.table(T::get_table_name());
        insert.insert_column_value(RDBC_DATA_ID, self.get_data_id());
        insert.insert_column_value(RDBC_DATA_FLAG, self.get_data_flag());
        insert.insert_column_value(RDBC_DATA_SORT, self.get_data_sort());
        insert.insert_column_value(RDBC_DATA_REMARK, self.get_data_remark());
        insert.insert_column_value(RDBC_DATA_CREATE_TIME, self.get_data_create_time());
        insert.insert_column_value(RDBC_DATA_CREATE_USER, self.get_data_create_user());
        insert.insert_column_value(RDBC_DATA_UPDATE_TIME, self.get_data_update_time());
        insert.insert_column_value(RDBC_DATA_UPDATE_USER, self.get_data_update_user());
        insert.insert_column_value(RDBC_DATA_OWNER_ORG, self.get_data_owner_org());
        insert.insert_column_value(RDBC_DATA_SIGN, self.get_data_sign());
        insert.insert_column_value(RDBC_DATA_LEVEL, self.get_data_level());
        insert.insert_column_value(RDBC_DATA_STATUS, self.get_data_status());
        insert
    }
    pub fn build_query() -> QueryWrapper {
        let mut query = QueryWrapper::new();
        query.table(T::get_table_name());
        let fields = Self::get_table_fields();
        for field in fields {
            query.select(field);
        }
        query
    }
    pub fn build_delete() -> DeleteWrapper {
        let mut delete = DeleteWrapper::new();
        delete.table(T::get_table_name());
        delete
    }

    pub fn init_values(&mut self) -> &mut Self {
        if self.get_data_id().is_none() {
            self.set_data_id(Uuid::new_v4().to_string());
        }
        if self.get_data_flag().is_none() {
            self.set_data_flag(RDBC_NEW_FLAG.to_string());
        }
        if self.get_data_sort().is_none() {
            self.set_data_sort(0);
        }
        if self.get_data_remark().is_none() {
            self.set_data_remark("".to_string());
        }
        if self.get_data_create_time().is_none() {
            self.set_data_create_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.get_data_create_user().is_none() {
            self.set_data_create_user("".to_string());
        }
        if self.get_data_update_time().is_none() {
            self.set_data_update_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.get_data_update_user().is_none() {
            self.set_data_update_user("".to_string());
        }
        if self.get_data_owner_org().is_none() {
            self.set_data_owner_org("".to_string());
        }
        if self.get_data_sign().is_none() {
            self.set_data_sign("".to_string());
        }
        self
    }
    pub fn init_update_values(&mut self) -> &mut Self {
        if self.get_data_update_time().is_none() {
            self.set_data_update_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.get_data_update_user().is_none() {
            self.set_data_update_user("".to_string());
        }
        self
    }
}

impl<T> RdbcModel for BmbpRdbcModel<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    fn get_table_name() -> String {
        T::get_table_name()
    }
    fn get_table_fields() -> Vec<String> {
        let mut row_fields = vec![
            RDBC_DATA_ID.to_string(),
            RDBC_DATA_LEVEL.to_string(),
            RDBC_DATA_STATUS.to_string(),
            RDBC_DATA_FLAG.to_string(),
            RDBC_DATA_SORT.to_string(),
            RDBC_DATA_REMARK.to_string(),
            RDBC_DATA_CREATE_TIME.to_string(),
            RDBC_DATA_CREATE_USER.to_string(),
            RDBC_DATA_UPDATE_TIME.to_string(),
            RDBC_DATA_UPDATE_USER.to_string(),
            RDBC_DATA_OWNER_ORG.to_string(),
            RDBC_DATA_SIGN.to_string(),
        ];
        let table_fields = T::get_table_fields();
        row_fields.extend_from_slice(table_fields.as_slice());
        row_fields
    }

    fn get_table_primary_key() -> String {
        let pri = T::get_table_primary_key();
        if pri != "" {
            pri
        } else {
            RDBC_DATA_ID.to_string()
        }
    }
}

/// RdbcTree 定义树型抽象
pub trait RdbcTree<T>
where
    T: RdbcTree<T>,
{
    fn get_code(&self) -> Option<&String>;
    fn set_code(&mut self, code: String) -> &mut Self;
    fn get_code_path(&self) -> Option<&String>;
    fn set_code_path(&mut self, code_path: String) -> &mut Self;
    fn get_parent_code(&self) -> Option<&String>;
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self;
    fn get_name(&self) -> Option<&String>;
    fn set_name(&mut self, name: String) -> &mut Self;
    fn get_name_path(&self) -> Option<&String>;
    fn set_name_path(&mut self, name_path: String) -> &mut Self;
    fn get_children(&self) -> &Vec<T>;
    fn get_children_mut(&mut self) -> &mut Vec<T>;
    fn set_children(&mut self, children: Vec<T>) -> &mut Self;
    fn get_node_level(&self) -> Option<&usize>;
    fn set_node_level(&mut self, node_level: usize) -> &mut Self;
    fn get_node_leaf(&self) -> Option<&usize>;
    fn set_node_leaf(&mut self, node_leaf: usize) -> &mut Self;
}

/// BmbpRdbcTree 定义平台提供的树节点
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize,
{
    // 节点编码
    code: Option<String>,
    // 节点路径编码
    code_path: Option<String>,
    // 父节点编码
    parent_code: Option<String>,
    // 节点名称
    name: Option<String>,
    // 节点路径名称
    name_path: Option<String>,
    // 子节点
    children: Vec<BmbpRdbcTree<T>>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<usize>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcTree<BmbpRdbcTree<T>> for BmbpRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize,
{
    fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }
    fn set_code(&mut self, code: String) -> &mut Self {
        self.code = Some(code);
        self
    }
    fn get_code_path(&self) -> Option<&String> {
        self.code_path.as_ref()
    }
    fn set_code_path(&mut self, code_path: String) -> &mut Self {
        self.code_path = Some(code_path);
        self
    }
    fn get_parent_code(&self) -> Option<&String> {
        self.parent_code.as_ref()
    }
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = Some(parent_code);
        self
    }
    fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    fn get_name_path(&self) -> Option<&String> {
        self.name_path.as_ref()
    }
    fn set_name_path(&mut self, name_path: String) -> &mut Self {
        self.name_path = Some(name_path);
        self
    }
    fn get_children(&self) -> &Vec<BmbpRdbcTree<T>> {
        &self.children
    }
    fn get_children_mut(&mut self) -> &mut Vec<BmbpRdbcTree<T>> {
        &mut self.children
    }
    fn set_children(&mut self, children: Vec<BmbpRdbcTree<T>>) -> &mut Self {
        self.children = children;
        self
    }
    fn get_node_level(&self) -> Option<&usize> {
        self.node_level.as_ref()
    }
    fn set_node_level(&mut self, node_level: usize) -> &mut Self {
        self.node_level = Some(node_level);
        self
    }
    fn get_node_leaf(&self) -> Option<&usize> {
        self.node_leaf.as_ref()
    }
    fn set_node_leaf(&mut self, node_leaf: usize) -> &mut Self {
        self.node_leaf = Some(node_leaf);
        self
    }
}

impl<T> BmbpRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize,
{
    pub fn get_ext_props(&self) -> &T {
        &self.ext_props
    }
    pub fn set_ext_props(&mut self, ext_props: T) -> &mut Self {
        self.ext_props = ext_props;
        self
    }
    pub fn get_ext_props_mut(&mut self) -> &mut T {
        &mut self.ext_props
    }
}

/// /// 定义数据库表-树型结构实体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpOrmRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    /// 记录主键
    data_id: Option<String>,
    /// 记录密级
    data_level: Option<String>,
    /// 记录状态
    data_status: Option<String>,
    /// 记录标识
    data_flag: Option<String>,
    /// 记录显示顺序
    data_sort: Option<usize>,
    /// 记录备注
    data_remark: Option<String>,
    /// 记录创建时间
    data_create_time: Option<String>,
    /// 记录创建人
    data_create_user: Option<String>,
    /// 记录更新时间
    data_update_time: Option<String>,
    /// 记录更新用户
    data_update_user: Option<String>,
    /// 记录所属组织
    data_owner_org: Option<String>,
    /// 记录防串改标识
    data_sign: Option<String>,
    // 节点编码
    code: Option<String>,
    // 节点路径编码
    code_path: Option<String>,
    // 父节点编码
    parent_code: Option<String>,
    // 节点名称
    name: Option<String>,
    // 节点路径名称
    name_path: Option<String>,
    // 子节点
    children: Vec<BmbpOrmRdbcTree<T>>,
    // 节点层级
    node_level: Option<usize>,
    // 是否叶子节点
    node_leaf: Option<usize>,
    #[serde(flatten)]
    ext_props: T,
}

impl<T> RdbcModel for BmbpOrmRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    fn get_table_name() -> String {
        T::get_table_name()
    }

    fn get_table_fields() -> Vec<String> {
        let mut row_fields = vec![
            RDBC_DATA_ID.to_string(),
            RDBC_DATA_LEVEL.to_string(),
            RDBC_DATA_STATUS.to_string(),
            RDBC_DATA_FLAG.to_string(),
            RDBC_DATA_SORT.to_string(),
            RDBC_DATA_REMARK.to_string(),
            RDBC_DATA_CREATE_TIME.to_string(),
            RDBC_DATA_CREATE_USER.to_string(),
            RDBC_DATA_UPDATE_TIME.to_string(),
            RDBC_DATA_UPDATE_USER.to_string(),
            RDBC_DATA_OWNER_ORG.to_string(),
            RDBC_DATA_SIGN.to_string(),
            RDBC_TREE_CODE.to_string(),
            RDBC_TREE_CODE_PATH.to_string(),
            RDBC_TREE_PARENT_CODE.to_string(),
            RDBC_TREE_NAME.to_string(),
            RDBC_TREE_NAME_PATH.to_string(),
            RDBC_TREE_NODE_LEVEL.to_string(),
            RDBC_TREE_NODE_LEAF.to_string(),
        ];
        let table_fields = T::get_table_fields();
        row_fields.extend_from_slice(table_fields.as_slice());
        row_fields
    }

    fn get_table_primary_key() -> String {
        RDBC_DATA_ID.to_string()
    }
}

impl<T> RdbcTree<BmbpOrmRdbcTree<T>> for BmbpOrmRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }
    fn set_code(&mut self, code: String) -> &mut Self {
        self.code = Some(code);
        self
    }
    fn get_code_path(&self) -> Option<&String> {
        self.code_path.as_ref()
    }
    fn set_code_path(&mut self, code_path: String) -> &mut Self {
        self.code_path = Some(code_path);
        self
    }
    fn get_parent_code(&self) -> Option<&String> {
        self.parent_code.as_ref()
    }
    fn set_parent_code(&mut self, parent_code: String) -> &mut Self {
        self.parent_code = Some(parent_code);
        self
    }
    fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    fn get_name_path(&self) -> Option<&String> {
        self.name_path.as_ref()
    }
    fn set_name_path(&mut self, name_path: String) -> &mut Self {
        self.name_path = Some(name_path);
        self
    }
    fn get_children(&self) -> &Vec<BmbpOrmRdbcTree<T>> {
        &self.children
    }
    fn get_children_mut(&mut self) -> &mut Vec<BmbpOrmRdbcTree<T>> {
        &mut self.children
    }
    fn set_children(&mut self, children: Vec<BmbpOrmRdbcTree<T>>) -> &mut Self {
        self.children = children;
        self
    }
    fn get_node_level(&self) -> Option<&usize> {
        self.node_level.as_ref()
    }
    fn set_node_level(&mut self, node_level: usize) -> &mut Self {
        self.node_level = Some(node_level);
        self
    }
    fn get_node_leaf(&self) -> Option<&usize> {
        self.node_leaf.as_ref()
    }
    fn set_node_leaf(&mut self, node_leaf: usize) -> &mut Self {
        self.node_leaf = Some(node_leaf);
        self
    }
}

impl<T> BmbpOrmRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    pub fn get_data_id(&self) -> Option<&String> {
        self.data_id.as_ref()
    }
    pub fn set_data_id(&mut self, data_id: String) -> &mut Self {
        self.data_id = Some(data_id);
        self
    }
    pub fn get_data_level(&self) -> Option<&String> {
        self.data_level.as_ref()
    }
    pub fn set_data_level(&mut self, data_level: String) -> &mut Self {
        self.data_level = Some(data_level);
        self
    }
    pub fn get_data_status(&self) -> Option<&String> {
        self.data_status.as_ref()
    }
    pub fn set_data_status(&mut self, data_status: String) -> &mut Self {
        self.data_status = Some(data_status);
        self
    }
    pub fn get_data_flag(&self) -> Option<&String> {
        self.data_flag.as_ref()
    }
    pub fn set_data_flag(&mut self, data_flag: String) -> &mut Self {
        self.data_flag = Some(data_flag);
        self
    }
    pub fn get_data_sort(&self) -> Option<&usize> {
        self.data_sort.as_ref()
    }
    pub fn set_data_sort(&mut self, data_sort: usize) -> &mut Self {
        self.data_sort = Some(data_sort);
        self
    }
    pub fn get_data_remark(&self) -> Option<&String> {
        self.data_remark.as_ref()
    }
    pub fn set_data_remark(&mut self, data_remark: String) -> &mut Self {
        self.data_remark = Some(data_remark);
        self
    }
    pub fn get_data_create_time(&self) -> Option<&String> {
        self.data_create_time.as_ref()
    }
    pub fn set_data_create_time(&mut self, data_create_time: String) -> &mut Self {
        self.data_create_time = Some(data_create_time);
        self
    }
    pub fn get_data_create_user(&self) -> Option<&String> {
        self.data_create_user.as_ref()
    }
    pub fn set_data_create_user(&mut self, data_create_user: String) -> &mut Self {
        self.data_create_user = Some(data_create_user);
        self
    }
    pub fn get_data_update_time(&self) -> Option<&String> {
        self.data_update_time.as_ref()
    }
    pub fn set_data_update_time(&mut self, data_update_time: String) -> &mut Self {
        self.data_update_time = Some(data_update_time);
        self
    }
    pub fn get_data_update_user(&self) -> Option<&String> {
        self.data_update_user.as_ref()
    }
    pub fn set_data_update_user(&mut self, data_update_user: String) -> &mut Self {
        self.data_update_user = Some(data_update_user);
        self
    }
    pub fn get_data_owner_org(&self) -> Option<&String> {
        self.data_owner_org.as_ref()
    }
    pub fn set_data_owner_org(&mut self, data_owner_org: String) -> &mut Self {
        self.data_owner_org = Some(data_owner_org);
        self
    }
    pub fn get_data_sign(&self) -> Option<&String> {
        self.data_sign.as_ref()
    }
    pub fn set_data_sign(&mut self, data_sign: String) -> &mut Self {
        self.data_sign = Some(data_sign);
        self
    }
    pub fn get_ext_props(&self) -> &T {
        &self.ext_props
    }
    pub fn get_ext_props_mut(&mut self) -> &mut T {
        &mut self.ext_props
    }
    pub fn set_ext_props(&mut self, ext_props: T) -> &mut Self {
        self.ext_props = ext_props;
        self
    }
}

impl<T> BmbpOrmRdbcTree<T>
where
    T: Default + Debug + Clone + Serialize + RdbcModel,
{
    pub fn build_insert(&self) -> InsertWrapper {
        let mut insert = InsertWrapper::new();
        insert.table(T::get_table_name());
        insert.insert_column_value(RDBC_DATA_ID, self.get_data_id());
        insert.insert_column_value(RDBC_DATA_FLAG, self.get_data_flag());
        insert.insert_column_value(RDBC_DATA_SORT, self.get_data_sort());
        insert.insert_column_value(RDBC_DATA_REMARK, self.get_data_remark());
        insert.insert_column_value(RDBC_DATA_CREATE_TIME, self.get_data_create_time());
        insert.insert_column_value(RDBC_DATA_CREATE_USER, self.get_data_create_user());
        insert.insert_column_value(RDBC_DATA_UPDATE_TIME, self.get_data_update_time());
        insert.insert_column_value(RDBC_DATA_UPDATE_USER, self.get_data_update_user());
        insert.insert_column_value(RDBC_DATA_OWNER_ORG, self.get_data_owner_org());
        insert.insert_column_value(RDBC_DATA_SIGN, self.get_data_sign());
        insert.insert_column_value(RDBC_DATA_LEVEL, self.get_data_level());
        insert.insert_column_value(RDBC_DATA_STATUS, self.get_data_status());
        insert.insert_column_value(RDBC_TREE_CODE, self.get_code().clone());
        insert.insert_column_value(RDBC_TREE_CODE_PATH, self.get_code_path());
        insert.insert_column_value(RDBC_TREE_PARENT_CODE, self.get_parent_code());
        insert.insert_column_value(RDBC_TREE_NAME, self.get_name());
        insert.insert_column_value(RDBC_TREE_NAME_PATH, self.get_name_path());
        insert.insert_column_value(RDBC_TREE_NODE_LEVEL, self.get_node_level());
        insert.insert_column_value(RDBC_TREE_NODE_LEAF, self.get_node_leaf());
        insert
    }
    pub fn build_query() -> QueryWrapper {
        let mut query = QueryWrapper::new();
        query.table(T::get_table_name());
        let fields = Self::get_table_fields();
        for field in fields {
            query.select(field);
        }
        query
    }
    pub fn build_delete() -> DeleteWrapper {
        let mut delete = DeleteWrapper::new();
        delete.table(T::get_table_name());
        delete
    }
    pub fn init_values(&mut self) -> &mut Self {
        if self.get_data_id().is_none() {
            self.set_data_id(Uuid::new_v4().to_string());
        }
        if self.get_data_flag().is_none() {
            self.set_data_flag(RDBC_NEW_FLAG.to_string());
        }
        if self.get_data_level().is_none() {
            self.set_data_level("0".to_string());
        }
        if self.get_data_status().is_none() {
            self.set_data_status(RDBC_ENABLE.to_string());
        }
        if self.get_data_sort().is_none() {
            self.set_data_sort(0);
        }
        if self.get_data_remark().is_none() {
            self.set_data_remark("".to_string());
        }
        if self.get_data_create_time().is_none() {
            self.set_data_create_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.get_data_create_user().is_none() {
            self.set_data_create_user("".to_string());
        }
        if self.get_data_update_time().is_none() {
            self.set_data_update_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.get_data_update_user().is_none() {
            self.set_data_update_user("".to_string());
        }
        if self.get_data_owner_org().is_none() {
            self.set_data_owner_org("".to_string());
        }
        if self.get_data_sign().is_none() {
            self.set_data_sign("".to_string());
        }
        self
    }
    pub fn init_update_values(&mut self) -> &mut Self {
        if self.get_data_update_time().is_none() {
            self.set_data_update_time(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        }
        if self.get_data_update_user().is_none() {
            self.set_data_update_user("".to_string());
        }
        self
    }
}

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

impl<T> From<RdbcOrmRow> for BmbpRdbcModel<T>
where
    T: From<RdbcOrmRow> + Default + Debug + Clone + Serialize + RdbcModel,
{
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpRdbcModel::<T>::default();
        if let Some(data) = row.data.get("data_id") {
            model.set_data_id(data.get_string());
        }
        if let Some(data) = row.data.get("data_level") {
            model.set_data_level(data.get_string());
        }
        if let Some(data) = row.data.get("data_status") {
            model.set_data_status(data.get_string());
        }
        if let Some(data) = row.data.get("data_flag") {
            model.set_data_flag(data.get_string());
        }
        if let Some(data) = row.data.get("data_sort") {
            if let Some(v) = data.get_usize() {
                model.set_data_sort(v);
            }
        }
        if let Some(data) = row.data.get("data_remark") {
            model.set_data_remark(data.get_string());
        }
        if let Some(data) = row.data.get("data_create_time") {
            model.set_data_create_time(data.get_string());
        }
        if let Some(data) = row.data.get("data_create_user") {
            model.set_data_create_user(data.get_string());
        }
        if let Some(data) = row.data.get("data_update_time") {
            model.set_data_update_time(data.get_string());
        }
        if let Some(data) = row.data.get("data_update_user") {
            model.set_data_update_user(data.get_string());
        }
        if let Some(data) = row.data.get("data_owner_org") {
            model.set_data_owner_org(data.get_string());
        }
        if let Some(data) = row.data.get("data_sign") {
            model.set_data_sign(data.get_string());
        }
        let ext_ops = T::from(row);
        model.set_ext_props(ext_ops);
        model
    }
}

impl<T> From<RdbcOrmRow> for BmbpRdbcTree<T>
where
    T: From<RdbcOrmRow> + Default + Debug + Clone + Serialize,
{
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpRdbcTree::<T>::default();
        if let Some(code) = row.data.get(RDBC_TREE_CODE) {
            model.set_code(code.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_CODE_PATH) {
            model.set_code_path(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_PARENT_CODE) {
            model.set_parent_code(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_NAME) {
            model.set_name(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_NAME_PATH) {
            model.set_name_path(data.get_string());
        }

        if let Some(data) = row.data.get(RDBC_TREE_NODE_LEVEL) {
            if let Some(v) = data.get_usize() {
                model.set_node_level(v);
            }
        }
        if let Some(data) = row.data.get(RDBC_TREE_NODE_LEAF) {
            if let Some(v) = data.get_usize() {
                model.set_node_leaf(v);
            }
        }
        let ext_ops = T::from(row);
        model.set_ext_props(ext_ops);
        model
    }
}

impl<T> From<RdbcOrmRow> for BmbpOrmRdbcTree<T>
where
    T: From<RdbcOrmRow> + Default + Debug + Clone + Serialize + RdbcModel,
{
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpOrmRdbcTree::<T>::default();
        if let Some(data) = row.data.get(RDBC_DATA_ID) {
            model.set_data_id(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_LEVEL) {
            model.set_data_level(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_STATUS) {
            model.set_data_status(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_FLAG) {
            model.set_data_flag(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_SORT) {
            if let Some(v) = data.get_usize() {
                model.set_data_sort(v);
            }
        }
        if let Some(data) = row.data.get(RDBC_DATA_REMARK) {
            model.set_data_remark(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_CREATE_TIME) {
            model.set_data_create_time(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_CREATE_TIME) {
            model.set_data_create_user(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_UPDATE_TIME) {
            model.set_data_update_time(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_UPDATE_USER) {
            model.set_data_update_user(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_OWNER_ORG) {
            model.set_data_owner_org(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_DATA_SIGN) {
            model.set_data_sign(data.get_string());
        }
        if let Some(code) = row.data.get(RDBC_TREE_CODE) {
            model.set_code(code.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_CODE_PATH) {
            model.set_code_path(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_PARENT_CODE) {
            model.set_parent_code(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_NAME) {
            model.set_name(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_NAME_PATH) {
            model.set_name_path(data.get_string());
        }
        if let Some(data) = row.data.get(RDBC_TREE_NODE_LEVEL) {
            if let Some(v) = data.get_usize() {
                model.set_node_level(v);
            }
        }
        if let Some(data) = row.data.get(RDBC_TREE_NODE_LEAF) {
            if let Some(v) = data.get_usize() {
                model.set_node_leaf(v);
            }
        }
        let ext_ops = T::from(row);
        model.set_ext_props(ext_ops);
        model
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

/// RdbcTree 定义树型抽象
pub trait RdbcMacroTree<T>
where
    T: RdbcMacroTree<T>,
{
    fn get_code(&self) -> &Option<String>;
    fn set_code(&mut self, code: Option<String>) -> &mut Self;
    fn get_parent_code(&self) -> &Option<String>;
    fn set_parent_code(&mut self, parent_code: Option<String>) -> &mut Self;
    fn get_children(&self) -> &Option<Vec<T>>;
    fn get_children_mut(&mut self) -> &mut Option<Vec<T>>;
    fn set_children(&mut self, children: Option<Vec<T>>) -> &mut Self;
}
