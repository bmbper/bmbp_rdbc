use bmbp_rdbc_type::{Executor, RdbcError, RdbcIdent, RdbcPage, RdbcRow};
pub trait RdbcCurdTrait<T>
where
    RdbcRow: From<T>,
{
    fn select_all_page(
        executor: &dyn Executor,
        page_num: usize,
        page_size: usize,
    ) -> Result<RdbcPage<T>, RdbcError>;
    fn select_all_list(executor: &dyn Executor) -> Result<Vec<T>, RdbcError>;
    fn select_one_by_id<T>(executor: &dyn Executor, data_id: T) -> Result<Option<T>, RdbcError>
    where
        T: RdbcIdent;
    fn remove_by_id<T>(executor: &dyn Executor, data_id: T) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    fn enable_by_id<T>(executor: &dyn Executor, data_id: T) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    fn disable_by_id<T>(executor: &dyn Executor, data_id: T) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    fn logic_remove_by_id<T>(executor: &dyn Executor, data_id: T) -> Result<usize, RdbcError>
    where
        T: RdbcIdent;
    fn login_remove_column() -> String;
}
