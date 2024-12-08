use crate::{RdbcOrmRow, RdbcValue};
use bytes::BytesMut;
use std::clone;
use tokio_postgres::types::IsNull;
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
                            .insert(col_name, RdbcValue::Varchar(value));
                    }
                }
                "int2" => {
                    let col_value: Option<i16> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Int(value as i32));
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
                            .insert(col_name, RdbcValue::Double(value));
                    }
                }
                "date" | "time" | "timestamp" => {
                    let col_value: Option<chrono::DateTime<chrono::Utc>> =
                        row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::DateTime(value.naive_local()));
                    }
                }
                "bool" => {
                    let col_value: Option<bool> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Boolean(value));
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
            RdbcValue::Char(v) => v.to_string().to_sql(ty, w),
            RdbcValue::Varchar(v) => v.to_sql(ty, w),
            RdbcValue::Text(v) => v.to_sql(ty, w),
            RdbcValue::LongText(v) => v.to_sql(ty, w),
            RdbcValue::SmallInt(v) => v.to_sql(ty, w),
            RdbcValue::Int(v) => v.to_sql(ty, w),
            RdbcValue::BigInt(v) => v.to_sql(ty, w),
            RdbcValue::Double(v) => v.to_sql(ty, w),
            RdbcValue::BigDouble(v) => v.to_sql(ty, w),
            RdbcValue::Date(v) => v.to_sql(ty, w),
            RdbcValue::DateTime(v) => v.to_sql(ty, w),
            RdbcValue::Time(v) => v.to_sql(ty, w),
            RdbcValue::TimeStamp(v) => (v.clone() as i64).to_sql(ty, w),
            RdbcValue::Bytes(v) => v.to_sql(ty, w),
            RdbcValue::Boolean(v) => v.to_sql(ty, w),
            RdbcValue::Array(v) => v.to_sql(ty, w),
            RdbcValue::Object(v) => match serde_json::to_string(v) {
                Ok(v) => v.to_sql(ty, w),
                Err(e) => e.to_string().to_sql(ty, w),
            },
            RdbcValue::Null => Ok(tokio_postgres::types::IsNull::Yes),
        }
    }
    fn accepts(_ty: &tokio_postgres::types::Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    to_sql_checked!();
}
