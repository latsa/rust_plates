extern crate libc;
extern crate nng_sys;

use std::env;

use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void};
use nng_sys::{nn_socket, nn_bind, nn_connect, nn_poll, nn_send, nn_close, nn_errno, nn_strerror};

fn puller(url: &CStr) {

   unsafe {
      let input = nn_socket(nng_sys::AF_SP as c_int, nng_sys::NN_PULL);
      nn_bind (input, url.as_ptr());
      println!("Puller listening on '{}'.", url.to_str().unwrap());

      let mut pfd = nng_sys::nn_pollfd {
         fd: input,
         events: nng_sys::NN_POLLIN as u16 ,
         revents: nng_sys::NN_ZERO as u16
      };

      let timeout = 10000;
      match  nn_poll (&mut pfd, 1, timeout) {
         0 => {
            println!("Puller timed out.");
            exit(0);
         },
         -1 => {
            let errno = nn_errno();
            let errstr = CStr::from_ptr(nn_strerror(errno));
            println!("Puller failed '{}'.", errstr.to_str().unwrap());
            exit(errno as i32);
         },
         _ => {
            println!("Puller got a message.");

            // TODO: Allocate a buffer and read the actual message.

         }
      }

   }
}

fn pusher(url: &CStr) {
   unsafe {
      let output = nn_socket(nng_sys::AF_SP as c_int, nng_sys::NN_PUSH);
      nn_connect (output, url.as_ptr());

      let msg = CString::new("Hello").unwrap();

      match nn_send (output, msg.into_raw() as *const c_void, 5, 0) {
         -1 => {
            let errno = nn_errno();
            let errstr = CStr::from_ptr(nn_strerror(errno));
            println!("Pusher failed '{}'.", errstr.to_str().unwrap());
            exit(errno as i32);
         },
         _ => {
            println!("Pusher sent a message.");
         }
      }

      match nn_close(output) {
         -1 => {
            let errno = nn_errno();
            let errstr = CStr::from_ptr(nn_strerror(errno));
            println!("Pusher failed '{}'.", errstr.to_str().unwrap());
            exit(errno as i32);
         },
         0 => {
            println!("Pusher sez bye.");
         },
         _ => {
            println!("Pusher sez wtf.");
         }
      }
   }
}

fn exit(code:i32) {
   unsafe { libc::exit(code); };
}

fn usage(program_name : &String) {
   println!("");
   println!("Usage: {} [push|pull]",program_name);
   println!("");
   exit(1);
}

fn env_is_set(envvar:&str) -> bool {
    match env::var(envvar) {
        Ok(_) => true,
        _ => false
    }
}

fn main() {
   let url = CString::new("ipc:///tmp/pipeline1.ipc").unwrap();
   let args: Vec<String> = env::args().collect();

   let program_name = &args[0];
   if args.len() == 1 {

     if !env_is_set("mode") {
         usage(program_name);
     }

     let mode: &str = &env::var("mode").unwrap();

     match mode {
       "push" => pusher(&url),
       "pull" => puller(&url),
       _ => usage(program_name)
     }
   } else if args.len() == 2 {
      let mode:&str = args[1].as_ref();

      match mode {
        "push" => pusher(&url),
        "pull" => puller(&url),
        _ => usage(program_name)
      }
   } else {
      usage(program_name);
   }
   exit(0);
}
