fn main() {
    println!("cargo:rustc-link-lib=dylib=openblas"); // Link OpenBLAS
    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/openblas/lib"); // Path to OpenBLAS
}
