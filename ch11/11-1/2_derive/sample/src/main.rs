use typename::{TypeName, TypeNameTrait};

#[derive(TypeName)]
struct Hello;

fn main() {
    let x = Hello;
    dbg!(x.type_name());
}
