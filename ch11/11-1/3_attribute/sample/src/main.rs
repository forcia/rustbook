use funclog::funclog;

#[funclog]
fn hello() {
    println!("Hello, world!");
}

fn main() {
    hello();
}
