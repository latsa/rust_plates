extern crate cmake;

fn main() {
   // Build the native library
   let dst = cmake::build("nng");

   // Tell cargo to tell rustc to link the nng library.
   println!("cargo:rustc-link-lib=nng");
   println!("cargo:rustc-link-search={}/lib", dst.display());
}
