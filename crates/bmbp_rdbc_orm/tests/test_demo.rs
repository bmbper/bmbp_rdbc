use bmbp_rdbc_orm::RdbcSqlTrait;
use bmbp_rdbc_sql::{
    RdbcDeleteBuilder, RdbcFilterBuilder, RdbcInsertBuilder,
    RdbcQueryBuilder, RdbcTableBuilder, RdbcUpdateBuilder,
};
use bmbp_rdbc_type::{RdbcError, RdbcIdent, RdbcTableIdent};
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
pub enum BmbpDictColumn {
    DictValue,
    DictAlias,
    DictCode,
    DictParentCode,
    DictName,
    DictCodePath,
    DictNamePath,
    DictTreeGrade,
    DictLeaf,
    DictType,
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
}
impl RdbcIdent for BmbpDictColumn {
    fn name(&self) -> String {
        match self {
            BmbpDictColumn::DictValue => "dict_value".to_string(),
            BmbpDictColumn::DictAlias => "dict_alias".to_string(),
            BmbpDictColumn::DictCode => "dict_code".to_string(),
            BmbpDictColumn::DictParentCode => "dict_parent_code".to_string(),
            BmbpDictColumn::DictName => "dict_name".to_string(),
            BmbpDictColumn::DictCodePath => "dict_code_path".to_string(),
            BmbpDictColumn::DictNamePath => "dict_name_path".to_string(),
            BmbpDictColumn::DictTreeGrade => "dict_tree_grade".to_string(),
            BmbpDictColumn::DictLeaf => "dict_leaf".to_string(),
            BmbpDictColumn::DictType => "dict_type".to_string(),
            BmbpDictColumn::DataId => "data_id".to_string(),
            BmbpDictColumn::DataLevel => "data_level".to_string(),
            BmbpDictColumn::DataFlag => "data_flag".to_string(),
            BmbpDictColumn::DataStatus => "data_status".to_string(),
            BmbpDictColumn::DataSort => "data_sort".to_string(),
            BmbpDictColumn::DataCreateTime => "data_create_time".to_string(),
            BmbpDictColumn::DataCreateUser => "data_create_user".to_string(),
            BmbpDictColumn::DataUpdateTime => "data_update_time".to_string(),
            BmbpDictColumn::DataUpdateUser => "data_update_user".to_string(),
            BmbpDictColumn::DataOwnerOrg => "data_owner".to_string(),
            BmbpDictColumn::DataSign => "data_sign".to_string(),
        }
    }
}

impl RdbcTableIdent for BmbpDict {
    fn table_name() -> String {
        "bmbp_config_dict".to_string()
    }

    fn columns() -> Vec<String> {
        vec![
            "dict_value".to_string(),
            "dict_alias".to_string(),
            "dict_code".to_string(),
            "dict_parent_code".to_string(),
            "dict_name".to_string(),
            "dict_code_path".to_string(),
            "dict_name_path".to_string(),
            "dict_tree_grade".to_string(),
            "dict_leaf".to_string(),
            "dict_type".to_string(),
            "data_id".to_string(),
            "data_level".to_string(),
            "data_flag".to_string(),
            "data_status".to_string(),
            "data_sort".to_string(),
            "data_create_time".to_string(),
            "data_create_user".to_string(),
            "data_update_time".to_string(),
            "data_update_user".to_string(),
            "data_owner".to_string(),
            "data_sign".to_string(),
        ]
    }

    fn primary_key() -> String {
        "data_id".to_string()
    }

    fn status_key() -> String {
        "data_status".to_string()
    }
    fn logic_delete_key() -> String {
        "data_flag".to_string()
    }
}

impl RdbcSqlTrait<BmbpDict> for BmbpDict {
    fn rdbc_query_all() -> Result<RdbcQueryBuilder, RdbcError> {
        let mut builder = RdbcQueryBuilder::new();
        builder.table(BmbpDict::table_name());
        builder.select_vec(BmbpDict::columns());
        Ok(builder)
    }

    fn rdbc_disable_all() -> Result<RdbcUpdateBuilder, RdbcError> {
        let mut update = RdbcUpdateBuilder::new();
        update.table(BmbpDict::table_name());
        update.set(BmbpDict::status_key(), "0");
        Ok(update)
    }

    fn rdbc_enable_all() -> Result<RdbcUpdateBuilder, RdbcError> {
        let mut update = RdbcUpdateBuilder::new();
        update.table(BmbpDict::table_name());
        update.set(BmbpDict::status_key(), "1");
        Ok(update)
    }

    fn rdbc_delete_all() -> Result<RdbcDeleteBuilder, RdbcError> {
        let mut delete = RdbcDeleteBuilder::new();
        delete.table(BmbpDict::table_name());
        Ok(delete)
    }

    fn rdbc_delete_logic_all() -> Result<RdbcUpdateBuilder, RdbcError> {
        let mut update = RdbcUpdateBuilder::new();
        update.table(BmbpDict::table_name());
        update.set(BmbpDict::logic_delete_key(), "Y");
        Ok(update)
    }

    fn rdbc_query(&self) -> Result<RdbcQueryBuilder, RdbcError> {
        let mut builder = RdbcQueryBuilder::new();
        builder.table(BmbpDict::table_name());
        builder.select_vec(BmbpDict::columns());
        Ok(builder)
    }

    fn rdbc_query_info(&self) -> Result<RdbcQueryBuilder, RdbcError> {
        let mut builder = RdbcQueryBuilder::new();
        builder.table(BmbpDict::table_name());
        builder.select_vec(BmbpDict::columns());
        builder.eq_v(BmbpDict::primary_key(), self.data_id.clone());
        Ok(builder)
    }

    fn rdbc_insert(&self) -> Result<RdbcInsertBuilder, RdbcError> {
        let mut insert = RdbcInsertBuilder::new();
        insert.insert_table(BmbpDict::table_name());
        if let Some(data_create_time) = self.data_create_time.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataCreateTime, data_create_time);
        }
        if let Some(data_create_user) = self.data_create_user.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataCreateUser, data_create_user);
        }
        if let Some(data_update_time) = self.data_update_time.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataUpdateTime, data_update_time);
        }
        if let Some(data_update_user) = self.data_update_user.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataUpdateUser, data_update_user);
        }
        if let Some(data_owner_org) = self.data_owner_org.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataOwnerOrg, data_owner_org);
        }
        if let Some(data_sign) = self.data_sign.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataSign, data_sign);
        }
        if let Some(data_flag) = self.data_flag.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataFlag, data_flag);
        }
        if let Some(data_status) = self.data_status.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataStatus, data_status);
        }
        if let Some(data_id) = self.data_id.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataId, data_id);
        }
        if let Some(data_level) = self.data_level.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataLevel, data_level);
        }
        if let Some(data_sort) = self.data_sort.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DataSort, data_sort);
        }
        if let Some(dict_value) = self.dict_value.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictValue, dict_value);
        }
        if let Some(dict_alias) = self.dict_alias.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictAlias, dict_alias);
        }
        if let Some(dict_code) = self.dict_code.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictCode, dict_code);
        }
        if let Some(dict_parent_code) = self.dict_parent_code.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictParentCode, dict_parent_code);
        }
        if let Some(dict_name) = self.dict_name.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictName, dict_name);
        }
        if let Some(dict_type) = self.dict_type.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictType, dict_type);
        }
        if let Some(dict_leaf) = self.dict_leaf.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictLeaf, dict_leaf);
        }
        if let Some(dict_code_path) = self.dict_code_path.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictCodePath, dict_code_path);
        }
        if let Some(dict_name_path) = self.dict_name_path.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictNamePath, dict_name_path);
        }
        if let Some(dict_tree_grade) = self.dict_tree_grade.as_ref() {
            insert.insert_col_val(BmbpDictColumn::DictTreeGrade, dict_tree_grade);
        }
        Ok(insert)
    }

    fn rdbc_insert_with_none(&self) -> Result<RdbcInsertBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_insert_with_empty(&self) -> Result<RdbcInsertBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_insert_with_all(&self) -> Result<RdbcInsertBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_with_none(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_with_empty(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_with_all(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_with_none_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_with_empty_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_update_with_all_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_disable_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_enable_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_delete_by_id(&self) -> Result<RdbcDeleteBuilder, RdbcError> {
        todo!()
    }

    fn rdbc_delete_logic_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError> {
        todo!()
    }
}

#[test]
fn test_demo() {}
