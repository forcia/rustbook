use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeName)]
pub fn derive_typename(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl ::typename::TypeNameTrait for #name {
            fn type_name(&self) -> &str {
                stringify!(#name)
            }
        }
    };

    gen.into()
}
