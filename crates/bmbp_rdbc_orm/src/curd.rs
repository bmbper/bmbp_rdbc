use crate::exec::Executor;
use bmbp_rdbc_sql::{RdbcDelete, RdbcInsert, RdbcQuery, RdbcUpdate, RdbcWhereFilter};
use bmbp_rdbc_type::{RdbcError, RdbcIdent, RdbcPage, RdbcRow, RdbcTableIdent};
use serde::Serialize;
use std::fmt::Debug;

pub trait RdbcCurdTrait<T>
where
    T: From<RdbcRow>,
    T: RdbcSqlTrait<T>,
    T: RdbcTableIdent,
    T: Default + Debug + Clone + Serialize,
{
    // 全部分页
    fn select_page_all(
        executor: &(impl Executor),
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 全部列表
    fn select_list_all(executor: &(impl  Executor)) -> Result<Vec<T>, RdbcError>;
    fn select_page_by_query(
        executor:&(impl  Executor),
        page_num: usize,
        page_size: usize,
        query: RdbcQuery,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    fn select_list_by_query(
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
        query: RdbcQuery,
    ) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    fn select_one_by_query(
        executor:&(impl  Executor),
        query: RdbcQuery,
    ) -> Result<Option<T>, RdbcError>;
    fn select_page_by_filter(
        executor:&(impl  Executor),
        page_num: usize,
        page_size: usize,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    fn select_list_by_filter(
        executor:&(impl  Executor),
        filter: Option<RdbcWhereFilter>,
    ) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    fn select_one_by_filter(
        executor:&(impl  Executor),
        filter: Option<RdbcWhereFilter>,
    ) -> Result<Option<T>, RdbcError>;
    fn select_one_by_id<I>(executor:&(impl  Executor), data_id: I) -> Result<Option<T>, RdbcError>
    where
        I: RdbcIdent;

    // 根据内置条件查询分页
    fn select_page(
        &self,
        executor:&(impl  Executor),
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    fn select_list(&self, executor:&(impl  Executor)) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    fn select_one(&self, executor:&(impl  Executor)) -> Result<Option<T>, RdbcError>;

    // 插入值，仅插入有效值
    fn insert(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    // 插入值，包含null值
    fn insert_with_none(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    // 插入值包含空值
    fn insert_with_empty(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    // 插入值，包含所有值
    fn insert_with_all(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn insert_custom(executor:&(impl  Executor), insert: RdbcInsert) -> Result<usize, RdbcError>;

    // 更新，仅插入有效值
    fn update(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    // 更新值，包含null值
    fn update_with_none(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    // 更新值，包含空值
    fn update_with_empty(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    // 更新值，包含所有值
    fn update_with_all(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;

    fn update_custom(executor:&(impl  Executor)) -> Result<usize, RdbcError>;

    // 更新值，仅插入有效值
    fn update_by_id<I>(&self, executor:&(impl  Executor), data_id: T) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    // 更新值，包含null值
    fn update_with_none_by_id<I>(
        &self,
        executor:&(impl  Executor),
        data_id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    // 更新值包含空值
    fn update_with_empty_by_id<I>(
        &self,
        executor:&(impl  Executor),
        data_id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    // 更新值，包含所有值
    fn update_with_all_by_id<I>(
        &self,
        executor:&(impl  Executor),
        data_id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;

    // 更新值，仅插入有效值
    fn update_by_filter(
        &self,
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含null值
    fn update_with_none_by_filter(
        &self,
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值包含空值
    fn update_with_empty_by_filter(
        &self,
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含所有值
    fn update_with_all_by_filter(
        &self,
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;

    fn delete(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn delete_all(executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn delete_by_id<I>(executor:&(impl  Executor), id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn delete_by_ids<I>(executor:&(impl  Executor), ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn delete_by_filter(
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn delete_by_custom(executor:&(impl  Executor), delete: RdbcDelete) -> Result<usize, RdbcError>;

    fn logic_delete(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn logic_delete_all(executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn logic_delete_by_id<I>(executor:&(impl  Executor), id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn logic_delete_by_ids<I>(executor:&(impl  Executor), ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn logic_delete_by_filter(
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn logic_delete_by_custom(
        executor:&(impl  Executor),
        delete: RdbcUpdate,
    ) -> Result<usize, RdbcError>;

    fn enable(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn enable_all(executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn enable_by_id<I>(executor:&(impl  Executor), id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn enable_by_ids<I>(executor:&(impl  Executor), ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn enable_by_filter(
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn enable_by_custom(executor:&(impl  Executor), update: RdbcUpdate) -> Result<usize, RdbcError>;
    fn disable(&self, executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn disable_all(executor:&(impl  Executor)) -> Result<usize, RdbcError>;
    fn disable_by_id<I>(executor:&(impl  Executor), id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn disable_by_ids<I>(executor:&(impl  Executor), ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn disable_by_filter(
        executor:&(impl  Executor),
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn disable_by_custom(executor:&(impl  Executor), update: RdbcUpdate) -> Result<usize, RdbcError>;
}

pub trait RdbcSqlTrait<T>
where
    T: RdbcTableIdent,
{
    fn rdbc_query_all() -> Result<RdbcQuery, RdbcError>;
    fn rdbc_disable_all() -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_enable_all() -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_delete_all() -> Result<RdbcDelete, RdbcError>;
    fn rdbc_delete_logic_all() -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_query(&self) -> Result<RdbcQuery, RdbcError>;
    fn rdbc_query_info(&self) -> Result<RdbcQuery, RdbcError>;
    fn rdbc_insert(&self) -> Result<RdbcInsert, RdbcError>;
    fn rdbc_insert_with_none(&self) -> Result<RdbcInsert, RdbcError>;
    fn rdbc_insert_with_empty(&self) -> Result<RdbcInsert, RdbcError>;
    fn rdbc_insert_with_all(&self) -> Result<RdbcInsert, RdbcError>;
    fn rdbc_update(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_with_none(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_with_empty(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_with_all(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_with_none_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_with_empty_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_update_with_all_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_disable_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_enable_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
    fn rdbc_delete_by_id(&self) -> Result<RdbcDelete, RdbcError>;
    fn rdbc_delete_logic_by_id(&self) -> Result<RdbcUpdate, RdbcError>;
}
