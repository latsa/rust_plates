#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub use libc::*;

pub use posix_consts::*;

pub const AF_SP: c_int = 1;
pub const AF_SP_RAW: c_int = 2;
pub const NN_PROTO_PIPELINE: c_int = 5;
pub const NN_PUSH: c_int = NN_PROTO_PIPELINE * 16 + 0;
pub const NN_PULL: c_int = NN_PROTO_PIPELINE * 16 + 1;
pub const NN_MSG: size_t = std::usize::MAX as size_t;
pub const NN_PROTO_REQREP: c_int = 3;
pub const NN_REQ: c_int = NN_PROTO_REQREP * 16 + 0;
pub const NN_REP: c_int = NN_PROTO_REQREP * 16 + 1;
pub const NN_REQ_RESEND_IVL: c_int = 1;
pub const NN_PROTO_PAIR: c_int = 1;
pub const NN_PAIR: c_int = NN_PROTO_PAIR * 16 + 0;
pub const NN_PROTO_BUS: c_int = 7;
pub const NN_BUS: c_int = NN_PROTO_BUS * 16 + 0;
pub const NN_PROTO_PUBSUB: c_int = 2;
pub const NN_PUB: c_int = NN_PROTO_PUBSUB * 16 + 0;
pub const NN_SUB: c_int = NN_PROTO_PUBSUB * 16 + 1;
pub const NN_SUB_SUBSCRIBE: c_int = 1;
pub const NN_SUB_UNSUBSCRIBE: c_int = 2;
pub const NN_PROTO_SURVEY: c_int = 6;
pub const NN_SURVEYOR: c_int = NN_PROTO_SURVEY * 16 + 2;
pub const NN_RESPONDENT: c_int = NN_PROTO_SURVEY * 16 + 3;
pub const NN_SURVEYOR_DEADLINE: c_int = 1;


pub const NN_SOCKADDR_MAX: c_int = 128;

pub const NN_SOL_SOCKET: c_int = 0;

pub const NN_LINGER: c_int = 1;
pub const NN_SNDBUF: c_int = 2;
pub const NN_RCVBUF: c_int = 3;
pub const NN_SNDTIMEO: c_int = 4;
pub const NN_RCVTIMEO: c_int = 5;
pub const NN_RECONNECT_IVL: c_int = 6;
pub const NN_RECONNECT_IVL_MAX: c_int = 7;
pub const NN_SNDPRIO: c_int = 8;
pub const NN_RCVPRIO: c_int = 9;
pub const NN_SNDFD: c_int = 10;
pub const NN_RCVFD: c_int = 11;
pub const NN_DOMAIN: c_int = 12;
pub const NN_PROTOCOL: c_int = 13;
pub const NN_IPV4ONLY: c_int = 14;
pub const NN_SOCKET_NAME: c_int = 15;
pub const NN_RCVMAXSIZE: c_int = 16;
pub const NN_MAXTTL: c_int = 17;

pub const NN_DONTWAIT: c_int = 1;

pub const NN_INPROC: c_int = -1;
pub const NN_IPC: c_int = -2;
pub const NN_TCP: c_int = -3;

pub const NN_TCP_NODELAY: c_int = 1;

pub const NN_POLLIN: c_short = 1;
pub const NN_POLLOUT: c_short = 2;
pub const NN_POLL_IN_AND_OUT: c_short = NN_POLLIN + NN_POLLOUT;

// error codes
pub const ETERM: c_int = posix_consts::NN_HAUSNUMERO + 53;
pub const EFSM: c_int = posix_consts::NN_HAUSNUMERO + 54;

#[cfg(not(windows))]
pub mod posix_consts {
    use libc::*;
    // NOTE:
    // If the platform you are compiling for fails to implement the posix
    // constant, then add an exception below
    // If this fails to compile, then remove it from this re-export, and add
    // an exception below, similar to the macos exceptions already added. If
    // a feature is not implemented on your system, then use an offset to
    // NN_HAUSNUMERO, which nanomsg uses for undefined constants.
    //
    // Use the value from the windows definitions if an override is required.
    pub const NN_HAUSNUMERO: c_int = 156384712;

    // nanomsg uses EACCESS as an alias for EACCES
    pub const EACCESS: c_int = ::libc::EACCES;
}

#[cfg(windows)]
pub mod posix_consts {
    use libc::c_int;

    pub const NN_HAUSNUMERO: c_int = 156384712;

    pub const ENOTSUP:         c_int = NN_HAUSNUMERO + 1;
    pub const EPROTO:          c_int = NN_HAUSNUMERO + 11;
    pub const EACCESS:         c_int = NN_HAUSNUMERO + 17;
    pub const EISCONN:         c_int = NN_HAUSNUMERO + 27;
    pub const ESOCKTNOSUPPORT: c_int = NN_HAUSNUMERO + 28;

    pub const EADDRINUSE:      c_int = 100;
    pub const EADDRNOTAVAIL:   c_int = 101;
    pub const EAFNOSUPPORT:    c_int = 102;
    pub const ECONNABORTED:    c_int = 106;
    pub const ECONNREFUSED:    c_int = 107;
    pub const ECONNRESET:      c_int = 108;
    pub const EHOSTUNREACH:    c_int = 110;
    pub const EINPROGRESS:     c_int = 112;
    pub const EMSGSIZE:        c_int = 115;
    pub const ENETDOWN:        c_int = 116;
    pub const ENETRESET:       c_int = 117;
    pub const ENETUNREACH:     c_int = 118;
    pub const ENOBUFS:         c_int = 119;
    pub const ENOPROTOOPT:     c_int = 123;
    pub const ENOTCONN:        c_int = 126;
    pub const ENOTSOCK:        c_int = 128;
    pub const EPROTONOSUPPORT: c_int = 135;
    pub const ETIMEDOUT:       c_int = 138;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nn_pollfd  {
    fd: c_int,
    events: c_short,
    revents: c_short
}

impl nn_pollfd {
    pub fn new (socket: c_int, pollin: bool, pollout: bool) -> nn_pollfd {
        let ev = match (pollin, pollout) {
            (true, true) => NN_POLL_IN_AND_OUT,
            (false, true) => NN_POLLOUT,
            (true, false) => NN_POLLIN,
            (false, false) => 0
        };

        nn_pollfd { fd: socket, events: ev , revents: 0 }
    }

    pub fn pollin_result(&self) -> bool {
        match self.revents {
            0 => false,
            NN_POLLIN => true,
            NN_POLLOUT => false,
            NN_POLL_IN_AND_OUT => true,
            _ => false
        }
    }

    pub fn pollout_result(&self) -> bool {
        match self.revents {
            0 => false,
            NN_POLLIN => false,
            NN_POLLOUT => true,
            NN_POLL_IN_AND_OUT => true,
           _ => false
         }
    }
}

#[cfg_attr(all(target_os = "linux", feature = "bundled", not(feature = "no_anl")), link(name = "anl"))]
#[cfg_attr(feature = "bundled", link(name = "nanomsg", kind = "static"))]
extern {
    /// "Creates an SP socket with specified domain and protocol. Returns
    /// a file descriptor for the newly created socket."
    ///
    /// http://nanomsg.org/v0.4/nn_socket.3.html
    pub fn nn_socket(domain: c_int, protocol: c_int) -> c_int;

    /// "Closes the socket s. Any buffered inbound messages that were not yet received
    /// by the application will be discarded. The library will try to deliver
    /// any outstanding outbound messages for the time specified by NN_LINGER socket
    /// option. The call will block in the meantime."
    ///
    /// http://nanomsg.org/v0.4/nn_close.3.html
    pub fn nn_close(socket: c_int) -> c_int;

    /// "Sets the value of the option. The level argument specifies the protocol level
    /// at which the option resides. For generic socket-level options use NN_SOL_SOCKET
    /// level. For socket-type-specific options use socket type for level argument
    /// (e.g. NN_SUB). For transport-specific options use ID of the transport as
    /// the level argument (e.g. NN_TCP).
    ///
    /// The new value is pointed to by optval argument. Size of the option is
    /// specified by the optvallen argument."
    ///
    /// http://nanomsg.org/v0.4/nn_setsockopt.3.html
    pub fn nn_setsockopt(socket: c_int, level: c_int, option: c_int, optval: *const c_void,
                         optvallen: size_t) -> c_int;

    /// "Retrieves the value for the option. The level argument specifies the protocol
    /// level at which the option resides. For generic socket-level options use NN_SOL_SOCKET
    /// level. For socket-type-specific options use socket type for level argument
    /// (e.g. NN_SUB). For transport-specific options use ID of the transport as the
    /// level argument (e.g. NN_TCP).
    ///
    /// The value is stored in the buffer pointed to by optval argument. Size of the
    /// buffer is specified by the optvallen argument. If the size of the option is greater
    /// than size of the buffer, the value will be silently truncated. Otherwise,
    /// the optvallen will be modified to indicate the actual length of the option."
    ///
    /// http://nanomsg.org/v0.4/nn_getsockopt.3.html
    pub fn nn_getsockopt(socket: c_int, level: c_int, option: c_int, optval: *mut c_void,
                         optvallen: *mut size_t) -> c_int;
    /// "Adds a local endpoint to the socket s. The endpoint can be then used by other
    /// applications to connect to. The addr argument consists of two parts as follows:
    /// transport://address. The transport specifies the underlying transport protocol
    /// to use. The meaning of the address part is specific to the underlying transport
    /// protocol."
    ///
    /// http://nanomsg.org/v0.4/nn_bind.3.html
    pub fn nn_bind(socket: c_int, addr: *const c_char) -> c_int;

    /// "Adds a remote endpoint to the socket s. The library would then try to connect to the
    /// specified remote endpoint. The addr argument consists of two parts as follows:
    /// transport://address. The transport specifies the underlying transport protocol to use.
    /// The meaning of the address part is specific to the underlying transport protocol."
    ///
    /// http://nanomsg.org/v0.4/nn_connect.3.html
    pub fn nn_connect(socket: c_int, addr: *const c_char) -> c_int;

    /// "Removes an endpoint from socket s. how parameter specifies the ID of the endpoint to
    /// remove as returned by prior call to nn_bind(3) or nn_connect(3). nn_shutdown() call
    /// will return immediately, however, the library will try to deliver any outstanding
    /// outbound messages to the endpoint for the time specified by NN_LINGER socket option."
    ///
    /// http://nanomsg.org/v0.4/nn_shutdown.3.html
    pub fn nn_shutdown(socket: c_int, how: c_int) -> c_int;

    /// "The function will send a message containing the data from buffer pointed to by buf
    /// parameter to the socket s. The message will be len bytes long. Alternatively, to send
    /// a buffer allocated by nn_allocmsg(3) function set the buf parameter to point to the
    /// pointer to the buffer and len parameter to NN_MSG constant. In this case a successful
    /// call to nn_send will deallocate the buffer. Trying to deallocate it afterwards will
    /// result in undefined behaviour.
    ///
    /// Which of the peers the message will be sent to is determined by the particular socket
    /// type."
    ///
    /// http://nanomsg.org/v0.4/nn_send.3.html
    pub fn nn_send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> c_int;

    /// "Receive a message from the socket s and store it in the buffer referenced by the buf
    /// argument. Any bytes exceeding the length specified by the len argument will be truncated.
    ///
    /// Alternatively, nanomsg can allocate the buffer for you. To do so, let the buf parameter
    /// be a pointer to a void* variable (pointer to pointer) to the receive buffer and set the
    /// len parameter to NN_MSG. If the call is successful the user is responsible for
   /// deallocating the message using the nn_freemsg(3) function."
    ///
    /// http://nanomsg.org/v0.4/nn_recv.3.html
    pub fn nn_recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_sendmsg.3.html
    pub fn nn_sendmsg(socket: c_int, msghdr: *const c_void, flags: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_recvmsg.3.html
    pub fn nn_recvmsg(socket: c_int, msghdr: *mut c_void, flags: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_allocmsg.3.html
    pub fn nn_allocmsg(size: size_t, ty: c_int) -> *mut c_void;

    /// http://nanomsg.org/v0.4/nn_reallocmsg.3.html
    pub fn nn_reallocmsg(msg: *mut c_void, size: size_t) -> *mut c_void;

    /// http://nanomsg.org/v0.4/nn_freemsg.3.html
    pub fn nn_freemsg(msg: *mut c_void) -> c_int;

    /// http://nanomsg.org/v0.4/nn_poll.3.html
    pub fn nn_poll(fds: *mut nn_pollfd, nfds: c_int, timeout: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_errno.3.html
    pub fn nn_errno() -> c_int;

    /// http://nanomsg.org/v0.4/nn_strerror.3.html
    pub fn nn_strerror(errnum: c_int) -> *const c_char;

    /// http://nanomsg.org/v0.4/nn_term.3.html
    pub fn nn_term() -> c_void;

    /// http://nanomsg.org/v0.4/nn_device.3.html
    pub fn nn_device(socket1: c_int, socket2: c_int) -> c_int;

    pub fn nn_symbol(index: c_int, value: *mut c_int) -> *const c_char;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;
    use std::ptr;
    use std::mem::transmute;

    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::ffi::CStr;
    use std::str;

    fn sleep_ms(millis: u64) {
        thread::sleep(::std::time::Duration::from_millis(millis));
    }

    fn test_create_socket(domain: c_int, protocol: c_int) -> c_int {
        let sock = unsafe { nn_socket(domain, protocol) };
        assert!(sock >= 0);
        sock
    }

    fn test_bind(socket: c_int, addr: *const c_char) -> c_int {
        let endpoint = unsafe { nn_bind(socket, addr) };
        assert!(endpoint >= 0);
        endpoint
    }

    fn test_connect(socket: c_int, addr: *const c_char) -> c_int {
        let endpoint = unsafe { nn_connect(socket, addr) };
        assert!(endpoint >= 0);
        endpoint
    }

    fn test_send(socket: c_int, msg: &str) {
        let bytes = unsafe {
            nn_send(socket, msg.as_ptr() as *const c_void, msg.len() as size_t, 0)
        };
        let expected = msg.len() as i32;
        assert!(bytes == expected);
    }

    fn test_receive(socket: c_int, expected: &str) {
        let mut buf: *mut u8 = ptr::null_mut();
        let bytes = unsafe { nn_recv(socket, transmute(&mut buf), NN_MSG, 0) };
        assert!(bytes >= 0);
        let msg = unsafe { slice::from_raw_parts_mut(buf, bytes as usize) };
        assert_eq!(msg, expected.as_bytes());
        unsafe { nn_freemsg(buf as *mut c_void); }
    }

    fn test_subscribe(socket: c_int, topic: &str) {
        let topic_len = topic.len() as size_t;
        let topic_ptr = topic.as_ptr();
        let topic_raw_ptr = topic_ptr as *const c_void;
        assert!(unsafe { nn_setsockopt (socket, NN_SUB, NN_SUB_SUBSCRIBE, topic_raw_ptr, topic_len) } >= 0);
    }

    /// This ensures that the one-way pipe works correctly and also serves as an example
    /// on how to properly use the low-level bindings directly, although it's recommended to
    /// use the high-level Rust idiomatic API to ensure safety. The low-level bindings are
    /// quite unsafe to use because there are a lot of unsafe pointers, unsafe blocks, etc...
    #[test]
    fn should_create_a_pipeline() {

        let url = "ipc:///tmp/should_create_a_pipeline.ipc";

        let push_sock = test_create_socket(AF_SP, NN_PUSH);
        let push_endpoint = test_bind(push_sock, url.as_ptr() as *const i8);

        let pull_sock = test_create_socket(AF_SP, NN_PULL);
        let pull_endpoint = test_connect(pull_sock, url.as_ptr() as *const i8);

        let push_msg = "foobar";
        test_send(push_sock, push_msg);
        test_receive(pull_sock, push_msg);

        unsafe {
            nn_shutdown(pull_sock, pull_endpoint);
            nn_close(pull_sock);
            nn_shutdown(push_sock, push_endpoint);
            nn_close(push_sock);
        }
    }

    #[test]
    fn should_create_a_pair() {

        let url = "ipc:///tmp/should_create_a_pair.ipc";
        let left_sock = test_create_socket(AF_SP, NN_PAIR);
        let left_endpoint = test_bind(left_sock, url.as_ptr() as *const i8);

        let right_sock = test_create_socket(AF_SP, NN_PAIR);
        let right_endpoint = test_connect(right_sock, url.as_ptr() as *const i8);

        let right_to_left_msg = "foobar";
        test_send(right_sock, right_to_left_msg);
        test_receive(left_sock, right_to_left_msg);

        let left_to_right_msg = "foobaz";
        test_send(left_sock, left_to_right_msg);
        test_receive(right_sock, left_to_right_msg);

        unsafe {
            nn_shutdown(left_sock, left_endpoint);
            nn_close(left_sock);
            nn_shutdown(right_sock, right_endpoint);
            nn_close(right_sock);
        }
    }

    #[test]
    fn should_create_a_bus() {

        let url = "ipc:///tmp/should_create_a_bus.ipc";

        let sock1 = test_create_socket(AF_SP, NN_BUS);
        let sock1_write_endpoint = test_bind(sock1, url.as_ptr() as *const i8);

        let sock2 = test_create_socket(AF_SP, NN_BUS);
        let sock2_read_endpoint = test_connect(sock2, url.as_ptr() as *const i8);

        let sock3 = test_create_socket(AF_SP, NN_BUS);
        let sock3_read_endpoint = test_connect(sock3, url.as_ptr() as *const i8);

        sleep_ms(10);

        let msg = "foobar";
        test_send(sock1, msg);
        test_receive(sock2, msg);
        test_receive(sock3, msg);

        unsafe {
            nn_shutdown(sock3, sock3_read_endpoint);
            nn_shutdown(sock2, sock2_read_endpoint);
            nn_shutdown(sock1, sock1_write_endpoint);

            nn_close(sock3);
            nn_close(sock2);
            nn_close(sock1);
        }
    }

    #[test]
    fn should_create_a_pubsub() {

        let url = "ipc:///tmp/should_create_a_pubsub.ipc";
        let pub_sock = test_create_socket(AF_SP, NN_PUB);
        let pub_endpoint = test_bind(pub_sock, url.as_ptr() as *const i8);

        let sub_sock1 = test_create_socket(AF_SP, NN_SUB);
        let sub_endpoint1 = test_connect(sub_sock1, url.as_ptr() as *const i8);
        let topic1 = "foo";
        test_subscribe(sub_sock1, topic1);

        let sub_sock2 = test_create_socket(AF_SP, NN_SUB);
        let sub_endpoint2 = test_connect(sub_sock2, url.as_ptr() as *const i8);
        let topic2 = "bar";
        test_subscribe(sub_sock2, topic2);

        sleep_ms(100);

        let msg1 = "foobar";
        test_send(pub_sock, msg1);
        test_receive(sub_sock1, msg1);

        let msg2 = "barfoo";
        test_send(pub_sock, msg2);
        test_receive(sub_sock2, msg2);

        unsafe {
            nn_shutdown(sub_sock2, sub_endpoint2);
            nn_shutdown(sub_sock1, sub_endpoint1);
            nn_shutdown(pub_sock, pub_endpoint);

            nn_close(sub_sock2);
            nn_close(sub_sock1);
            nn_close(pub_sock);
        }
    }

    #[test]
    fn should_create_a_survey() {

        let url = "ipc:///tmp/should_create_a_survey.ipc";
        let surv_sock = test_create_socket(AF_SP, NN_SURVEYOR);
        let surv_endpoint = test_bind(surv_sock, url.as_ptr() as *const i8);

        let resp_sock1 = test_create_socket(AF_SP, NN_RESPONDENT);
        let resp_endpoint1 = test_connect(resp_sock1, url.as_ptr() as *const i8);

        let resp_sock2 = test_create_socket(AF_SP, NN_RESPONDENT);
        let resp_endpoint2 = test_connect(resp_sock2, url.as_ptr() as *const i8);

        sleep_ms(10);

        let survey = "are_you_there";
        test_send(surv_sock, survey);
        test_receive(resp_sock1, survey);
        test_receive(resp_sock2, survey);

        let vote = "yes";
        test_send(resp_sock1, vote);
        test_send(resp_sock2, vote);
        test_receive(surv_sock, vote);
        test_receive(surv_sock, vote);

        unsafe {
            nn_shutdown(resp_sock2, resp_endpoint2);
            nn_shutdown(resp_sock1, resp_endpoint1);
            nn_shutdown(surv_sock, surv_endpoint);

            nn_close(resp_sock2);
            nn_close(resp_sock1);
            nn_close(surv_sock);
        }
    }

    #[test]
    fn poll_should_work() {
        let url = "ipc:///tmp/poll_should_work.ipc";
        let s1 = test_create_socket(AF_SP, NN_PAIR);
        let s2 = test_create_socket(AF_SP, NN_PAIR);
        let pollfd1 = nn_pollfd { fd: s1, events: 3i16, revents: 0i16 };
        let pollfd2 = nn_pollfd { fd: s2, events: 3i16, revents: 0i16 };
        let mut fd_vector: Vec<nn_pollfd> = vec![pollfd1, pollfd2];
        let fd_ptr = fd_vector.as_mut_ptr();

        let poll_result = unsafe { nn_poll(fd_ptr, 2, 0) as usize };
        let fd_slice = &mut fd_vector[..];
        assert_eq!(0, poll_result);
        assert_eq!(0, fd_slice[0].revents);
        assert_eq!(0, fd_slice[1].revents);

        test_bind(s1, url.as_ptr() as *const i8);
        test_connect(s2, url.as_ptr() as *const i8);
        sleep_ms(10);

        let poll_result2 = unsafe { nn_poll(fd_ptr, 2, 10) as usize };
        assert_eq!(2, poll_result2);
        assert_eq!(NN_POLLOUT, fd_slice[0].revents);
        assert_eq!(NN_POLLOUT, fd_slice[1].revents);

        let msg = "foobar";
        test_send(s2, msg);
        sleep_ms(10);

        let poll_result3 = unsafe { nn_poll(fd_ptr, 2, 10) as usize };
        assert_eq!(2, poll_result3);
        assert_eq!(NN_POLLOUT + NN_POLLIN, fd_slice[0].revents);
        assert_eq!(NN_POLLOUT, fd_slice[1].revents);

        unsafe {
            nn_close(s1);
            nn_close(s2);
        }
    }

    fn finish_child_task(checkin: Arc<Barrier>, socket: c_int, endpoint: c_int) {

        checkin.wait();

        unsafe {
            nn_shutdown(socket, endpoint);
            nn_close(socket);
        }
    }

    fn test_multithread_pipeline(url: &'static str) {

        // this is required to prevent the sender from being closed before the receiver gets the message
        let drop_after_use = Arc::new(Barrier::new(2));
        let drop_after_use_pull = drop_after_use.clone();
        let drop_after_use_push = drop_after_use.clone();

        let push_thread = thread::spawn(move || {
            let push_msg = "foobar";
            let push_sock = test_create_socket(AF_SP, NN_PUSH);
            let push_endpoint = test_bind(push_sock, url.as_ptr() as *const i8);

            test_send(push_sock, push_msg);

            finish_child_task(drop_after_use_push, push_sock, push_endpoint);
        });

        let pull_thread = thread::spawn(move || {
            let pull_msg = "foobar";
            let pull_sock = test_create_socket(AF_SP, NN_PULL);
            let pull_endpoint = test_connect(pull_sock, url.as_ptr() as *const i8);

            test_receive(pull_sock, pull_msg);

            finish_child_task(drop_after_use_pull, pull_sock, pull_endpoint);
        });

        push_thread.join().unwrap();
        pull_thread.join().unwrap();
    }

    #[test]
    fn should_create_a_pipeline_mt1() {
        test_multithread_pipeline("ipc:///tmp/should_create_a_pipeline_mt1.ipc")
    }

    #[test]
    fn should_create_a_pipeline_mt2() {
        test_multithread_pipeline("ipc:///tmp/should_create_a_pipeline_mt2.ipc")
    }

    #[test]
    fn constants_should_match_return_of_symbol_func() {
        unsafe {
            let mut index: c_int = 0;
            loop {
                let mut c_value: c_int = -1;
                let c_name_ptr = nn_symbol(index, &mut c_value);

                if c_name_ptr.is_null() {
                   break;
                }

                let c_name_str = CStr::from_ptr(c_name_ptr);
                let c_name_bytes = c_name_str.to_bytes();
                let c_name = str::from_utf8(c_name_bytes).unwrap();
                let mb_rust_value = get_constant_value_by_name(c_name);

                if mb_rust_value.is_some() {
                    let rust_value = mb_rust_value.unwrap();
                    if c_value != rust_value {
                        panic!("Constant {} value mismatch: {} != {}", c_name, c_value, rust_value);
                    }
                }

                index = index + 1;
            }
        }
    }

    fn get_constant_value_by_name(name: &str) -> Option<c_int> {
        match name {
            "AF_SP" => Some(AF_SP),
            "AF_SP_RAW" => Some(AF_SP_RAW),
            "NN_PROTO_PIPELINE" => Some(NN_PROTO_PIPELINE),
            "NN_PUSH" => Some(NN_PUSH),
            "NN_PULL" => Some(NN_PULL),
            "NN_PROTO_REQREP" => Some(NN_PROTO_REQREP),
            "NN_REQ" => Some(NN_REQ),
            "NN_REP" => Some(NN_REP),
            "NN_REQ_RESEND_IVL" => Some(NN_REQ_RESEND_IVL),
            "NN_PROTO_PAIR" => Some(NN_PROTO_PAIR),
            "NN_PAIR" => Some(NN_PAIR),
            "NN_PROTO_BUS" => Some(NN_PROTO_BUS),
            "NN_BUS" => Some(NN_BUS),
            "NN_PROTO_PUBSUB" => Some(NN_PROTO_PUBSUB),
            "NN_PUB" => Some(NN_PUB),
            "NN_SUB" => Some(NN_SUB),
            "NN_SUB_SUBSCRIBE" => Some(NN_SUB_SUBSCRIBE),
            "NN_SUB_UNSUBSCRIBE" => Some(NN_SUB_UNSUBSCRIBE),
            "NN_PROTO_SURVEY" => Some(NN_PROTO_SURVEY),
            "NN_SURVEYOR" => Some(NN_SURVEYOR),
            "NN_RESPONDENT" => Some(NN_RESPONDENT),
            "NN_SURVEYOR_DEADLINE" => Some(NN_SURVEYOR_DEADLINE),
            "NN_SOCKADDR_MAX" => Some(NN_SOCKADDR_MAX),
            "NN_SOL_SOCKET" => Some(NN_SOL_SOCKET),
            "NN_LINGER" => Some(NN_LINGER),
            "NN_SNDBUF" => Some(NN_SNDBUF),
            "NN_RCVBUF" => Some(NN_RCVBUF),
            "NN_SNDTIMEO" => Some(NN_SNDTIMEO),
            "NN_RCVTIMEO" => Some(NN_RCVTIMEO),
            "NN_RECONNECT_IVL" => Some(NN_RECONNECT_IVL),
            "NN_RECONNECT_IVL_MAX" => Some(NN_RECONNECT_IVL_MAX),
            "NN_SNDPRIO" => Some(NN_SNDPRIO),
            "NN_RCVPRIO" => Some(NN_RCVPRIO),
            "NN_SNDFD" => Some(NN_SNDFD),
            "NN_RCVFD" => Some(NN_RCVFD),
            "NN_DOMAIN" => Some(NN_DOMAIN),
            "NN_PROTOCOL" => Some(NN_PROTOCOL),
            "NN_IPV4ONLY" => Some(NN_IPV4ONLY),
            "NN_SOCKET_NAME" => Some(NN_SOCKET_NAME),
            "NN_RCVMAXSIZE" => Some(NN_RCVMAXSIZE),
            "NN_DONTWAIT" => Some(NN_DONTWAIT),
            "NN_INPROC" => Some(NN_INPROC),
            "NN_IPC" => Some(NN_IPC),
            "NN_TCP" => Some(NN_TCP),
            "NN_TCP_NODELAY" => Some(NN_TCP_NODELAY),
            "ETERM" => Some(ETERM),
            "EFSM" => Some(EFSM),
            "ENAMETOOLONG" => Some(ENAMETOOLONG),
            "ENODEV" => Some(ENODEV),
            "EINTR" => Some(EINTR),
            "NN_HAUSNUMERO" => Some(NN_HAUSNUMERO),
            "ENOTSUP " => Some(ENOTSUP ),
            "EPROTONOSUPPORT" => Some(EPROTONOSUPPORT),
            "ENOBUFS" => Some(ENOBUFS),
            "ENETDOWN" => Some(ENETDOWN),
            "EADDRINUSE" => Some(EADDRINUSE),
            "EADDRNOTAVAIL" => Some(EADDRNOTAVAIL),
            "ECONNREFUSED" => Some(ECONNREFUSED),
            "EINPROGRESS" => Some(EINPROGRESS),
            "ENOTSOCK" => Some(ENOTSOCK),
            "EAFNOSUPPORT" => Some(EAFNOSUPPORT),
            "EPROTO " => Some(EPROTO ),
            "EAGAIN" => Some(EAGAIN),
            "EBADF" => Some(EBADF),
            "EINVAL" => Some(EINVAL),
            "EMFILE" => Some(EMFILE),
            "EFAULT" => Some(EFAULT),
            "EACCESS" => Some(EACCESS),
            "ENETRESET" => Some(ENETRESET),
            "ENETUNREACH" => Some(ENETUNREACH),
            "EHOSTUNREACH" => Some(EHOSTUNREACH),
            "ENOTCONN" => Some(ENOTCONN),
            "EMSGSIZE" => Some(EMSGSIZE),
            "ETIMEDOUT" => Some(ETIMEDOUT),
            "ECONNABORTED" => Some(ECONNABORTED),
            "ECONNRESET" => Some(ECONNRESET),
            "ENOPROTOOPT" => Some(ENOPROTOOPT),
            "EISCONN" => Some(EISCONN),
            "ESOCKTNOSUPPORT" => Some(ESOCKTNOSUPPORT),
            _ => None
        }
    }
}

/*
/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
pub const true_: ::std::os::raw::c_uint = 1;
pub const false_: ::std::os::raw::c_uint = 0;
pub const __bool_true_false_are_defined: ::std::os::raw::c_uint = 1;
pub const _SAL_VERSION: ::std::os::raw::c_uint = 20;
pub const __SAL_H_VERSION: ::std::os::raw::c_uint = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: ::std::os::raw::c_uint = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: ::std::os::raw::c_uint = 0;
pub const _CRT_PACKING: ::std::os::raw::c_uint = 8;
pub const _HAS_EXCEPTIONS: ::std::os::raw::c_uint = 1;
pub const WCHAR_MIN: ::std::os::raw::c_uint = 0;
pub const WCHAR_MAX: ::std::os::raw::c_uint = 65535;
pub const WINT_MIN: ::std::os::raw::c_uint = 0;
pub const WINT_MAX: ::std::os::raw::c_uint = 65535;
pub const NNG_MAJOR_VERSION: ::std::os::raw::c_uint = 1;
pub const NNG_MINOR_VERSION: ::std::os::raw::c_uint = 0;
pub const NNG_PATCH_VERSION: ::std::os::raw::c_uint = 0;
pub const NNG_RELEASE_SUFFIX: &'static [u8; 1usize] = b"\x00";
pub const NNG_MAXADDRLEN: ::std::os::raw::c_uint = 128;
pub const NNG_DURATION_INFINITE: ::std::os::raw::c_int = -1;
pub const NNG_DURATION_DEFAULT: ::std::os::raw::c_int = -2;
pub const NNG_DURATION_ZERO: ::std::os::raw::c_uint = 0;
pub const NNG_OPT_SOCKNAME: &'static [u8; 12usize] = b"socket-name\x00";
pub const NNG_OPT_RAW: &'static [u8; 4usize] = b"raw\x00";
pub const NNG_OPT_PROTO: &'static [u8; 9usize] = b"protocol\x00";
pub const NNG_OPT_PROTONAME: &'static [u8; 14usize] = b"protocol-name\x00";
pub const NNG_OPT_PEER: &'static [u8; 5usize] = b"peer\x00";
pub const NNG_OPT_PEERNAME: &'static [u8; 10usize] = b"peer-name\x00";
pub const NNG_OPT_RECVBUF: &'static [u8; 12usize] = b"recv-buffer\x00";
pub const NNG_OPT_SENDBUF: &'static [u8; 12usize] = b"send-buffer\x00";
pub const NNG_OPT_RECVFD: &'static [u8; 8usize] = b"recv-fd\x00";
pub const NNG_OPT_SENDFD: &'static [u8; 8usize] = b"send-fd\x00";
pub const NNG_OPT_RECVTIMEO: &'static [u8; 13usize] = b"recv-timeout\x00";
pub const NNG_OPT_SENDTIMEO: &'static [u8; 13usize] = b"send-timeout\x00";
pub const NNG_OPT_LOCADDR: &'static [u8; 14usize] = b"local-address\x00";
pub const NNG_OPT_REMADDR: &'static [u8; 15usize] = b"remote-address\x00";
pub const NNG_OPT_URL: &'static [u8; 4usize] = b"url\x00";
pub const NNG_OPT_MAXTTL: &'static [u8; 8usize] = b"ttl-max\x00";
pub const NNG_OPT_RECVMAXSZ: &'static [u8; 14usize] = b"recv-size-max\x00";
pub const NNG_OPT_RECONNMINT: &'static [u8; 19usize] =
    b"reconnect-time-min\x00";
pub const NNG_OPT_RECONNMAXT: &'static [u8; 19usize] =
    b"reconnect-time-max\x00";
pub const NNG_OPT_TLS_CONFIG: &'static [u8; 11usize] = b"tls-config\x00";
pub const NNG_OPT_TLS_AUTH_MODE: &'static [u8; 13usize] = b"tls-authmode\x00";
pub const NNG_OPT_TLS_CERT_KEY_FILE: &'static [u8; 18usize] =
    b"tls-cert-key-file\x00";
pub const NNG_OPT_TLS_CA_FILE: &'static [u8; 12usize] = b"tls-ca-file\x00";
pub const NNG_OPT_TLS_SERVER_NAME: &'static [u8; 16usize] =
    b"tls-server-name\x00";
pub const NNG_OPT_TLS_VERIFIED: &'static [u8; 13usize] = b"tls-verified\x00";
pub const NNG_OPT_TCP_NODELAY: &'static [u8; 12usize] = b"tcp-nodelay\x00";
pub const NNG_OPT_TCP_KEEPALIVE: &'static [u8; 14usize] =
    b"tcp-keepalive\x00";
pub const _ARGMAX: ::std::os::raw::c_uint = 100;
pub const _CRT_INT_MAX: ::std::os::raw::c_uint = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: ::std::os::raw::c_uint = 1;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: ::std::os::raw::c_uint
          =
    0;
pub const _CRT_BUILD_DESKTOP_APP: ::std::os::raw::c_uint = 1;
pub const __STDC_SECURE_LIB__: ::std::os::raw::c_uint = 200411;
pub const __GOT_SECURE_LIB__: ::std::os::raw::c_uint = 200411;
pub const __STDC_WANT_SECURE_LIB__: ::std::os::raw::c_uint = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: ::std::os::raw::c_uint = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: ::std::os::raw::c_uint = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT:
          ::std::os::raw::c_uint =
    0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: ::std::os::raw::c_uint = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY:
          ::std::os::raw::c_uint =
    0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: ::std::os::raw::c_uint
          =
    0;
pub const EPERM: ::std::os::raw::c_uint = 1;
pub const ENOENT: ::std::os::raw::c_uint = 2;
pub const ESRCH: ::std::os::raw::c_uint = 3;
pub const EINTR: ::std::os::raw::c_uint = 4;
pub const EIO: ::std::os::raw::c_uint = 5;
pub const ENXIO: ::std::os::raw::c_uint = 6;
pub const E2BIG: ::std::os::raw::c_uint = 7;
pub const ENOEXEC: ::std::os::raw::c_uint = 8;
pub const EBADF: ::std::os::raw::c_uint = 9;
pub const ECHILD: ::std::os::raw::c_uint = 10;
pub const EAGAIN: ::std::os::raw::c_uint = 11;
pub const ENOMEM: ::std::os::raw::c_uint = 12;
pub const EACCES: ::std::os::raw::c_uint = 13;
pub const EFAULT: ::std::os::raw::c_uint = 14;
pub const EBUSY: ::std::os::raw::c_uint = 16;
pub const EEXIST: ::std::os::raw::c_uint = 17;
pub const EXDEV: ::std::os::raw::c_uint = 18;
pub const ENODEV: ::std::os::raw::c_uint = 19;
pub const ENOTDIR: ::std::os::raw::c_uint = 20;
pub const EISDIR: ::std::os::raw::c_uint = 21;
pub const ENFILE: ::std::os::raw::c_uint = 23;
pub const EMFILE: ::std::os::raw::c_uint = 24;
pub const ENOTTY: ::std::os::raw::c_uint = 25;
pub const EFBIG: ::std::os::raw::c_uint = 27;
pub const ENOSPC: ::std::os::raw::c_uint = 28;
pub const ESPIPE: ::std::os::raw::c_uint = 29;
pub const EROFS: ::std::os::raw::c_uint = 30;
pub const EMLINK: ::std::os::raw::c_uint = 31;
pub const EPIPE: ::std::os::raw::c_uint = 32;
pub const EDOM: ::std::os::raw::c_uint = 33;
pub const EDEADLK: ::std::os::raw::c_uint = 36;
pub const ENAMETOOLONG: ::std::os::raw::c_uint = 38;
pub const ENOLCK: ::std::os::raw::c_uint = 39;
pub const ENOSYS: ::std::os::raw::c_uint = 40;
pub const ENOTEMPTY: ::std::os::raw::c_uint = 41;
pub const EINVAL: ::std::os::raw::c_uint = 22;
pub const ERANGE: ::std::os::raw::c_uint = 34;
pub const EILSEQ: ::std::os::raw::c_uint = 42;
pub const STRUNCATE: ::std::os::raw::c_uint = 80;
pub const EDEADLOCK: ::std::os::raw::c_uint = 36;
pub const EADDRINUSE: ::std::os::raw::c_uint = 100;
pub const EADDRNOTAVAIL: ::std::os::raw::c_uint = 101;
pub const EAFNOSUPPORT: ::std::os::raw::c_uint = 102;
pub const EALREADY: ::std::os::raw::c_uint = 103;
pub const EBADMSG: ::std::os::raw::c_uint = 104;
pub const ECANCELED: ::std::os::raw::c_uint = 105;
pub const ECONNABORTED: ::std::os::raw::c_uint = 106;
pub const ECONNREFUSED: ::std::os::raw::c_uint = 107;
pub const ECONNRESET: ::std::os::raw::c_uint = 108;
pub const EDESTADDRREQ: ::std::os::raw::c_uint = 109;
pub const EHOSTUNREACH: ::std::os::raw::c_uint = 110;
pub const EIDRM: ::std::os::raw::c_uint = 111;
pub const EINPROGRESS: ::std::os::raw::c_uint = 112;
pub const EISCONN: ::std::os::raw::c_uint = 113;
pub const ELOOP: ::std::os::raw::c_uint = 114;
pub const EMSGSIZE: ::std::os::raw::c_uint = 115;
pub const ENETDOWN: ::std::os::raw::c_uint = 116;
pub const ENETRESET: ::std::os::raw::c_uint = 117;
pub const ENETUNREACH: ::std::os::raw::c_uint = 118;
pub const ENOBUFS: ::std::os::raw::c_uint = 119;
pub const ENODATA: ::std::os::raw::c_uint = 120;
pub const ENOLINK: ::std::os::raw::c_uint = 121;
pub const ENOMSG: ::std::os::raw::c_uint = 122;
pub const ENOPROTOOPT: ::std::os::raw::c_uint = 123;
pub const ENOSR: ::std::os::raw::c_uint = 124;
pub const ENOSTR: ::std::os::raw::c_uint = 125;
pub const ENOTCONN: ::std::os::raw::c_uint = 126;
pub const ENOTRECOVERABLE: ::std::os::raw::c_uint = 127;
pub const ENOTSOCK: ::std::os::raw::c_uint = 128;
pub const ENOTSUP: ::std::os::raw::c_uint = 129;
pub const EOPNOTSUPP: ::std::os::raw::c_uint = 130;
pub const EOTHER: ::std::os::raw::c_uint = 131;
pub const EOVERFLOW: ::std::os::raw::c_uint = 132;
pub const EOWNERDEAD: ::std::os::raw::c_uint = 133;
pub const EPROTO: ::std::os::raw::c_uint = 134;
pub const EPROTONOSUPPORT: ::std::os::raw::c_uint = 135;
pub const EPROTOTYPE: ::std::os::raw::c_uint = 136;
pub const ETIME: ::std::os::raw::c_uint = 137;
pub const ETIMEDOUT: ::std::os::raw::c_uint = 138;
pub const ETXTBSY: ::std::os::raw::c_uint = 139;
pub const EWOULDBLOCK: ::std::os::raw::c_uint = 140;
pub const AF_SP: ::std::os::raw::c_uint = 1;
pub const AF_SP_RAW: ::std::os::raw::c_uint = 2;
pub const NN_SOCKADDR_MAX: ::std::os::raw::c_uint = 128;
pub const NN_SOL_SOCKET: ::std::os::raw::c_uint = 0;
pub const NN_DONTWAIT: ::std::os::raw::c_uint = 1;
pub const PROTO_SP: ::std::os::raw::c_uint = 1;
pub const SP_HDR: ::std::os::raw::c_uint = 1;
pub const NN_ERRBASE: ::std::os::raw::c_uint = 268435456;
pub const ESOCKNOSPPORT: ::std::os::raw::c_uint = 268435484;
pub const ETERM: ::std::os::raw::c_uint = 268435485;
pub const EFSM: ::std::os::raw::c_uint = 268435486;
pub const NN_LINGER: ::std::os::raw::c_uint = 1;
pub const NN_SNDBUF: ::std::os::raw::c_uint = 2;
pub const NN_RCVBUF: ::std::os::raw::c_uint = 3;
pub const NN_SNDTIMEO: ::std::os::raw::c_uint = 4;
pub const NN_RCVTIMEO: ::std::os::raw::c_uint = 5;
pub const NN_RECONNECT_IVL: ::std::os::raw::c_uint = 6;
pub const NN_RECONNECT_IVL_MAX: ::std::os::raw::c_uint = 7;
pub const NN_SNDPRIO: ::std::os::raw::c_uint = 8;
pub const NN_RCVPRIO: ::std::os::raw::c_uint = 9;
pub const NN_SNDFD: ::std::os::raw::c_uint = 10;
pub const NN_RCVFD: ::std::os::raw::c_uint = 11;
pub const NN_DOMAIN: ::std::os::raw::c_uint = 12;
pub const NN_PROTOCOL: ::std::os::raw::c_uint = 13;
pub const NN_IPV4ONLY: ::std::os::raw::c_uint = 14;
pub const NN_SOCKET_NAME: ::std::os::raw::c_uint = 15;
pub const NN_RCVMAXSIZE: ::std::os::raw::c_uint = 16;
pub const NN_MAXTTL: ::std::os::raw::c_uint = 17;
pub const NN_POLLIN: ::std::os::raw::c_uint = 1;
pub const NN_POLLOUT: ::std::os::raw::c_uint = 2;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
pub type __vcrt_bool = bool;
extern "C" {
    //#[link_name = "___security_init_cookie"]
    pub fn __security_init_cookie();
}
extern "fastcall" {
    //#[link_name = "@__security_check_cookie@4"]
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    //#[link_name = "___report_gsfailure"]
    pub fn __report_gsfailure();
}
extern "C" {
    //#[link_name = "___security_cookie"]
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_char;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_char;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_ctx_s {
    pub id: u32,
}
#[test]
fn bindgen_test_layout_nng_ctx_s() {
    assert_eq!(::std::mem::size_of::<nng_ctx_s>() , 4usize);
    assert_eq!(::std::mem::align_of::<nng_ctx_s>() , 4usize);
}
impl Clone for nng_ctx_s {
    fn clone(&self) -> Self { *self }
}
pub type nng_ctx = nng_ctx_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_dialer_s {
    pub id: u32,
}
#[test]
fn bindgen_test_layout_nng_dialer_s() {
    assert_eq!(::std::mem::size_of::<nng_dialer_s>() , 4usize);
    assert_eq!(::std::mem::align_of::<nng_dialer_s>() , 4usize);
}
impl Clone for nng_dialer_s {
    fn clone(&self) -> Self { *self }
}
pub type nng_dialer = nng_dialer_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_listener_s {
    pub id: u32,
}
#[test]
fn bindgen_test_layout_nng_listener_s() {
    assert_eq!(::std::mem::size_of::<nng_listener_s>() , 4usize);
    assert_eq!(::std::mem::align_of::<nng_listener_s>() , 4usize);
}
impl Clone for nng_listener_s {
    fn clone(&self) -> Self { *self }
}
pub type nng_listener = nng_listener_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_pipe_s {
    pub id: u32,
}
#[test]
fn bindgen_test_layout_nng_pipe_s() {
    assert_eq!(::std::mem::size_of::<nng_pipe_s>() , 4usize);
    assert_eq!(::std::mem::align_of::<nng_pipe_s>() , 4usize);
}
impl Clone for nng_pipe_s {
    fn clone(&self) -> Self { *self }
}
pub type nng_pipe = nng_pipe_s;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_socket_s {
    pub id: u32,
}
#[test]
fn bindgen_test_layout_nng_socket_s() {
    assert_eq!(::std::mem::size_of::<nng_socket_s>() , 4usize);
    assert_eq!(::std::mem::align_of::<nng_socket_s>() , 4usize);
}
impl Clone for nng_socket_s {
    fn clone(&self) -> Self { *self }
}
pub type nng_socket = nng_socket_s;
pub type nng_duration = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_msg([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_snapshot([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_stat([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nng_aio([u8; 0]);
#[repr(C)]
pub struct nng_sockaddr_inproc {
    pub sa_family: u16,
    pub sa_name: [::std::os::raw::c_char; 128usize],
}
#[test]
fn bindgen_test_layout_nng_sockaddr_inproc() {
    assert_eq!(::std::mem::size_of::<nng_sockaddr_inproc>() , 130usize);
    assert_eq!(::std::mem::align_of::<nng_sockaddr_inproc>() , 2usize);
}
#[repr(C)]
pub struct nng_sockaddr_path {
    pub sa_family: u16,
    pub sa_path: [::std::os::raw::c_char; 128usize],
}
#[test]
fn bindgen_test_layout_nng_sockaddr_path() {
    assert_eq!(::std::mem::size_of::<nng_sockaddr_path>() , 130usize);
    assert_eq!(::std::mem::align_of::<nng_sockaddr_path>() , 2usize);
}
pub type nng_sockaddr_ipc = nng_sockaddr_path;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_sockaddr_in6 {
    pub sa_family: u16,
    pub sa_port: u16,
    pub sa_addr: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_nng_sockaddr_in6() {
    assert_eq!(::std::mem::size_of::<nng_sockaddr_in6>() , 20usize);
    assert_eq!(::std::mem::align_of::<nng_sockaddr_in6>() , 2usize);
}
impl Clone for nng_sockaddr_in6 {
    fn clone(&self) -> Self { *self }
}
pub type nng_sockaddr_udp6 = nng_sockaddr_in6;
pub type nng_sockaddr_tcp6 = nng_sockaddr_in6;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_sockaddr_in {
    pub sa_family: u16,
    pub sa_port: u16,
    pub sa_addr: u32,
}
#[test]
fn bindgen_test_layout_nng_sockaddr_in() {
    assert_eq!(::std::mem::size_of::<nng_sockaddr_in>() , 8usize);
    assert_eq!(::std::mem::align_of::<nng_sockaddr_in>() , 4usize);
}
impl Clone for nng_sockaddr_in {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_sockaddr_zt {
    pub sa_family: u16,
    pub sa_nwid: u64,
    pub sa_nodeid: u64,
    pub sa_port: u32,
}
#[test]
fn bindgen_test_layout_nng_sockaddr_zt() {
    assert_eq!(::std::mem::size_of::<nng_sockaddr_zt>() , 32usize);
    assert_eq!(::std::mem::align_of::<nng_sockaddr_zt>() , 8usize);
}
impl Clone for nng_sockaddr_zt {
    fn clone(&self) -> Self { *self }
}
pub type nng_sockaddr_udp = nng_sockaddr_in;
pub type nng_sockaddr_tcp = nng_sockaddr_in;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_sockaddr {
    pub s_family: __BindgenUnionField<u16>,
    pub s_ipc: __BindgenUnionField<nng_sockaddr_ipc>,
    pub s_inproc: __BindgenUnionField<nng_sockaddr_inproc>,
    pub s_in6: __BindgenUnionField<nng_sockaddr_in6>,
    pub s_in: __BindgenUnionField<nng_sockaddr_in>,
    pub s_zt: __BindgenUnionField<nng_sockaddr_zt>,
    pub bindgen_union_field: [u64; 17usize],
}
#[test]
fn bindgen_test_layout_nng_sockaddr() {
    assert_eq!(::std::mem::size_of::<nng_sockaddr>() , 136usize);
    assert_eq!(::std::mem::align_of::<nng_sockaddr>() , 8usize);
}
impl Clone for nng_sockaddr {
    fn clone(&self) -> Self { *self }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_sockaddr_family {
    NNG_AF_UNSPEC = 0,
    NNG_AF_INPROC = 1,
    NNG_AF_IPC = 2,
    NNG_AF_INET = 3,
    NNG_AF_INET6 = 4,
    NNG_AF_ZT = 5,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_iov {
    pub iov_buf: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}
#[test]
fn bindgen_test_layout_nng_iov() {
    assert_eq!(::std::mem::size_of::<nng_iov>() , 8usize);
    assert_eq!(::std::mem::align_of::<nng_iov>() , 4usize);
}
impl Clone for nng_iov {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    //#[link_name = "_nng_fini"]
    pub fn nng_fini();
}
extern "C" {
    //#[link_name = "_nng_close"]
    pub fn nng_close(arg1: nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_socket_id"]
    pub fn nng_socket_id(arg1: nng_socket) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_closeall"]
    pub fn nng_closeall();
}
extern "C" {
    //#[link_name = "_nng_setopt"]
    pub fn nng_setopt(arg1: nng_socket, arg2: *const ::std::os::raw::c_char,
                      arg3: *const ::std::os::raw::c_void, arg4: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_bool"]
    pub fn nng_setopt_bool(arg1: nng_socket,
                           arg2: *const ::std::os::raw::c_char, arg3: bool)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_int"]
    pub fn nng_setopt_int(arg1: nng_socket,
                          arg2: *const ::std::os::raw::c_char,
                          arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_ms"]
    pub fn nng_setopt_ms(arg1: nng_socket,
                         arg2: *const ::std::os::raw::c_char,
                         arg3: nng_duration) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_size"]
    pub fn nng_setopt_size(arg1: nng_socket,
                           arg2: *const ::std::os::raw::c_char, arg3: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_uint64"]
    pub fn nng_setopt_uint64(arg1: nng_socket,
                             arg2: *const ::std::os::raw::c_char, arg3: u64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_string"]
    pub fn nng_setopt_string(arg1: nng_socket,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_setopt_ptr"]
    pub fn nng_setopt_ptr(arg1: nng_socket,
                          arg2: *const ::std::os::raw::c_char,
                          arg3: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt"]
    pub fn nng_getopt(arg1: nng_socket, arg2: *const ::std::os::raw::c_char,
                      arg3: *mut ::std::os::raw::c_void, arg4: *mut usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_bool"]
    pub fn nng_getopt_bool(arg1: nng_socket,
                           arg2: *const ::std::os::raw::c_char,
                           arg3: *mut bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_int"]
    pub fn nng_getopt_int(arg1: nng_socket,
                          arg2: *const ::std::os::raw::c_char,
                          arg3: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_ms"]
    pub fn nng_getopt_ms(arg1: nng_socket,
                         arg2: *const ::std::os::raw::c_char,
                         arg3: *mut nng_duration) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_size"]
    pub fn nng_getopt_size(arg1: nng_socket,
                           arg2: *const ::std::os::raw::c_char,
                           arg3: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_uint64"]
    pub fn nng_getopt_uint64(arg1: nng_socket,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_ptr"]
    pub fn nng_getopt_ptr(arg1: nng_socket,
                          arg2: *const ::std::os::raw::c_char,
                          arg3: *mut *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
pub const NNG_PIPE_EV_ADD_PRE: _bindgen_ty_1 =
    _bindgen_ty_1::NNG_PIPE_EV_ADD_PRE;
pub const NNG_PIPE_EV_ADD_POST: _bindgen_ty_1 =
    _bindgen_ty_1::NNG_PIPE_EV_ADD_POST;
pub const NNG_PIPE_EV_REM_POST: _bindgen_ty_1 =
    _bindgen_ty_1::NNG_PIPE_EV_REM_POST;
pub const NNG_PIPE_EV_NUM: _bindgen_ty_1 = _bindgen_ty_1::NNG_PIPE_EV_NUM;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 {
    NNG_PIPE_EV_ADD_PRE = 0,
    NNG_PIPE_EV_ADD_POST = 1,
    NNG_PIPE_EV_REM_POST = 2,
    NNG_PIPE_EV_NUM = 3,
}
pub use self::_bindgen_ty_1 as nng_pipe_ev;
pub type nng_pipe_cb =
    ::std::option::Option<unsafe extern "C" fn(arg1: nng_pipe,
                                               arg2: ::std::os::raw::c_int,
                                               arg3:
                                                   *mut ::std::os::raw::c_void)>;
extern "C" {
    //#[link_name = "_nng_pipe_notify"]
    pub fn nng_pipe_notify(arg1: nng_socket, arg2: ::std::os::raw::c_int,
                           arg3: nng_pipe_cb,
                           arg4: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_getopt_string"]
    pub fn nng_getopt_string(arg1: nng_socket,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listen"]
    pub fn nng_listen(arg1: nng_socket, arg2: *const ::std::os::raw::c_char,
                      arg3: *mut nng_listener, arg4: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dial"]
    pub fn nng_dial(arg1: nng_socket, arg2: *const ::std::os::raw::c_char,
                    arg3: *mut nng_dialer, arg4: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_create"]
    pub fn nng_dialer_create(arg1: *mut nng_dialer, arg2: nng_socket,
                             arg3: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_create"]
    pub fn nng_listener_create(arg1: *mut nng_listener, arg2: nng_socket,
                               arg3: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_start"]
    pub fn nng_dialer_start(arg1: nng_dialer, arg2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_start"]
    pub fn nng_listener_start(arg1: nng_listener, arg2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_close"]
    pub fn nng_dialer_close(arg1: nng_dialer) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_close"]
    pub fn nng_listener_close(arg1: nng_listener) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_id"]
    pub fn nng_dialer_id(arg1: nng_dialer) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_id"]
    pub fn nng_listener_id(arg1: nng_listener) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt"]
    pub fn nng_dialer_setopt(arg1: nng_dialer,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *const ::std::os::raw::c_void, arg4: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_bool"]
    pub fn nng_dialer_setopt_bool(arg1: nng_dialer,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_int"]
    pub fn nng_dialer_setopt_int(arg1: nng_dialer,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_ms"]
    pub fn nng_dialer_setopt_ms(arg1: nng_dialer,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: nng_duration) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_size"]
    pub fn nng_dialer_setopt_size(arg1: nng_dialer,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_uint64"]
    pub fn nng_dialer_setopt_uint64(arg1: nng_dialer,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_ptr"]
    pub fn nng_dialer_setopt_ptr(arg1: nng_dialer,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_setopt_string"]
    pub fn nng_dialer_setopt_string(arg1: nng_dialer,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt"]
    pub fn nng_dialer_getopt(arg1: nng_dialer,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *mut ::std::os::raw::c_void,
                             arg4: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_bool"]
    pub fn nng_dialer_getopt_bool(arg1: nng_dialer,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_int"]
    pub fn nng_dialer_getopt_int(arg1: nng_dialer,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_ms"]
    pub fn nng_dialer_getopt_ms(arg1: nng_dialer,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut nng_duration)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_size"]
    pub fn nng_dialer_getopt_size(arg1: nng_dialer,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_sockaddr"]
    pub fn nng_dialer_getopt_sockaddr(arg1: nng_dialer,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut nng_sockaddr)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_uint64"]
    pub fn nng_dialer_getopt_uint64(arg1: nng_dialer,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_ptr"]
    pub fn nng_dialer_getopt_ptr(arg1: nng_dialer,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: *mut *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_dialer_getopt_string"]
    pub fn nng_dialer_getopt_string(arg1: nng_dialer,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt"]
    pub fn nng_listener_setopt(arg1: nng_listener,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *const ::std::os::raw::c_void,
                               arg4: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_bool"]
    pub fn nng_listener_setopt_bool(arg1: nng_listener,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_int"]
    pub fn nng_listener_setopt_int(arg1: nng_listener,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_ms"]
    pub fn nng_listener_setopt_ms(arg1: nng_listener,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: nng_duration)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_size"]
    pub fn nng_listener_setopt_size(arg1: nng_listener,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_uint64"]
    pub fn nng_listener_setopt_uint64(arg1: nng_listener,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_ptr"]
    pub fn nng_listener_setopt_ptr(arg1: nng_listener,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_setopt_string"]
    pub fn nng_listener_setopt_string(arg1: nng_listener,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt"]
    pub fn nng_listener_getopt(arg1: nng_listener,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut ::std::os::raw::c_void,
                               arg4: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_bool"]
    pub fn nng_listener_getopt_bool(arg1: nng_listener,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_int"]
    pub fn nng_listener_getopt_int(arg1: nng_listener,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_ms"]
    pub fn nng_listener_getopt_ms(arg1: nng_listener,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut nng_duration)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_size"]
    pub fn nng_listener_getopt_size(arg1: nng_listener,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_sockaddr"]
    pub fn nng_listener_getopt_sockaddr(arg1: nng_listener,
                                        arg2: *const ::std::os::raw::c_char,
                                        arg3: *mut nng_sockaddr)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_uint64"]
    pub fn nng_listener_getopt_uint64(arg1: nng_listener,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut u64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_ptr"]
    pub fn nng_listener_getopt_ptr(arg1: nng_listener,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_listener_getopt_string"]
    pub fn nng_listener_getopt_string(arg1: nng_listener,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_strerror"]
    pub fn nng_strerror(arg1: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    //#[link_name = "_nng_send"]
    pub fn nng_send(arg1: nng_socket, arg2: *mut ::std::os::raw::c_void,
                    arg3: usize, arg4: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_recv"]
    pub fn nng_recv(arg1: nng_socket, arg2: *mut ::std::os::raw::c_void,
                    arg3: *mut usize, arg4: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_sendmsg"]
    pub fn nng_sendmsg(arg1: nng_socket, arg2: *mut nng_msg,
                       arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_recvmsg"]
    pub fn nng_recvmsg(arg1: nng_socket, arg2: *mut *mut nng_msg,
                       arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_send_aio"]
    pub fn nng_send_aio(arg1: nng_socket, arg2: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_recv_aio"]
    pub fn nng_recv_aio(arg1: nng_socket, arg2: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_ctx_open"]
    pub fn nng_ctx_open(arg1: *mut nng_ctx, arg2: nng_socket)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_close"]
    pub fn nng_ctx_close(arg1: nng_ctx) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_id"]
    pub fn nng_ctx_id(arg1: nng_ctx) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_recv"]
    pub fn nng_ctx_recv(arg1: nng_ctx, arg2: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_ctx_send"]
    pub fn nng_ctx_send(arg1: nng_ctx, arg2: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_ctx_getopt"]
    pub fn nng_ctx_getopt(arg1: nng_ctx, arg2: *const ::std::os::raw::c_char,
                          arg3: *mut ::std::os::raw::c_void, arg4: *mut usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_getopt_bool"]
    pub fn nng_ctx_getopt_bool(arg1: nng_ctx,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_getopt_int"]
    pub fn nng_ctx_getopt_int(arg1: nng_ctx,
                              arg2: *const ::std::os::raw::c_char,
                              arg3: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_getopt_ms"]
    pub fn nng_ctx_getopt_ms(arg1: nng_ctx,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *mut nng_duration)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_getopt_size"]
    pub fn nng_ctx_getopt_size(arg1: nng_ctx,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_setopt"]
    pub fn nng_ctx_setopt(arg1: nng_ctx, arg2: *const ::std::os::raw::c_char,
                          arg3: *const ::std::os::raw::c_void, arg4: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_setopt_bool"]
    pub fn nng_ctx_setopt_bool(arg1: nng_ctx,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_setopt_int"]
    pub fn nng_ctx_setopt_int(arg1: nng_ctx,
                              arg2: *const ::std::os::raw::c_char,
                              arg3: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_setopt_ms"]
    pub fn nng_ctx_setopt_ms(arg1: nng_ctx,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: nng_duration) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_ctx_setopt_size"]
    pub fn nng_ctx_setopt_size(arg1: nng_ctx,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_alloc"]
    pub fn nng_alloc(arg1: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nng_free"]
    pub fn nng_free(arg1: *mut ::std::os::raw::c_void, arg2: usize);
}
extern "C" {
    //#[link_name = "_nng_strdup"]
    pub fn nng_strdup(arg1: *const ::std::os::raw::c_char)
     -> *mut ::std::os::raw::c_char;
}
extern "C" {
    //#[link_name = "_nng_strfree"]
    pub fn nng_strfree(arg1: *mut ::std::os::raw::c_char);
}
extern "C" {
    //#[link_name = "_nng_aio_alloc"]
    pub fn nng_aio_alloc(arg1: *mut *mut nng_aio,
                         arg2:
                             ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                            *mut ::std::os::raw::c_void)>,
                         arg3: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_aio_free"]
    pub fn nng_aio_free(arg1: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_aio_stop"]
    pub fn nng_aio_stop(arg1: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_aio_result"]
    pub fn nng_aio_result(arg1: *mut nng_aio) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_aio_count"]
    pub fn nng_aio_count(arg1: *mut nng_aio) -> usize;
}
extern "C" {
    //#[link_name = "_nng_aio_cancel"]
    pub fn nng_aio_cancel(arg1: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_aio_abort"]
    pub fn nng_aio_abort(arg1: *mut nng_aio, arg2: ::std::os::raw::c_int);
}
extern "C" {
    //#[link_name = "_nng_aio_wait"]
    pub fn nng_aio_wait(arg1: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_aio_set_msg"]
    pub fn nng_aio_set_msg(arg1: *mut nng_aio, arg2: *mut nng_msg);
}
extern "C" {
    //#[link_name = "_nng_aio_get_msg"]
    pub fn nng_aio_get_msg(arg1: *mut nng_aio) -> *mut nng_msg;
}
extern "C" {
    //#[link_name = "_nng_aio_set_input"]
    pub fn nng_aio_set_input(arg1: *mut nng_aio, arg2: ::std::os::raw::c_uint,
                             arg3: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_aio_get_input"]
    pub fn nng_aio_get_input(arg1: *mut nng_aio, arg2: ::std::os::raw::c_uint)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nng_aio_set_output"]
    pub fn nng_aio_set_output(arg1: *mut nng_aio,
                              arg2: ::std::os::raw::c_uint,
                              arg3: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_aio_get_output"]
    pub fn nng_aio_get_output(arg1: *mut nng_aio,
                              arg2: ::std::os::raw::c_uint)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nng_aio_set_timeout"]
    pub fn nng_aio_set_timeout(arg1: *mut nng_aio, arg2: nng_duration);
}
extern "C" {
    //#[link_name = "_nng_aio_set_iov"]
    pub fn nng_aio_set_iov(arg1: *mut nng_aio, arg2: ::std::os::raw::c_uint,
                           arg3: *const nng_iov) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_aio_finish"]
    pub fn nng_aio_finish(arg1: *mut nng_aio, arg2: ::std::os::raw::c_int);
}
extern "C" {
    //#[link_name = "_nng_sleep_aio"]
    pub fn nng_sleep_aio(arg1: nng_duration, arg2: *mut nng_aio);
}
extern "C" {
    //#[link_name = "_nng_msg_alloc"]
    pub fn nng_msg_alloc(arg1: *mut *mut nng_msg, arg2: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_free"]
    pub fn nng_msg_free(arg1: *mut nng_msg);
}
extern "C" {
    //#[link_name = "_nng_msg_realloc"]
    pub fn nng_msg_realloc(arg1: *mut nng_msg, arg2: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header"]
    pub fn nng_msg_header(arg1: *mut nng_msg) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nng_msg_header_len"]
    pub fn nng_msg_header_len(arg1: *const nng_msg) -> usize;
}
extern "C" {
    //#[link_name = "_nng_msg_body"]
    pub fn nng_msg_body(arg1: *mut nng_msg) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nng_msg_len"]
    pub fn nng_msg_len(arg1: *const nng_msg) -> usize;
}
extern "C" {
    //#[link_name = "_nng_msg_append"]
    pub fn nng_msg_append(arg1: *mut nng_msg,
                          arg2: *const ::std::os::raw::c_void, arg3: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_insert"]
    pub fn nng_msg_insert(arg1: *mut nng_msg,
                          arg2: *const ::std::os::raw::c_void, arg3: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_trim"]
    pub fn nng_msg_trim(arg1: *mut nng_msg, arg2: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_chop"]
    pub fn nng_msg_chop(arg1: *mut nng_msg, arg2: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_append"]
    pub fn nng_msg_header_append(arg1: *mut nng_msg,
                                 arg2: *const ::std::os::raw::c_void,
                                 arg3: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_insert"]
    pub fn nng_msg_header_insert(arg1: *mut nng_msg,
                                 arg2: *const ::std::os::raw::c_void,
                                 arg3: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_trim"]
    pub fn nng_msg_header_trim(arg1: *mut nng_msg, arg2: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_chop"]
    pub fn nng_msg_header_chop(arg1: *mut nng_msg, arg2: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_append_u32"]
    pub fn nng_msg_header_append_u32(arg1: *mut nng_msg, arg2: u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_insert_u32"]
    pub fn nng_msg_header_insert_u32(arg1: *mut nng_msg, arg2: u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_chop_u32"]
    pub fn nng_msg_header_chop_u32(arg1: *mut nng_msg, arg2: *mut u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_header_trim_u32"]
    pub fn nng_msg_header_trim_u32(arg1: *mut nng_msg, arg2: *mut u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_append_u32"]
    pub fn nng_msg_append_u32(arg1: *mut nng_msg, arg2: u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_insert_u32"]
    pub fn nng_msg_insert_u32(arg1: *mut nng_msg, arg2: u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_chop_u32"]
    pub fn nng_msg_chop_u32(arg1: *mut nng_msg, arg2: *mut u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_trim_u32"]
    pub fn nng_msg_trim_u32(arg1: *mut nng_msg, arg2: *mut u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_dup"]
    pub fn nng_msg_dup(arg1: *mut *mut nng_msg, arg2: *const nng_msg)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_msg_clear"]
    pub fn nng_msg_clear(arg1: *mut nng_msg);
}
extern "C" {
    //#[link_name = "_nng_msg_header_clear"]
    pub fn nng_msg_header_clear(arg1: *mut nng_msg);
}
extern "C" {
    //#[link_name = "_nng_msg_set_pipe"]
    pub fn nng_msg_set_pipe(arg1: *mut nng_msg, arg2: nng_pipe);
}
extern "C" {
    //#[link_name = "_nng_msg_get_pipe"]
    pub fn nng_msg_get_pipe(arg1: *const nng_msg) -> nng_pipe;
}
extern "C" {
    //#[link_name = "_nng_msg_getopt"]
    pub fn nng_msg_getopt(arg1: *mut nng_msg, arg2: ::std::os::raw::c_int,
                          arg3: *mut ::std::os::raw::c_void, arg4: *mut usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt"]
    pub fn nng_pipe_getopt(arg1: nng_pipe,
                           arg2: *const ::std::os::raw::c_char,
                           arg3: *mut ::std::os::raw::c_void,
                           arg4: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_bool"]
    pub fn nng_pipe_getopt_bool(arg1: nng_pipe,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut bool) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_int"]
    pub fn nng_pipe_getopt_int(arg1: nng_pipe,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_ms"]
    pub fn nng_pipe_getopt_ms(arg1: nng_pipe,
                              arg2: *const ::std::os::raw::c_char,
                              arg3: *mut nng_duration)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_size"]
    pub fn nng_pipe_getopt_size(arg1: nng_pipe,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_sockaddr"]
    pub fn nng_pipe_getopt_sockaddr(arg1: nng_pipe,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut nng_sockaddr)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_uint64"]
    pub fn nng_pipe_getopt_uint64(arg1: nng_pipe,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_ptr"]
    pub fn nng_pipe_getopt_ptr(arg1: nng_pipe,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_getopt_string"]
    pub fn nng_pipe_getopt_string(arg1: nng_pipe,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_close"]
    pub fn nng_pipe_close(arg1: nng_pipe) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_id"]
    pub fn nng_pipe_id(arg1: nng_pipe) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_pipe_socket"]
    pub fn nng_pipe_socket(arg1: nng_pipe) -> nng_socket;
}
extern "C" {
    //#[link_name = "_nng_pipe_dialer"]
    pub fn nng_pipe_dialer(arg1: nng_pipe) -> nng_dialer;
}
extern "C" {
    //#[link_name = "_nng_pipe_listener"]
    pub fn nng_pipe_listener(arg1: nng_pipe) -> nng_listener;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_flag_enum { NNG_FLAG_ALLOC = 1, NNG_FLAG_NONBLOCK = 2, }
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_stat_type_enum { NNG_STAT_LEVEL = 0, NNG_STAT_COUNTER = 1, }
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_unit_enum {
    NNG_UNIT_NONE = 0,
    NNG_UNIT_BYTES = 1,
    NNG_UNIT_MESSAGES = 2,
    NNG_UNIT_BOOLEAN = 3,
    NNG_UNIT_MILLIS = 4,
    NNG_UNIT_EVENTS = 5,
}
extern "C" {
    //#[link_name = "_nng_device"]
    pub fn nng_device(arg1: nng_socket, arg2: nng_socket)
     -> ::std::os::raw::c_int;
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nng_errno_enum {
    NNG_EINTR = 1,
    NNG_ENOMEM = 2,
    NNG_EINVAL = 3,
    NNG_EBUSY = 4,
    NNG_ETIMEDOUT = 5,
    NNG_ECONNREFUSED = 6,
    NNG_ECLOSED = 7,
    NNG_EAGAIN = 8,
    NNG_ENOTSUP = 9,
    NNG_EADDRINUSE = 10,
    NNG_ESTATE = 11,
    NNG_ENOENT = 12,
    NNG_EPROTO = 13,
    NNG_EUNREACHABLE = 14,
    NNG_EADDRINVAL = 15,
    NNG_EPERM = 16,
    NNG_EMSGSIZE = 17,
    NNG_ECONNABORTED = 18,
    NNG_ECONNRESET = 19,
    NNG_ECANCELED = 20,
    NNG_ENOFILES = 21,
    NNG_ENOSPC = 22,
    NNG_EEXIST = 23,
    NNG_EREADONLY = 24,
    NNG_EWRITEONLY = 25,
    NNG_ECRYPTO = 26,
    NNG_EPEERAUTH = 27,
    NNG_ENOARG = 28,
    NNG_EAMBIGUOUS = 29,
    NNG_EBADTYPE = 30,
    NNG_EINTERNAL = 1000,
    NNG_ESYSERR = 268435456,
    NNG_ETRANERR = 536870912,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nng_url {
    pub u_rawurl: *mut ::std::os::raw::c_char,
    pub u_scheme: *mut ::std::os::raw::c_char,
    pub u_userinfo: *mut ::std::os::raw::c_char,
    pub u_host: *mut ::std::os::raw::c_char,
    pub u_hostname: *mut ::std::os::raw::c_char,
    pub u_port: *mut ::std::os::raw::c_char,
    pub u_path: *mut ::std::os::raw::c_char,
    pub u_query: *mut ::std::os::raw::c_char,
    pub u_fragment: *mut ::std::os::raw::c_char,
    pub u_requri: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_nng_url() {
    assert_eq!(::std::mem::size_of::<nng_url>() , 40usize);
    assert_eq!(::std::mem::align_of::<nng_url>() , 4usize);
}
impl Clone for nng_url {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    //#[link_name = "_nng_url_parse"]
    pub fn nng_url_parse(arg1: *mut *mut nng_url,
                         arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_url_free"]
    pub fn nng_url_free(arg1: *mut nng_url);
}
extern "C" {
    //#[link_name = "_nng_url_clone"]
    pub fn nng_url_clone(arg1: *mut *mut nng_url, arg2: *const nng_url)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nng_version"]
    pub fn nng_version() -> *const ::std::os::raw::c_char;
}
pub type __crt_bool = bool;
extern "C" {
    //#[link_name = "__invalid_parameter_noinfo"]
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    //#[link_name = "__invalid_parameter_noinfo_noreturn"]
    pub fn _invalid_parameter_noinfo_noreturn();
}
extern "C" {
    //#[link_name = "__invoke_watson"]
    pub fn _invoke_watson(arg1: *const wchar_t, arg2: *const wchar_t,
                          arg3: *const wchar_t, arg4: ::std::os::raw::c_uint,
                          arg5: usize);
}
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
impl Clone for __crt_locale_data_public {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_pointers___crt_locale_data,
    pub mbcinfo: *mut __crt_locale_pointers___crt_multibyte_data,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers___crt_locale_data([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers___crt_multibyte_data([u8; 0]);
impl Clone for __crt_locale_pointers {
    fn clone(&self) -> Self { *self }
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
impl Clone for _Mbstatet {
    fn clone(&self) -> Self { *self }
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
extern "C" {
    //#[link_name = "__errno"]
    pub fn _errno() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "__set_errno"]
    pub fn _set_errno(_Value: ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    //#[link_name = "__get_errno"]
    pub fn _get_errno(_Value: *mut ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    //#[link_name = "___doserrno"]
    pub fn __doserrno() -> *mut ::std::os::raw::c_ulong;
}
extern "C" {
    //#[link_name = "__set_doserrno"]
    pub fn _set_doserrno(_Value: ::std::os::raw::c_ulong) -> errno_t;
}
extern "C" {
    //#[link_name = "__get_doserrno"]
    pub fn _get_doserrno(_Value: *mut ::std::os::raw::c_ulong) -> errno_t;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nn_pollfd {
    pub fd: ::std::os::raw::c_int,
    pub events: u16,
    pub revents: u16,
}
#[test]
fn bindgen_test_layout_nn_pollfd() {
    assert_eq!(::std::mem::size_of::<nn_pollfd>() , 8usize);
    assert_eq!(::std::mem::align_of::<nn_pollfd>() , 4usize);
}
impl Clone for nn_pollfd {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nn_iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}
#[test]
fn bindgen_test_layout_nn_iovec() {
    assert_eq!(::std::mem::size_of::<nn_iovec>() , 8usize);
    assert_eq!(::std::mem::align_of::<nn_iovec>() , 4usize);
}
impl Clone for nn_iovec {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nn_msghdr {
    pub msg_iov: *mut nn_iovec,
    pub msg_iovlen: ::std::os::raw::c_int,
    pub msg_control: *mut ::std::os::raw::c_void,
    pub msg_controllen: usize,
}
#[test]
fn bindgen_test_layout_nn_msghdr() {
    assert_eq!(::std::mem::size_of::<nn_msghdr>() , 16usize);
    assert_eq!(::std::mem::align_of::<nn_msghdr>() , 4usize);
}
impl Clone for nn_msghdr {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nn_cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: ::std::os::raw::c_int,
    pub cmsg_type: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_nn_cmsghdr() {
    assert_eq!(::std::mem::size_of::<nn_cmsghdr>() , 12usize);
    assert_eq!(::std::mem::align_of::<nn_cmsghdr>() , 4usize);
}
impl Clone for nn_cmsghdr {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    //#[link_name = "_nn_cmsg_next"]
    pub fn nn_cmsg_next(arg1: *mut nn_msghdr, arg2: *mut nn_cmsghdr)
     -> *mut nn_cmsghdr;
}
extern "C" {
    //#[link_name = "_nn_socket"]
    pub fn nn_socket(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_setsockopt"]
    pub fn nn_setsockopt(arg1: ::std::os::raw::c_int,
                         arg2: ::std::os::raw::c_int,
                         arg3: ::std::os::raw::c_int,
                         arg4: *const ::std::os::raw::c_void, arg5: usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_getsockopt"]
    pub fn nn_getsockopt(arg1: ::std::os::raw::c_int,
                         arg2: ::std::os::raw::c_int,
                         arg3: ::std::os::raw::c_int,
                         arg4: *mut ::std::os::raw::c_void, arg5: *mut usize)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_bind"]
    pub fn nn_bind(arg1: ::std::os::raw::c_int,
                   arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_connect"]
    pub fn nn_connect(arg1: ::std::os::raw::c_int,
                      arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_shutdown"]
    pub fn nn_shutdown(arg1: ::std::os::raw::c_int,
                       arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_send"]
    pub fn nn_send(arg1: ::std::os::raw::c_int,
                   arg2: *const ::std::os::raw::c_void, arg3: usize,
                   arg4: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_recv"]
    pub fn nn_recv(arg1: ::std::os::raw::c_int,
                   arg2: *mut ::std::os::raw::c_void, arg3: usize,
                   arg4: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_sendmsg"]
    pub fn nn_sendmsg(arg1: ::std::os::raw::c_int, arg2: *const nn_msghdr,
                      arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_recvmsg"]
    pub fn nn_recvmsg(arg1: ::std::os::raw::c_int, arg2: *mut nn_msghdr,
                      arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_close"]
    pub fn nn_close(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_poll"]
    pub fn nn_poll(arg1: *mut nn_pollfd, arg2: ::std::os::raw::c_int,
                   arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_device"]
    pub fn nn_device(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_get_statistic"]
    pub fn nn_get_statistic(arg1: ::std::os::raw::c_int,
                            arg2: ::std::os::raw::c_int) -> u64;
}
extern "C" {
    //#[link_name = "_nn_allocmsg"]
    pub fn nn_allocmsg(arg1: usize, arg2: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nn_reallocmsg"]
    pub fn nn_reallocmsg(arg1: *mut ::std::os::raw::c_void, arg2: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nn_freemsg"]
    pub fn nn_freemsg(arg1: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_errno"]
    pub fn nn_errno() -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_strerror"]
    pub fn nn_strerror(arg1: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    //#[link_name = "_nn_term"]
    pub fn nn_term();
}

*/