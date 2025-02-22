use crate::orm::RdbcOrmExecutor;
use bmbp_rdbc_sql::{
    RdbcDelete, RdbcDeleteBuilder, RdbcInsert, RdbcInsertBuilder, RdbcQuery, RdbcQueryBuilder,
    RdbcTableBuilder, RdbcUpdate, RdbcUpdateBuilder, RdbcWhereFilter,
};
use bmbp_rdbc_type::{RdbcError, RdbcIdent, RdbcPage, RdbcRow, RdbcTableIdent};
use serde::Serialize;
use std::fmt::Debug;

pub trait RdbcOrmCurd<T>
where
    T: From<RdbcRow>,
    T: RdbcOrmSqlBuilder<T>,
    T: RdbcTableIdent,
    T: Default + Debug + Clone + Serialize,
{
    // 全部分页
    async fn select_page_all(
        executor: &impl RdbcOrmExecutor,
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 全部列表
    async fn select_list_all(executor: &impl RdbcOrmExecutor) -> Result<Vec<T>, RdbcError>;
    async fn select_page_by_query(
        executor: &impl RdbcOrmExecutor,
        page_num: usize,
        page_size: usize,
        query: RdbcQuery,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    async fn select_list_by_query(
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
        query: RdbcQuery,
    ) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    async fn select_one_by_query(
        executor: &impl RdbcOrmExecutor,
        query: RdbcQuery,
    ) -> Result<Option<T>, RdbcError>;
    async fn select_page_by_filter(
        executor: &impl RdbcOrmExecutor,
        page_num: usize,
        page_size: usize,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    async fn select_list_by_filter(
        executor: &impl RdbcOrmExecutor,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    async fn select_one_by_filter(
        executor: &impl RdbcOrmExecutor,
        filter: Option<RdbcWhereFilter>,
    ) -> Result<Option<T>, RdbcError>;
    async fn select_one_by_id<I>(
        executor: &impl RdbcOrmExecutor,
        data_id: I,
    ) -> Result<Option<T>, RdbcError>
    where
        I: RdbcIdent;

    // 根据内置条件查询分页
    async fn select_page(
        &self,
        executor: &impl RdbcOrmExecutor,
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    // 根据内置条件查询列表
    async fn select_list(&self, executor: &impl RdbcOrmExecutor) -> Result<Vec<T>, RdbcError>;
    // 根据内置条件查询详情
    async fn select_one(&self, executor: &impl RdbcOrmExecutor) -> Result<Option<T>, RdbcError>;

    // 插入值，仅插入有效值
    async fn insert(&self, executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    // 插入值，包含null值
    async fn insert_ignore_none(&self, executor: &impl RdbcOrmExecutor)
    -> Result<usize, RdbcError>;
    // 插入值包含空值
    async fn insert_ignore_empty(
        &self,
        executor: &impl RdbcOrmExecutor,
    ) -> Result<usize, RdbcError>;
    // 插入值，包含所有值
    async fn insert_ignore_both(&self, executor: &impl RdbcOrmExecutor)
    -> Result<usize, RdbcError>;
    async fn insert_custom(
        executor: &impl RdbcOrmExecutor,
        insert: RdbcInsert,
    ) -> Result<usize, RdbcError>;

    // 更新，仅插入有效值
    async fn update(&self, executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    // 更新值，包含null值
    async fn update_ignore_none(&self, executor: &impl RdbcOrmExecutor)
    -> Result<usize, RdbcError>;
    // 更新值，包含空值
    async fn update_ignore_empty(
        &self,
        executor: &impl RdbcOrmExecutor,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含所有值
    async fn update_ignore_both(&self, executor: &impl RdbcOrmExecutor)
    -> Result<usize, RdbcError>;

    async fn update_custom(executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;

    // 更新值，仅插入有效值
    async fn update_by_id<I>(
        &self,
        executor: &impl RdbcOrmExecutor,
        data_id: T,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    // 更新值，包含null值
    async fn update_ignore_none_by_id<I>(
        &self,
        executor: &impl RdbcOrmExecutor,
        data_id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    // 更新值包含空值
    async fn update_ignore_empty_by_id<I>(
        &self,
        executor: &impl RdbcOrmExecutor,
        data_id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    // 更新值，包含所有值
    async fn update_ignore_both_by_id<I>(
        &self,
        executor: &impl RdbcOrmExecutor,
        data_id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;

    // 更新值，仅插入有效值
    async fn update_by_filter(
        &self,
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含null值
    async fn update_ignore_none_by_filter(
        &self,
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值包含空值
    async fn update_ignore_empty_by_filter(
        &self,
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    // 更新值，包含所有值
    async fn update_ignore_both_by_filter(
        &self,
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;

    async fn delete(&self, executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn delete_all(executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn delete_by_id<I>(executor: &impl RdbcOrmExecutor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn delete_by_ids<I>(
        executor: &impl RdbcOrmExecutor,
        ids: &[I],
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn delete_by_filter(
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    async fn delete_by_custom(
        executor: &impl RdbcOrmExecutor,
        delete: RdbcDelete,
    ) -> Result<usize, RdbcError>;

    async fn logic_delete(&self, executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn logic_delete_all(executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn logic_delete_by_id<I>(
        executor: &impl RdbcOrmExecutor,
        id: I,
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn logic_delete_by_ids<I>(
        executor: &impl RdbcOrmExecutor,
        ids: &[I],
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn logic_delete_by_filter(
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    async fn logic_delete_by_custom(
        executor: &impl RdbcOrmExecutor,
        delete: RdbcUpdate,
    ) -> Result<usize, RdbcError>;

    async fn enable(&self, executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn enable_all(executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn enable_by_id<I>(executor: &impl RdbcOrmExecutor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn enable_by_ids<I>(
        executor: &impl RdbcOrmExecutor,
        ids: &[I],
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn enable_by_filter(
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    async fn enable_by_custom(
        executor: &impl RdbcOrmExecutor,
        update: RdbcUpdate,
    ) -> Result<usize, RdbcError>;
    async fn disable(&self, executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn disable_all(executor: &impl RdbcOrmExecutor) -> Result<usize, RdbcError>;
    async fn disable_by_id<I>(executor: &impl RdbcOrmExecutor, id: I) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn disable_by_ids<I>(
        executor: &impl RdbcOrmExecutor,
        ids: &[I],
    ) -> Result<usize, RdbcError>
    where
        I: RdbcIdent;
    async fn disable_by_filter(
        executor: &impl RdbcOrmExecutor,
        filter: RdbcWhereFilter,
    ) -> Result<usize, RdbcError>;
    async fn disable_by_custom(
        executor: &impl RdbcOrmExecutor,
        update: RdbcUpdate,
    ) -> Result<usize, RdbcError>;
}

pub trait RdbcOrmSqlBuilder<T>
where
    T: RdbcTableIdent,
{
    fn rdbc_query_all() -> Result<RdbcQueryBuilder, RdbcError> {
        let mut query = RdbcQueryBuilder::new();
        query.table(T::table_name());
        query.select_vec(T::columns());
        Ok(query)
    }
    fn rdbc_disable_all() -> Result<RdbcUpdateBuilder, RdbcError> {
        let mut update = RdbcUpdateBuilder::new();
        update.table(T::table_name());
        update.set(T::status_key(), "N");
        Ok(update)
    }
    fn rdbc_enable_all() -> Result<RdbcUpdateBuilder, RdbcError> {
        let mut update = RdbcUpdateBuilder::new();
        update.table(T::table_name());
        update.set(T::status_key(), "Y");
        Ok(update)
    }
    fn rdbc_delete_all() -> Result<RdbcDeleteBuilder, RdbcError> {
        let mut delete = RdbcDeleteBuilder::new();
        delete.table(T::table_name());
        Ok(delete)
    }
    fn rdbc_delete_logic_all() -> Result<RdbcUpdateBuilder, RdbcError> {
        let mut update = RdbcUpdateBuilder::new();
        update.table(T::table_name());
        update.set(T::logic_delete_key(), "Y");
        Ok(update)
    }
    fn rdbc_query(&self) -> Result<RdbcQueryBuilder, RdbcError>;
    fn rdbc_query_info(&self) -> Result<RdbcQueryBuilder, RdbcError>;
    fn rdbc_insert(&self) -> Result<RdbcInsertBuilder, RdbcError>;
    fn rdbc_insert_ignore_none(&self) -> Result<RdbcInsertBuilder, RdbcError>;
    fn rdbc_insert_ignore_empty(&self) -> Result<RdbcInsertBuilder, RdbcError>;
    fn rdbc_insert_ignore_both(&self) -> Result<RdbcInsertBuilder, RdbcError>;
    fn rdbc_update(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_ignore_none(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_ignore_empty(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_ignore_both(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_ignore_none_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_ignore_empty_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_update_ignore_both_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_disable_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_enable_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
    fn rdbc_delete_by_id(&self) -> Result<RdbcDeleteBuilder, RdbcError>;
    fn rdbc_delete_logic_by_id(&self) -> Result<RdbcUpdateBuilder, RdbcError>;
}
