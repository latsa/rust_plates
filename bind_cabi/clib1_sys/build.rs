extern crate cmake;

fn main() {
   let dst = cmake::build("clib1");
   println!("cargo:rustc-link-search={}", dst.display());
   println!("cargo:rustc-link-lib=clib1");
}