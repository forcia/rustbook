use tomlstruct::tomlstruct;

tomlstruct! {
    struct A {
        x: i32
    }
}

fn main() {
    let x = A { x: 0 };
    dbg!(x.x);
}
