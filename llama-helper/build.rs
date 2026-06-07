fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    let cc = std::env::var("CC").unwrap_or_default();
    let using_clang = cc.contains("clang");

    // When llama-cpp-sys-2 is built with clang-cl /openmp on Windows ARM64
    // it emits LLVM OpenMP (__kmpc_*) calls but never emits
    // cargo:rustc-link-lib for libomp, so rustc's final link fails with
    // LNK2019 on every OpenMP symbol. Wire it in here. The clang gate
    // matters because MSVC cl.exe (the default on x86_64-pc-windows-msvc)
    // uses vcomp instead, auto-linked via its own /DEFAULTLIB pragma — no
    // help needed there, and forcing libomp would over-link.
    if target == "aarch64-pc-windows-msvc" && using_clang {
        if let Ok(vc) = std::env::var("VCINSTALLDIR") {
            let llvm_lib = std::path::Path::new(&vc)
                .join("Tools")
                .join("Llvm")
                .join("ARM64")
                .join("lib");
            println!("cargo:rustc-link-search=native={}", llvm_lib.display());
        }
        println!("cargo:rustc-link-lib=libomp");
        println!("cargo:rerun-if-env-changed=VCINSTALLDIR");
        println!("cargo:rerun-if-env-changed=CC");
    }
}
