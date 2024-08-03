mod table;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn table(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = table::marco_table(bean_meta_token, bean_struct_token);
    println!("==>{}",token.to_string());
    token
}