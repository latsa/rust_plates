extern crate libc;
extern crate nng_sys;

fn exit(code:i32) {
   unsafe { libc::exit(code); };
}

fn main() {
   exit(0);
}
