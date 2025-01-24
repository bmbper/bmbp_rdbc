
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

#[test]
fn test_demo() {

}