use proc_macro::{ TokenStream};
use bmbp_marco_util::{build_struct_field_token, field_has_attrs_ident, parse_struct_fields};
use case_style::CaseStyle;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Field, parse_macro_input};

pub(crate) fn marco_table(meta_token: TokenStream, struct_token: TokenStream) -> TokenStream {
    let parse_struct_meta_token = struct_token.clone();
    let input_derive = parse_macro_input!(parse_struct_meta_token as syn::DeriveInput);
    let struct_ident = &input_derive.ident;
    let struct_attrs = &input_derive.attrs;
    let table_name = parse_struct_table_name(&meta_token, struct_ident);
    let struct_field = parse_struct_fields(&input_derive);
    let struct_field_token = build_struct_field_token(struct_field.as_slice());
    let struct_column = build_struct_column_enum(struct_ident, struct_field.as_slice());
    let impl_rdbc_ident = build_impl_rdbc_ident(struct_ident, struct_field.as_slice());
    let impl_rdbc_table = build_impl_rdbc_table(struct_ident, &table_name, struct_field.as_slice());
    let token = quote! {
        #(#struct_attrs)*
        pub struct #struct_ident {
            #(#struct_field_token),*
        }
        #struct_column
        #impl_rdbc_ident
        #impl_rdbc_table
    };
    token.into()
}

fn build_struct_column_enum(struct_ident: &Ident, fields: &[Field]) -> TokenStream2 {
    let struct_columns_name = format_ident!("{}Column", struct_ident);
    let column_fields = build_struct_column_enum_field_ident(fields);
    let token = quote! {
        pub enum #struct_columns_name {
            #(#column_fields),*
        }
    };
    token
}
fn build_impl_rdbc_ident(struct_ident: &Ident, fields: &[Field]) -> TokenStream2 {
    let struct_columns_ident = format_ident!("{}Column", struct_ident);
    let match_column_fields = build_impl_rdbc_ident_field_ident(fields);
    let token = quote! {
        impl RdbcIdent for #struct_columns_ident {
            fn get_ident(&self) -> String {
                match self {
                    #(#match_column_fields),*
                }
            }
        }
    };
    token
}
fn build_impl_rdbc_table(struct_ident: &Ident, table_name: &String, fields: &[Field]) -> TokenStream2 {
    let struct_columns_ident = format_ident!("{}Column", struct_ident);
    let mut match_column_fields = build_impl_rdbc_table_field_ident(fields);
    let token = quote! {
        impl RdbcTable for #struct_ident {
            fn get_table() -> impl RdbcIdent {
                #table_name.to_string()
            }
            fn get_columns() -> Vec<impl RdbcIdent> {
                vec![
                    #(#struct_columns_ident::#match_column_fields),*
                ]
            }
        }
    };
    token
}
fn parse_struct_table_name(meta: &TokenStream, struct_ident: &Ident) -> String {
    let mut table_name = meta.to_string().replace("\"", "");
    if table_name.is_empty() {
        table_name = struct_ident.to_string();
    }
    table_name = CaseStyle::guess(table_name).unwrap().to_snakecase().to_uppercase();
    table_name
}
fn build_struct_column_enum_field_ident(fields: &[Field]) -> Vec<Ident> {
    let mut column_fields = vec![];
    for field in fields {
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_vars = CaseStyle::guess(field_name).unwrap().to_pascalcase();
        column_fields.push(format_ident!("{}",enum_vars))
    }
    column_fields
}
fn build_impl_rdbc_ident_field_ident(fields: &[Field]) -> Vec<TokenStream2> {
    let mut column_fields = vec![];
    for field in fields {
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_vars = CaseStyle::guess(field_name.clone()).unwrap().to_pascalcase();
        let enum_ident = format_ident!("{}",enum_vars);
        let token = quote! {
            Self::#enum_ident => #field_name.to_string()
        };
        column_fields.push(token)
    }
    column_fields
}
fn build_impl_rdbc_table_field_ident(fields: &[Field]) -> Vec<Ident> {
    let mut match_column_fields = vec![];
    for field in fields {
        if field_has_attrs_ident(field, "skip") {
            continue;
        }
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_vars = CaseStyle::guess(field_name).unwrap().to_pascalcase();
        let ident = format_ident!("{}",enum_vars);
        match_column_fields.push(ident)
    }
    match_column_fields
}
