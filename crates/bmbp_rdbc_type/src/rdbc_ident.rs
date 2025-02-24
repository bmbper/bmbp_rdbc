pub trait RdbcIdent {
    fn name(&self) -> String;
}
macro_rules! number_ident {
    ( $( $t:ty ),* ) => {
        $(
            impl RdbcIdent for $t {
                fn name(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}
number_ident!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
macro_rules! string_ident {
    ( $( $t:ty ),* ) => {
        $(
            impl RdbcIdent for $t {
                fn name(&self) -> String {
                    format!("{}",self.to_string())
                }
            }
        )*
    };
}
string_ident!(&str, String, &String);


pub trait RdbcValueIdent {
    fn value(&self) -> String;
}

macro_rules! select_number_value {
    ( $( $t:ty ),* ) => {
        $(
            impl RdbcValueIdent for $t {
                fn value(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}
select_number_value!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
macro_rules! select_string_value {
    ( $( $t:ty ),* ) => {
        $(
            impl RdbcValueIdent for $t {
                fn value(&self) -> String {
                    format!("'{}'",self.to_string())
                }
            }
        )*
    };
}
select_string_value!(&str, String, &String);

pub trait RdbcTableIdent {
    fn table_name() -> String;
    fn columns() -> Vec<String>;
    fn primary_key() -> String;
    fn unique_keys() -> Vec<String> {
        vec![]
    }
    fn table_alias() -> String {
        "".to_string()
    }
    fn status_key() -> String {
        "".to_string()
    }
    fn order_key() -> String {
        "".to_string()
    }
    fn logic_delete_key() -> String {
        "".to_string()
    }
}
pub trait RdbcColumnIdent {
    fn table_name() -> String {
        "".to_string()
    }
    fn table_alis() -> String {
        "".to_string()
    }
    fn column_name() -> String;
}
