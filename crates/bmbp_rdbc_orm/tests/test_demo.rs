use bmbp_rdbc_orm::{RdbcDataBase, RdbcDataSource, RdbcIdent, RdbcOrm, RdbcOrmRow, RdbcTable};
use bmbp_rdbc_sql::{
    DeleteWrapper, InsertWrapper, QueryWrapper, RdbcColumn, RdbcTableFilter, RdbcTableWrapper,
    UpdateWrapper,
};
use serde::{Deserialize, Serialize};

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
impl From<RdbcOrmRow> for BmbpDict {
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = BmbpDict::new();
        if let Some(data) = row.get_data().get("dict_value") {
            model.set_dict_value(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_alias") {
            model.set_dict_alias(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_id") {
            model.set_data_id(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_level") {
            model.set_data_level(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_flag") {
            model.set_data_flag(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_status") {
            model.set_data_status(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_sort") {
            model.set_data_sort(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_create_time") {
            model.set_data_create_time(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_create_user") {
            model.set_data_create_user(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_update_time") {
            model.set_data_update_time(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_update_user") {
            model.set_data_update_user(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_owner_org") {
            model.set_data_owner_org(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_sign") {
            model.set_data_sign(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_code") {
            model.set_dict_code(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_parent_code") {
            model.set_dict_parent_code(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_name") {
            model.set_dict_name(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_code_path") {
            model.set_dict_code_path(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_name_path") {
            model.set_dict_name_path(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_tree_grade") {
            model.set_dict_tree_grade(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_leaf") {
            model.set_dict_leaf(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("dict_type") {
            model.set_dict_type(Some(data.into()));
        }
        model
    }
}

fn build_data_source() -> RdbcDataSource {
    let mut data_source = RdbcDataSource::new();
    data_source.set_typ(RdbcDataBase::Postgres);
    data_source.set_host(Some("127.0.0.1".to_string()));
    data_source.set_port(Some(5432u32));
    data_source.set_username(Some("bmbp".to_string()));
    data_source.set_password(Some("zgk0130!".to_string()));
    data_source.set_database(Some("bmbp".to_string()));
    data_source
}
async fn build_orm() -> RdbcOrm {
    let data_source = build_data_source();
    let orm = RdbcOrm::new(data_source).await.unwrap();
    orm
}

#[tokio::test]
async fn test_query_dict_page() {
    let orm = build_orm().await;
    let query = QueryWrapper::new_from::<BmbpDict>();
    let dict_vec = orm
        .select_page_by_query::<BmbpDict>(1, 2, &query)
        .await
        .unwrap();
    if let Some(dict) = dict_vec.data() {
        for item in dict {
            println!("{}", item.get_dict_name().as_ref().unwrap());
        }
    }
}
#[tokio::test]
async fn test_query_dict_list() {
    let orm = build_orm().await;
    let query = QueryWrapper::new_from::<BmbpDict>();
    let dict_vec = orm.select_list_by_query::<BmbpDict>(&query).await.unwrap();
    if let Some(dict) = dict_vec {
        for item in dict {
            println!("{}", item.get_dict_name().as_ref().unwrap());
        }
    }
}
#[tokio::test]
async fn test_query_dict_info() {
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let mut query = QueryWrapper::new_from::<BmbpDict>();
    query.eq_(BmbpDictColumn::DataId, "9a5709fee8bd4c5c9cff0974f0e3982c");
    let dict_vec = orm.select_one_by_query::<BmbpDict>(&query).await.unwrap();
    if let Some(dict) = dict_vec {
        println!("{}", serde_json::to_string_pretty(&dict).unwrap());
    }
}
#[tokio::test]
async fn test_query_combo_dict_by_alias() {
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let mut query = QueryWrapper::new_from::<BmbpDict>();
    query.in_v(BmbpDictColumn::DictAlias, vec!["D1", "AA"]);
    let dict_vec = orm.select_list_by_query::<BmbpDict>(&query).await.unwrap();
    if let Some(dict) = dict_vec {
        println!("{}", serde_json::to_string_pretty(&dict).unwrap());
    }
}
#[tokio::test]
async fn test_query_combo_between() {
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let mut query = QueryWrapper::new_from::<BmbpDict>();
    query.between_(BmbpDictColumn::DictAlias, "A", "D");
    let dict_vec = orm.select_list_by_query::<BmbpDict>(&query).await.unwrap();
    if let Some(dict) = dict_vec {
        println!("{}", serde_json::to_string_pretty(&dict).unwrap());
    }
}
#[tokio::test]
async fn test_query_combo_like() {
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let mut query = QueryWrapper::new_from::<BmbpDict>();
    query.like_left_(BmbpDictColumn::DictAlias, "D");
    let dict_vec = orm.select_list_by_query::<BmbpDict>(&query).await.unwrap();
    if let Some(dict) = dict_vec {
        println!("{}", serde_json::to_string_pretty(&dict).unwrap());
    }
}
#[tokio::test]
async fn test_insert_dict() {
    let mut insert = InsertWrapper::new();
    insert
        .table(BmbpDict::get_table())
        .insert(BmbpDictColumn::DataId, "9a5709fee8bd4c5c9cff0974f0e3982d")
        .insert(BmbpDictColumn::DictCode, "D11")
        .insert(BmbpDictColumn::DictParentCode, "D00")
        .insert(BmbpDictColumn::DictName, "D10")
        .insert(BmbpDictColumn::DictCodePath, "#,D00")
        .insert(BmbpDictColumn::DictNamePath, "#,D00")
        .insert(BmbpDictColumn::DictTreeGrade, 1);
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let rs = orm.execute_insert(&insert).await.unwrap();
    println!("{}", rs);
}

#[tokio::test]
async fn delete_delete_dict() {
    let mut delete = DeleteWrapper::new();
    delete.table(BmbpDict::get_table());
    delete.eq_(BmbpDictColumn::DataId, "9a5709fee8bd4c5c9cff0974f0e3982c");
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let rs = orm.execute_delete(&delete).await.unwrap();
    println!("{}", rs);
}

#[tokio::test]
async fn test_update_path() {
    let mut update = UpdateWrapper::new();
    update
        .table(BmbpDict::get_table())
        .set(
            BmbpDictColumn::DictNamePath,
            RdbcColumn::replace(BmbpDictColumn::DictNamePath.get_ident(), "#,DD", "#,BB"),
        )
        .set(
            BmbpDictColumn::DictCodePath,
            RdbcColumn::replace(
                BmbpDictColumn::DictCodePath.get_ident(),
                "#,0b1bf8a785fb4db38809de24826e41d1,",
                "#,4aa4a14c781f4a9c952e7879b2d9aeb6,",
            ),
        );
    update.like_left_(
        BmbpDictColumn::DictCodePath,
        "#,0b1bf8a785fb4db38809de24826e41d1,",
    );
    tracing_subscriber::fmt().init();
    let orm = build_orm().await;
    let rs = orm.execute_update(&update).await.unwrap();
    println!("{}", rs);
}
