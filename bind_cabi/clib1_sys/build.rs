fn main() {
    println!("cargo:rustc-link-search=clib1_sys/clib1");
    println!("cargo:rustc-link-lib=clib1");
}