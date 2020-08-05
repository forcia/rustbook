use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::str::FromStr;

#[proc_macro]
pub fn tomlstruct(input: TokenStream) -> TokenStream {
    let mut ret = String::from("");
    for token in input {
        match &token {
            TokenTree::Group(x) => {
                let name = get_struct_name(x).unwrap();
                if ret == "" {
                    ret = format!("struct {} {{", name);
                } else {
                    ret = format!("{}}}\nstruct {} {{", ret, name);
                }
            }
            TokenTree::Ident(x) => {
                ret = format!("{}\n    {}", ret, x.to_string());
            }
            TokenTree::Literal(x) => {
                if x.to_string().starts_with('"') {
                    ret = format!("{}: String,", ret);
                } else {
                    ret = format!("{}: f64,", ret);
                }
            }
            _ => (),
        }
    }
    ret = format!("{}\n}}", ret);
    //println!("{}", &ret);
    FromStr::from_str(&ret).unwrap()
}

fn get_struct_name(input: &Group) -> Option<String> {
    match input.delimiter() {
        Delimiter::Bracket => {
            for token in input.stream() {
                if let TokenTree::Ident(x) = token {
                    return Some(x.to_string());
                }
            }
        }
        _ => (),
    }
    None
}
