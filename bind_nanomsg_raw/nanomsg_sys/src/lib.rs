#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_int};
pub const NN_ZERO: ::std::os::raw::c_int = 0;
pub const NN_PROTO_PIPELINE: c_int = 5;
pub const NN_PUSH: c_int = NN_PROTO_PIPELINE * 16 + 0;
pub const NN_PULL: c_int = NN_PROTO_PIPELINE * 16 + 1;
pub const NN_PROTO_PAIR: c_int = 1;
pub const NN_PAIR: c_int = NN_PROTO_PAIR * 16 + 0;

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const _SAL_VERSION: ::std::os::raw::c_uint = 20;
pub const __SAL_H_VERSION: ::std::os::raw::c_uint = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: ::std::os::raw::c_uint = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: ::std::os::raw::c_uint = 0;
pub const _CRT_PACKING: ::std::os::raw::c_uint = 8;
pub const _HAS_EXCEPTIONS: ::std::os::raw::c_uint = 1;
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
pub const WCHAR_MIN: ::std::os::raw::c_uint = 0;
pub const WCHAR_MAX: ::std::os::raw::c_uint = 65535;
pub const WINT_MIN: ::std::os::raw::c_uint = 0;
pub const WINT_MAX: ::std::os::raw::c_uint = 65535;
pub const NN_VERSION_CURRENT: ::std::os::raw::c_uint = 5;
pub const NN_VERSION_REVISION: ::std::os::raw::c_uint = 1;
pub const NN_VERSION_AGE: ::std::os::raw::c_uint = 0;
pub const NN_HAUSNUMERO: ::std::os::raw::c_uint = 156384712;
pub const EACCESS: ::std::os::raw::c_uint = 13;
pub const ESOCKTNOSUPPORT: ::std::os::raw::c_uint = 156384740;
pub const ETERM: ::std::os::raw::c_uint = 156384765;
pub const EFSM: ::std::os::raw::c_uint = 156384766;
pub const NN_NS_NAMESPACE: ::std::os::raw::c_uint = 0;
pub const NN_NS_VERSION: ::std::os::raw::c_uint = 1;
pub const NN_NS_DOMAIN: ::std::os::raw::c_uint = 2;
pub const NN_NS_TRANSPORT: ::std::os::raw::c_uint = 3;
pub const NN_NS_PROTOCOL: ::std::os::raw::c_uint = 4;
pub const NN_NS_OPTION_LEVEL: ::std::os::raw::c_uint = 5;
pub const NN_NS_SOCKET_OPTION: ::std::os::raw::c_uint = 6;
pub const NN_NS_TRANSPORT_OPTION: ::std::os::raw::c_uint = 7;
pub const NN_NS_OPTION_TYPE: ::std::os::raw::c_uint = 8;
pub const NN_NS_OPTION_UNIT: ::std::os::raw::c_uint = 9;
pub const NN_NS_FLAG: ::std::os::raw::c_uint = 10;
pub const NN_NS_ERROR: ::std::os::raw::c_uint = 11;
pub const NN_NS_LIMIT: ::std::os::raw::c_uint = 12;
pub const NN_NS_EVENT: ::std::os::raw::c_uint = 13;
pub const NN_NS_STATISTIC: ::std::os::raw::c_uint = 14;
pub const NN_TYPE_NONE: ::std::os::raw::c_uint = 0;
pub const NN_TYPE_INT: ::std::os::raw::c_uint = 1;
pub const NN_TYPE_STR: ::std::os::raw::c_uint = 2;
pub const NN_UNIT_NONE: ::std::os::raw::c_uint = 0;
pub const NN_UNIT_BYTES: ::std::os::raw::c_uint = 1;
pub const NN_UNIT_MILLISECONDS: ::std::os::raw::c_uint = 2;
pub const NN_UNIT_PRIORITY: ::std::os::raw::c_uint = 3;
pub const NN_UNIT_BOOLEAN: ::std::os::raw::c_uint = 4;
pub const NN_UNIT_MESSAGES: ::std::os::raw::c_uint = 5;
pub const NN_UNIT_COUNTER: ::std::os::raw::c_uint = 6;
pub const AF_SP: ::std::os::raw::c_uint = 1;
pub const AF_SP_RAW: ::std::os::raw::c_uint = 2;
pub const NN_SOCKADDR_MAX: ::std::os::raw::c_uint = 128;
pub const NN_SOL_SOCKET: ::std::os::raw::c_uint = 0;
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
pub const NN_DONTWAIT: ::std::os::raw::c_uint = 1;
pub const PROTO_SP: ::std::os::raw::c_uint = 1;
pub const SP_HDR: ::std::os::raw::c_uint = 1;
pub const NN_POLLIN: ::std::os::raw::c_uint = 1;
pub const NN_POLLOUT: ::std::os::raw::c_uint = 2;
pub const NN_STAT_ESTABLISHED_CONNECTIONS: ::std::os::raw::c_uint = 101;
pub const NN_STAT_ACCEPTED_CONNECTIONS: ::std::os::raw::c_uint = 102;
pub const NN_STAT_DROPPED_CONNECTIONS: ::std::os::raw::c_uint = 103;
pub const NN_STAT_BROKEN_CONNECTIONS: ::std::os::raw::c_uint = 104;
pub const NN_STAT_CONNECT_ERRORS: ::std::os::raw::c_uint = 105;
pub const NN_STAT_BIND_ERRORS: ::std::os::raw::c_uint = 106;
pub const NN_STAT_ACCEPT_ERRORS: ::std::os::raw::c_uint = 107;
pub const NN_STAT_CURRENT_CONNECTIONS: ::std::os::raw::c_uint = 201;
pub const NN_STAT_INPROGRESS_CONNECTIONS: ::std::os::raw::c_uint = 202;
pub const NN_STAT_CURRENT_EP_ERRORS: ::std::os::raw::c_uint = 203;
pub const NN_STAT_MESSAGES_SENT: ::std::os::raw::c_uint = 301;
pub const NN_STAT_MESSAGES_RECEIVED: ::std::os::raw::c_uint = 302;
pub const NN_STAT_BYTES_SENT: ::std::os::raw::c_uint = 303;
pub const NN_STAT_BYTES_RECEIVED: ::std::os::raw::c_uint = 304;
pub const NN_STAT_CURRENT_SND_PRIORITY: ::std::os::raw::c_uint = 401;
pub type va_list = *mut ::std::os::raw::c_char;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
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
pub type max_align_t = f64;
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
extern "C" {
    //#[link_name = "_nn_errno"]
    pub fn nn_errno() -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_strerror"]
    pub fn nn_strerror(errnum: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    //#[link_name = "_nn_symbol"]
    pub fn nn_symbol(i: ::std::os::raw::c_int,
                     value: *mut ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nn_symbol_properties {
    pub value: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
    pub ns: ::std::os::raw::c_int,
    pub type_: ::std::os::raw::c_int,
    pub unit: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_nn_symbol_properties() {
    assert_eq!(::std::mem::size_of::<nn_symbol_properties>() , 20usize);
    assert_eq!(::std::mem::align_of::<nn_symbol_properties>() , 4usize);
}
impl Clone for nn_symbol_properties {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    //#[link_name = "_nn_symbol_info"]
    pub fn nn_symbol_info(i: ::std::os::raw::c_int,
                          buf: *mut nn_symbol_properties,
                          buflen: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    /******************************************************************************/
    //#[link_name = "_nn_term"]
    pub fn nn_term();
}
extern "C" {
    //#[link_name = "_nn_allocmsg"]
    pub fn nn_allocmsg(size: usize, type_: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nn_reallocmsg"]
    pub fn nn_reallocmsg(msg: *mut ::std::os::raw::c_void, size: usize)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    //#[link_name = "_nn_freemsg"]
    pub fn nn_freemsg(msg: *mut ::std::os::raw::c_void)
     -> ::std::os::raw::c_int;
}
/******************************************************************************/
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
    //#[link_name = "_nn_cmsg_nxthdr_"]
    pub fn nn_cmsg_nxthdr_(mhdr: *const nn_msghdr, cmsg: *const nn_cmsghdr)
     -> *mut nn_cmsghdr;
}
extern "C" {
    ////#[link_name = "_nn_socket"]
    pub fn nn_socket(domain: ::std::os::raw::c_int,
                     protocol: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_close"]
    pub fn nn_close(s: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_setsockopt"]
    pub fn nn_setsockopt(s: ::std::os::raw::c_int,
                         level: ::std::os::raw::c_int,
                         option: ::std::os::raw::c_int,
                         optval: *const ::std::os::raw::c_void,
                         optvallen: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_getsockopt"]
    pub fn nn_getsockopt(s: ::std::os::raw::c_int,
                         level: ::std::os::raw::c_int,
                         option: ::std::os::raw::c_int,
                         optval: *mut ::std::os::raw::c_void,
                         optvallen: *mut usize) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_bind"]
    pub fn nn_bind(s: ::std::os::raw::c_int,
                   addr: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_connect"]
    pub fn nn_connect(s: ::std::os::raw::c_int,
                      addr: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_shutdown"]
    pub fn nn_shutdown(s: ::std::os::raw::c_int, how: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_send"]
    pub fn nn_send(s: ::std::os::raw::c_int,
                   buf: *const ::std::os::raw::c_void, len: usize,
                   flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_recv"]
    pub fn nn_recv(s: ::std::os::raw::c_int, buf: *mut ::std::os::raw::c_void,
                   len: usize, flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_sendmsg"]
    pub fn nn_sendmsg(s: ::std::os::raw::c_int, msghdr: *const nn_msghdr,
                      flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_recvmsg"]
    pub fn nn_recvmsg(s: ::std::os::raw::c_int, msghdr: *mut nn_msghdr,
                      flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nn_pollfd {
    pub fd: ::std::os::raw::c_int,
    pub events: ::std::os::raw::c_short,
    pub revents: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_nn_pollfd() {
    assert_eq!(::std::mem::size_of::<nn_pollfd>() , 8usize);
    assert_eq!(::std::mem::align_of::<nn_pollfd>() , 4usize);
}
impl Clone for nn_pollfd {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    //#[link_name = "_nn_poll"]
    pub fn nn_poll(fds: *mut nn_pollfd, nfds: ::std::os::raw::c_int,
                   timeout: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    /******************************************************************************/
    //#[link_name = "_nn_device"]
    pub fn nn_device(s1: ::std::os::raw::c_int, s2: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    //#[link_name = "_nn_get_statistic"]
    pub fn nn_get_statistic(s: ::std::os::raw::c_int,
                            stat: ::std::os::raw::c_int) -> u64;
}


