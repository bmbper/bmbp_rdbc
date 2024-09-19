use bmbp_rdbc_sql::RdbcTableWrapper;
use bmbp_rdbc_sql::{PgRdbcSQLBuilder, QueryWrapper, RdbcTableFilter, SQLBuilder};
use bmbp_rdbc_type::{RdbcIdent, RdbcTable};
use serde::Deserialize;
use serde::Serialize;
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct BmbpDict {
    dict_value: Option<String>,
    dict_alias: Option<String>,
    data_id: Option<String>,
    data_level: Option<String>,
    data_flag: Option<String>,
    data_status: Option<String>,
    data_sort: Option<i32>,
    data_create_time: Option<String>,
    data_create_user: Option<String>,
    data_update_time: Option<String>,
    data_update_user: Option<String>,
    data_owner_org: Option<String>,
    data_sign: Option<String>,
    dict_code: Option<String>,
    dict_parent_code: Option<String>,
    dict_name: Option<String>,
    dict_code_path: Option<String>,
    dict_name_path: Option<String>,
    dict_tree_grade: Option<u32>,
    dict_leaf: Option<String>,
    dict_type: Option<String>,
    dict_children: Option<Vec<BmbpDict>>,
}
impl BmbpDict {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_dict_value(&mut self, value: Option<String>) -> &mut Self {
        self.dict_value = value;
        self
    }
    pub fn get_dict_value(&self) -> &Option<String> {
        &self.dict_value
    }
    pub fn get_mut_dict_value(&mut self) -> &mut Option<String> {
        &mut self.dict_value
    }
    pub fn set_dict_alias(&mut self, value: Option<String>) -> &mut Self {
        self.dict_alias = value;
        self
    }
    pub fn get_dict_alias(&self) -> &Option<String> {
        &self.dict_alias
    }
    pub fn get_mut_dict_alias(&mut self) -> &mut Option<String> {
        &mut self.dict_alias
    }
    pub fn set_data_id(&mut self, value: Option<String>) -> &mut Self {
        self.data_id = value;
        self
    }
    pub fn get_data_id(&self) -> &Option<String> {
        &self.data_id
    }
    pub fn get_mut_data_id(&mut self) -> &mut Option<String> {
        &mut self.data_id
    }
    pub fn set_data_level(&mut self, value: Option<String>) -> &mut Self {
        self.data_level = value;
        self
    }
    pub fn get_data_level(&self) -> &Option<String> {
        &self.data_level
    }
    pub fn get_mut_data_level(&mut self) -> &mut Option<String> {
        &mut self.data_level
    }
    pub fn set_data_flag(&mut self, value: Option<String>) -> &mut Self {
        self.data_flag = value;
        self
    }
    pub fn get_data_flag(&self) -> &Option<String> {
        &self.data_flag
    }
    pub fn get_mut_data_flag(&mut self) -> &mut Option<String> {
        &mut self.data_flag
    }
    pub fn set_data_status(&mut self, value: Option<String>) -> &mut Self {
        self.data_status = value;
        self
    }
    pub fn get_data_status(&self) -> &Option<String> {
        &self.data_status
    }
    pub fn get_mut_data_status(&mut self) -> &mut Option<String> {
        &mut self.data_status
    }
    pub fn set_data_sort(&mut self, value: Option<i32>) -> &mut Self {
        self.data_sort = value;
        self
    }
    pub fn get_data_sort(&self) -> &Option<i32> {
        &self.data_sort
    }
    pub fn get_mut_data_sort(&mut self) -> &mut Option<i32> {
        &mut self.data_sort
    }
    pub fn set_data_create_time(&mut self, value: Option<String>) -> &mut Self {
        self.data_create_time = value;
        self
    }
    pub fn get_data_create_time(&self) -> &Option<String> {
        &self.data_create_time
    }
    pub fn get_mut_data_create_time(&mut self) -> &mut Option<String> {
        &mut self.data_create_time
    }
    pub fn set_data_create_user(&mut self, value: Option<String>) -> &mut Self {
        self.data_create_user = value;
        self
    }
    pub fn get_data_create_user(&self) -> &Option<String> {
        &self.data_create_user
    }
    pub fn get_mut_data_create_user(&mut self) -> &mut Option<String> {
        &mut self.data_create_user
    }
    pub fn set_data_update_time(&mut self, value: Option<String>) -> &mut Self {
        self.data_update_time = value;
        self
    }
    pub fn get_data_update_time(&self) -> &Option<String> {
        &self.data_update_time
    }
    pub fn get_mut_data_update_time(&mut self) -> &mut Option<String> {
        &mut self.data_update_time
    }
    pub fn set_data_update_user(&mut self, value: Option<String>) -> &mut Self {
        self.data_update_user = value;
        self
    }
    pub fn get_data_update_user(&self) -> &Option<String> {
        &self.data_update_user
    }
    pub fn get_mut_data_update_user(&mut self) -> &mut Option<String> {
        &mut self.data_update_user
    }
    pub fn set_data_owner_org(&mut self, value: Option<String>) -> &mut Self {
        self.data_owner_org = value;
        self
    }
    pub fn get_data_owner_org(&self) -> &Option<String> {
        &self.data_owner_org
    }
    pub fn get_mut_data_owner_org(&mut self) -> &mut Option<String> {
        &mut self.data_owner_org
    }
    pub fn set_data_sign(&mut self, value: Option<String>) -> &mut Self {
        self.data_sign = value;
        self
    }
    pub fn get_data_sign(&self) -> &Option<String> {
        &self.data_sign
    }
    pub fn get_mut_data_sign(&mut self) -> &mut Option<String> {
        &mut self.data_sign
    }
    pub fn set_dict_code(&mut self, value: Option<String>) -> &mut Self {
        self.dict_code = value;
        self
    }
    pub fn get_dict_code(&self) -> &Option<String> {
        &self.dict_code
    }
    pub fn get_mut_dict_code(&mut self) -> &mut Option<String> {
        &mut self.dict_code
    }
    pub fn set_dict_parent_code(&mut self, value: Option<String>) -> &mut Self {
        self.dict_parent_code = value;
        self
    }
    pub fn get_dict_parent_code(&self) -> &Option<String> {
        &self.dict_parent_code
    }
    pub fn get_mut_dict_parent_code(&mut self) -> &mut Option<String> {
        &mut self.dict_parent_code
    }
    pub fn set_dict_name(&mut self, value: Option<String>) -> &mut Self {
        self.dict_name = value;
        self
    }
    pub fn get_dict_name(&self) -> &Option<String> {
        &self.dict_name
    }
    pub fn get_mut_dict_name(&mut self) -> &mut Option<String> {
        &mut self.dict_name
    }
    pub fn set_dict_code_path(&mut self, value: Option<String>) -> &mut Self {
        self.dict_code_path = value;
        self
    }
    pub fn get_dict_code_path(&self) -> &Option<String> {
        &self.dict_code_path
    }
    pub fn get_mut_dict_code_path(&mut self) -> &mut Option<String> {
        &mut self.dict_code_path
    }
    pub fn set_dict_name_path(&mut self, value: Option<String>) -> &mut Self {
        self.dict_name_path = value;
        self
    }
    pub fn get_dict_name_path(&self) -> &Option<String> {
        &self.dict_name_path
    }
    pub fn get_mut_dict_name_path(&mut self) -> &mut Option<String> {
        &mut self.dict_name_path
    }
    pub fn set_dict_tree_grade(&mut self, value: Option<u32>) -> &mut Self {
        self.dict_tree_grade = value;
        self
    }
    pub fn get_dict_tree_grade(&self) -> &Option<u32> {
        &self.dict_tree_grade
    }
    pub fn get_mut_dict_tree_grade(&mut self) -> &mut Option<u32> {
        &mut self.dict_tree_grade
    }
    pub fn set_dict_leaf(&mut self, value: Option<String>) -> &mut Self {
        self.dict_leaf = value;
        self
    }
    pub fn get_dict_leaf(&self) -> &Option<String> {
        &self.dict_leaf
    }
    pub fn get_mut_dict_leaf(&mut self) -> &mut Option<String> {
        &mut self.dict_leaf
    }
    pub fn set_dict_type(&mut self, value: Option<String>) -> &mut Self {
        self.dict_type = value;
        self
    }
    pub fn get_dict_type(&self) -> &Option<String> {
        &self.dict_type
    }
    pub fn get_mut_dict_type(&mut self) -> &mut Option<String> {
        &mut self.dict_type
    }
    pub fn set_dict_children(&mut self, value: Option<Vec<BmbpDict>>) -> &mut Self {
        self.dict_children = value;
        self
    }
    pub fn get_dict_children(&self) -> &Option<Vec<BmbpDict>> {
        &self.dict_children
    }
    pub fn get_mut_dict_children(&mut self) -> &mut Option<Vec<BmbpDict>> {
        &mut self.dict_children
    }
}
pub enum BmbpDictColumn {
    DictValue,
    DictAlias,
    DataId,
    DataLevel,
    DataFlag,
    DataStatus,
    DataSort,
    DataCreateTime,
    DataCreateUser,
    DataUpdateTime,
    DataUpdateUser,
    DataOwnerOrg,
    DataSign,
    DictCode,
    DictParentCode,
    DictName,
    DictCodePath,
    DictNamePath,
    DictTreeGrade,
    DictLeaf,
    DictType,
}
impl RdbcIdent for BmbpDictColumn {
    fn get_ident(&self) -> String {
        match self {
            Self::DictValue => "dict_value".to_string(),
            Self::DictAlias => "dict_alias".to_string(),
            Self::DataId => "data_id".to_string(),
            Self::DataLevel => "data_level".to_string(),
            Self::DataFlag => "data_flag".to_string(),
            Self::DataStatus => "data_status".to_string(),
            Self::DataSort => "data_sort".to_string(),
            Self::DataCreateTime => "data_create_time".to_string(),
            Self::DataCreateUser => "data_create_user".to_string(),
            Self::DataUpdateTime => "data_update_time".to_string(),
            Self::DataUpdateUser => "data_update_user".to_string(),
            Self::DataOwnerOrg => "data_owner_org".to_string(),
            Self::DataSign => "data_sign".to_string(),
            Self::DictCode => "dict_code".to_string(),
            Self::DictParentCode => "dict_parent_code".to_string(),
            Self::DictName => "dict_name".to_string(),
            Self::DictCodePath => "dict_code_path".to_string(),
            Self::DictNamePath => "dict_name_path".to_string(),
            Self::DictTreeGrade => "dict_tree_grade".to_string(),
            Self::DictLeaf => "dict_leaf".to_string(),
            Self::DictType => "dict_type".to_string(),
        }
    }
}
impl RdbcTable for BmbpDict {
    fn get_table() -> impl RdbcIdent {
        "BMBP_CONFIG_DICT".to_string()
    }
    fn get_columns() -> Vec<impl RdbcIdent> {
        vec![
            BmbpDictColumn::DictValue,
            BmbpDictColumn::DictAlias,
            BmbpDictColumn::DataId,
            BmbpDictColumn::DataLevel,
            BmbpDictColumn::DataFlag,
            BmbpDictColumn::DataStatus,
            BmbpDictColumn::DataSort,
            BmbpDictColumn::DataCreateTime,
            BmbpDictColumn::DataCreateUser,
            BmbpDictColumn::DataUpdateTime,
            BmbpDictColumn::DataUpdateUser,
            BmbpDictColumn::DataOwnerOrg,
            BmbpDictColumn::DataSign,
            BmbpDictColumn::DictCode,
            BmbpDictColumn::DictParentCode,
            BmbpDictColumn::DictName,
            BmbpDictColumn::DictCodePath,
            BmbpDictColumn::DictNamePath,
            BmbpDictColumn::DictTreeGrade,
            BmbpDictColumn::DictLeaf,
            BmbpDictColumn::DictType,
        ]
    }
    fn get_primary_key() -> impl RdbcIdent {
        "data_id".to_string()
    }
    fn get_union_key() -> Vec<impl RdbcIdent> {
        vec!["data_id".to_string()]
    }
}
#[test]
fn test_query_new_from() {
    let query_wrapper = QueryWrapper::new_from::<BmbpDict>();
    let (sql, _) = SQLBuilder::build_query_script::<PgRdbcSQLBuilder>(&query_wrapper);
    println!("===>{}", sql);
}
#[test]
fn test_query_select_default() {
    let mut query_wrapper = QueryWrapper::new();
    query_wrapper.table(BmbpDict::get_table());
    let (sql, _) = SQLBuilder::build_query_script::<PgRdbcSQLBuilder>(&query_wrapper);
    println!("===>: {}", sql);
}
#[test]
fn test_query_select() {
    let mut query_wrapper = QueryWrapper::new();
    query_wrapper.table(BmbpDict::get_table());
    query_wrapper.select_value("1");
    query_wrapper.select_value_alias(1, "demo");
    let (sql, _) = SQLBuilder::build_query_script::<PgRdbcSQLBuilder>(&query_wrapper);
    println!("===>: {}", sql);
}

#[test]
fn test_query_select_filter() {
    let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
    query_wrapper.in_v(BmbpDictColumn::DictCode, vec!["D", "D", "C"]);
    let (sql, _) = SQLBuilder::build_query_script::<PgRdbcSQLBuilder>(&query_wrapper);
    println!("===>: {}", sql);
}

#[test]
fn test_query_select_table() {
    let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
    query_wrapper
        .table(BmbpDict::get_table())
        .table("A1")
        .table_alias(BmbpDict::get_table(), "t1");
    query_wrapper.schema_table("bmbp", BmbpDict::get_table());
    query_wrapper.schema_table_alias("bmbp", BmbpDict::get_table(), "t3");
    let (sql, _) = SQLBuilder::build_query_script::<PgRdbcSQLBuilder>(&query_wrapper);
    println!("===>: {}", sql);
}

#[test]
fn test_query_select_join_table() {
    let mut query_wrapper = QueryWrapper::new_from::<BmbpDict>();
    query_wrapper
        .table(BmbpDict::get_table())
        .table("A1")
        .table_alias(BmbpDict::get_table(), "t1");
    query_wrapper.schema_table("bmbp", BmbpDict::get_table());
    query_wrapper.schema_table_alias("bmbp", BmbpDict::get_table(), "t3");
    query_wrapper.join_table(BmbpDict::get_table());
    let (sql, _) = SQLBuilder::build_query_script::<PgRdbcSQLBuilder>(&query_wrapper);
    println!("===>: {}", sql);
}
