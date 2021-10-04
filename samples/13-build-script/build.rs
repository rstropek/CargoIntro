fn main() {
    cc::Build::new()
        .file("src/qps.c")
        .compile("qps");
    println!("cargo:rerun-if-changed=src/hello.c");
}
