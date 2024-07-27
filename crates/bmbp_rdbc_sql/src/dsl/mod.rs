pub use dsl_func::*;
pub use dsl_model::*;
pub use dsl_trait::*;
pub use dsl_value::*;

mod dsl_func;
mod dsl_model;
mod dsl_trait;
mod dsl_value;
mod func;

#[derive(Clone)]
pub enum DatabaseType {
    MySQL,
    Postgres,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_table() {}
}
