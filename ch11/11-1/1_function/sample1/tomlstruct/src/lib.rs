use proc_macro::TokenStream;

#[proc_macro]
pub fn tomlstruct(input: TokenStream) -> TokenStream {
    dbg!(&input);
    input
}
