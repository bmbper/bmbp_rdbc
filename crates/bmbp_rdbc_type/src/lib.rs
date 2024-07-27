pub use consts::*;
pub use model::*;
pub use tree::*;
pub use tree_util::*;
pub use valid::*;
mod consts;
mod model;
mod tree;
mod tree_util;
mod valid;

pub trait BmbpRdbcTable {
    fn get_table_name() -> String;
    fn get_table_columns() -> Vec<String>;
    fn get_table_primary_key() -> String;
    fn get_table_union_primary_key() -> Vec<String>;
}
