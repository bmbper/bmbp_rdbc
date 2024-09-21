use crate::{RdbcOrmRow, RdbcValue};
use bytes::BytesMut;
use tokio_postgres::{
    types::{to_sql_checked, ToSql},
    Row,
};
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

impl ToSql for RdbcValue {
    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        w: &mut BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        match self {
            RdbcValue::Int(i) => i.to_sql(ty, w),
            RdbcValue::BigInt(i) => i.to_sql(ty, w),
            RdbcValue::Float(i) => i.to_sql(ty, w),
            RdbcValue::BigFloat(i) => i.to_sql(ty, w),
            RdbcValue::String(s) => s.to_sql(ty, w),
            RdbcValue::DateTime(dt) => dt.to_sql(ty, w),
            RdbcValue::Bool(b) => b.to_sql(ty, w),
            RdbcValue::Null => Ok(tokio_postgres::types::IsNull::Yes),
            RdbcValue::Vec(v) => v.to_sql(ty, w),
            RdbcValue::Map(_) => Ok(tokio_postgres::types::IsNull::Yes),
        }
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    to_sql_checked!();
}
