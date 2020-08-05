pub use typename_derive::TypeName;

pub trait TypeNameTrait {
    fn type_name(&self) -> &str;
}
