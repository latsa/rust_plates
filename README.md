This repository contains some boilerplate projects meant to be
used as a starting point.


 - `bind_cabi` - a simple Rust project that uses a static library written in C.
   To build and run the project, enter the corresponding directory and type
   `cargo run`.

 - `nanomsg_raw` - Same as above, except that it statically links to a third
    party library, named [nanomsg](https://github.com/nanomsg/nanomsg).
    rust-bindgen was used to generate the language bindings from C to Rust. Run
   `cargo build` to build the project. run `nanomsg_pull&` and `nanomsg_push`
    to run a sample application.

 - `nng_raw` - Same as `nanomsg_raw`, except that it links to the
    [nng](https://github.com/nanomsg/nng) library.

 - `nanomsg_cooked` - Same as above, except that it statically links to a third
   party library, named [nanomsgrs](https://github.com/thehydroimpulse/nanomsg.rs).

 - `nanomsg_compat_cooked` - This one links to a tweaked [nanomsgrs](https://github.com/thehydroimpulse/nanomsg.rs)
    which uses [nng](https://github.com/nanomsg/nng) in compatibility mode.

 - `nanomsg_cooked` - Upcoming, higher level interface to [nng](https://github.com/nanomsg/nng).
