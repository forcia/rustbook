fn main() {
    cc::Build::new()
        .flag("-v")
        .file("src/hello.c")
        .compile("hello");
}
