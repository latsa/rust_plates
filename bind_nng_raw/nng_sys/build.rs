extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
   // Build the native library
   let dst = cmake::build("nng");

   // Tell cargo to tell rustc to link the nng library.
   println!("cargo:rustc-link-lib=nng");
   println!("cargo:rustc-link-search={}/lib", dst.display());

   // The bindgen::Builder is the main entry point
   // to bindgen, and lets you build up options for
   // the resulting bindings.
   let bindings = bindgen::Builder::default()
      // Do not generate unstable Rust code that requires a nightly rustc and enabling unstable features.
      .no_unstable_rust()

      // The input header we would like to generate bindings for.
      .header("wrapper.h")

      // Finish the builder and generate the bindings.
      .generate()

      // Unwrap the Result and panic on failure.
      .expect("Unable to generate bindings");

   // Write the bindings to the $OUT_DIR/bindings.rs file.
   let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
   bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
}
