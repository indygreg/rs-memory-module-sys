fn main() {
    println!("cargo:rerun-if-changed=src/memorymodule.c");
    println!("cargo:rerun-if-changed=src/memorymodule.h");

    cc::Build::new()
        .file("src/memorymodule.c")
        .compile("memorymodule");
}
