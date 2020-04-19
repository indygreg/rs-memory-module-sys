fn main() {
    // Skip C compilation if building on docs.rs.
    if cfg!(not(memory_module_sys_docs_rs)) {
        println!("cargo:rerun-if-changed=src/memorymodule.c");
        println!("cargo:rerun-if-changed=src/memorymodule.h");

        cc::Build::new()
            .file("src/memorymodule.c")
            .compile("memorymodule");
    }
}
