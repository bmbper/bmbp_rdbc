use bmbp_rdbc_sql::{
    RdbcDelete, RdbcInsert, RdbcQuery, RdbcUpdate, RdbcWhereFilter,
};
use bmbp_rdbc_type::{Executor, RdbcError, RdbcIdent, RdbcPage, RdbcRow, RdbcTableIdent};
pub trait RdbcCurdTrait<T>
where
    RdbcRow: From<T>,
    T: RdbcSqlTrait<T>,
    T: RdbcTableIdent,
{
    // 全部分页
    fn select_page_all(
        executor: &dyn Executor,
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 全部列表
    fn select_list_all(executor: &dyn Executor) -> Result<Vec<T>, RdbcError>;
    fn select_page_by_query(
        executor: &dyn Executor,
        page_num: usize,
        page_size: usize,
        query: RdbcQuery,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    fn select_list_by_query(
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
        query: RdbcQuery,
    ) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    fn select_one_by_query(
        executor: &dyn Executor,
        query: RdbcQuery,
    ) -> Result<Option<T>, RdbcError>;
    fn select_page_by_filter(
        executor: &dyn Executor,
        page_num: usize,
        page_size: usize,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    fn select_list_by_filter(
        executor: &dyn Executor,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    fn select_one_by_filter(
        executor: &dyn Executor,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<Option<T>, RdbcError>;
    fn select_one_by_id<I>(executor: &dyn Executor, data_id: I) -> Result<Option<T>, RdbcError>
    where
        I: RdbcIdent;

    // 根据内置条件查询分页
    fn select_page(
        &self,
        executor: &dyn Executor,
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    fn select_list(&self, executor: &dyn Executor) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    fn select_one(&self, executor: &dyn Executor) -> Result<Option<T>, RdbcError>;

    // 插入值，仅插入有效值
    fn insert(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    // 插入值，包含null值
    fn insert_with_none(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    // 插入值包含空值
    fn insert_with_empty(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    // 插入值，包含所有值
    fn insert_with_all(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn insert_custom(executor: &dyn Executor, insert: RdbcInsert) -> Result<usize, RdbcError>;

    // 更新，仅插入有效值
    fn update(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    // 更新值，包含null值
    fn update_with_none(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    // 更新值，包含空值
    fn update_with_empty(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    // 更新值，包含所有值
    fn update_with_all(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;

    fn update_custom(executor: &dyn Executor) -> Result<usize, RdbcError>;

    // 更新值，仅插入有效值
    fn update_by_id<T>(&self, executor: &dyn Executor, data_id: T) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    // 更新值，包含null值
    fn update_with_none_by_id<T>(
        &self,
        executor: &dyn Executor,
        data_id: T,
    ) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    // 更新值包含空值
    fn update_with_empty_by_id<T>(
        &self,
        executor: &dyn Executor,
        data_id: T,
    ) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    // 更新值，包含所有值
    fn update_with_all_by_id<T>(
        &self,
        executor: &dyn Executor,
        data_id: T,
    ) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;

    // 更新值，仅插入有效值
    fn update_by_filter(
        &self,
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含null值
    fn update_with_none_by_filter(
        &self,
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值包含空值
    fn update_with_empty_by_filter(
        &self,
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含所有值
    fn update_with_all_by_filter(
        &self,
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;

    fn delete(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn delete_all(executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn delete_by_id<I>(executor: &dyn Executor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn delete_by_ids<I>(executor: &dyn Executor, ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn delete_by_filter(
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn delete_by_custom(executor: &dyn Executor, delete: RdbcDelete) -> Result<usize, RdbcError>;

    fn logic_delete(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn logic_delete_all(executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn logic_delete_by_id<I>(executor: &dyn Executor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn logic_delete_by_ids<I>(executor: &dyn Executor, ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn logic_delete_by_filter(
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn logic_delete_by_custom(executor: &dyn Executor, delete: RdbcUpdate) -> Result<usize, RdbcError>;

    fn enable(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn enable_all(executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn enable_by_id<I>(executor: &dyn Executor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn enable_by_ids<I>(executor: &dyn Executor, ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn enable_by_filter(
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn enable_by_custom(executor: &dyn Executor, update: RdbcUpdate) -> Result<usize, RdbcError>;
    fn disable(&self, executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn disable_all(executor: &dyn Executor) -> Result<usize, RdbcError>;
    fn disable_by_id<I>(executor: &dyn Executor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn disable_by_ids<I>(executor: &dyn Executor, ids: &[I]) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    fn disable_by_filter(
        executor: &dyn Executor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    fn disable_by_custom(executor: &dyn Executor, update: RdbcUpdate) -> Result<usize, RdbcError>;
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
