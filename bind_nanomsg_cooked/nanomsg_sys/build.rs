extern crate cmake;

fn main() {
   // Build the native library
   let dst = cmake::build("nanomsg");

   // Tell cargo to tell rustc to link the nng library.
   println!("cargo:rustc-link-lib=nanomsg");
   println!("cargo:rustc-link-search={}/lib", dst.display());
}
