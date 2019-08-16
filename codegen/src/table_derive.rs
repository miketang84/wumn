use quote;
use syn;

pub fn impl_to_table_name(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl wumn_dao::ToTableName for  #name {

            fn to_table_name() -> wumn_dao::TableName {
                wumn_dao::TableName{
                    name: stringify!(#name).to_lowercase().into(),
                    schema: None,
                    alias: None,
                }
            }
        }
    }
}
