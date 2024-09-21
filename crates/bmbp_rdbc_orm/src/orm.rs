use std::fmt::Debug;
use std::sync::Arc;

use serde::Serialize;

use bmbp_rdbc_sql::{DeleteWrapper, InsertWrapper, QueryWrapper, UpdateWrapper};
use bmbp_rdbc_type::{RdbcDataSource, RdbcOrmRow, RdbcPage};

use crate::err::RdbcResult;
use crate::pool::{RdbcConn, RdbcConnPool};

pub struct RdbcOrm {
    datasource: Arc<RdbcDataSource>,
    pool: RdbcConnPool,
}

impl RdbcOrm {
    pub async fn new(data_source: RdbcDataSource) -> RdbcResult<Self> {
        let arc_ds = Arc::new(data_source);
        let arc_pool = RdbcConnPool::new(arc_ds.clone());
        arc_pool.init().await?;
        Ok(RdbcOrm {
            datasource: arc_ds.clone(),
            pool: arc_pool,
        })
    }
}

impl RdbcOrm {
    pub async fn get_conn(&self) -> RdbcResult<RdbcConn> {
        self.pool.get_conn().await
    }
    pub async fn valid(&self) -> RdbcResult<bool> {
        self.pool.valid().await
    }
    pub async fn select_page_by_query<T>(
        &self,
        page_no: usize,
        page_size: usize,
        query: &QueryWrapper,
    ) -> RdbcResult<RdbcPage<T>>
    where
        T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
    {
        let page_no = if page_no < 1 { 1 } else { page_no };
        let page_size = if page_size < 1 { 10 } else { page_size };
        let (row_count, row_ata) = self
            .pool
            .get_conn()
            .await?
            .select_page_by_query(page_no, page_size, query)
            .await?;
        let mut new_page = RdbcPage::<T>::new();
        new_page.set_page_size(page_size);
        new_page.set_page_num(page_no);
        new_page.set_total(row_count);
        let mut data_vec = vec![];
        if let Some(rows) = row_ata {
            for row in rows {
                let v = T::from(row);
                data_vec.push(v);
            }
        }
        new_page.set_data(Some(data_vec));
        Ok(new_page)
    }
    pub async fn select_list_by_query<T>(&self, query: &QueryWrapper) -> RdbcResult<Option<Vec<T>>>
    where
        T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
    {
        let row_op = self
            .pool
            .get_conn()
            .await?
            .select_list_by_query(query)
            .await?;
        match row_op {
            Some(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    let t = T::from(row);
                    list.push(t);
                }
                Ok(Some(list))
            }
            None => Ok(None),
        }
    }
    pub async fn select_one_by_query<T>(&self, query: &QueryWrapper) -> RdbcResult<Option<T>>
    where
        T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
    {
        let row_op = self
            .pool
            .get_conn()
            .await?
            .select_one_by_query(query)
            .await?;
        match row_op {
            Some(row) => Ok(Some(T::from(row))),
            None => Ok(None),
        }
    }
    pub async fn execute_insert(&self, insert: &InsertWrapper) -> RdbcResult<u64> {
        self.pool.get_conn().await?.execute_insert(insert).await
    }
    pub async fn execute_update(&self, update: &UpdateWrapper) -> RdbcResult<u64> {
        self.pool.get_conn().await?.execute_update(update).await
    }
    pub async fn execute_delete(&self, delete: &DeleteWrapper) -> RdbcResult<u64> {
        self.pool.get_conn().await?.execute_delete(delete).await
    }
}
