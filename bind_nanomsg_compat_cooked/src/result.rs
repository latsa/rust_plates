extern crate libc;

use libc::{c_int};
use nng_sys;

use std::str;
use std::fmt;
use std::io;
use std::convert::From;
use std::ffi::CStr;
use std::result;
use std::error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Unknown                    = 0 as isize,
    OperationNotSupported      = nng_sys::ENOTSUP          as isize,
    ProtocolNotSupported       = nng_sys::EPROTONOSUPPORT  as isize,
    NoBufferSpace              = nng_sys::ENOBUFS          as isize,
    NetworkDown                = nng_sys::ENETDOWN         as isize,
    AddressInUse               = nng_sys::EADDRINUSE       as isize,
    AddressNotAvailable        = nng_sys::EADDRNOTAVAIL    as isize,
    ConnectionRefused          = nng_sys::ECONNREFUSED     as isize,
    OperationNowInProgress     = nng_sys::EINPROGRESS      as isize,
    NotSocket                  = nng_sys::ENOTSOCK         as isize,
    AddressFamilyNotSupported  = nng_sys::EAFNOSUPPORT     as isize,
    #[cfg(not(target_os = "openbsd"))]
    WrongProtocol              = nng_sys::EPROTO           as isize,
    #[cfg(target_os = "openbsd")]
    WrongProtocol              = nng_sys::EPROTOTYPE       as isize,
    TryAgain                   = nng_sys::EAGAIN           as isize,
    BadFileDescriptor          = nng_sys::EBADF            as isize,
    InvalidInput               = nng_sys::EINVAL           as isize,
    TooManyOpenFiles           = nng_sys::EMFILE           as isize,
    BadAddress                 = nng_sys::EFAULT           as isize,
    PermissionDenied           = nng_sys::EACCESS          as isize,
    NetworkReset               = nng_sys::ENETRESET        as isize,
    NetworkUnreachable         = nng_sys::ENETUNREACH      as isize,
    HostUnreachable            = nng_sys::EHOSTUNREACH     as isize,
    NotConnected               = nng_sys::ENOTCONN         as isize,
    MessageTooLong             = nng_sys::EMSGSIZE         as isize,
    TimedOut                   = nng_sys::ETIMEDOUT        as isize,
    ConnectionAborted          = nng_sys::ECONNABORTED     as isize,
    ConnectionReset            = nng_sys::ECONNRESET       as isize,
    ProtocolNotAvailable       = nng_sys::ENOPROTOOPT      as isize,
    AlreadyConnected           = nng_sys::EISCONN          as isize,
    SocketTypeNotSupported     = nng_sys::ESOCKTNOSUPPORT  as isize,
    Terminating                = nng_sys::ETERM            as isize,
    NameTooLong                = nng_sys::ENAMETOOLONG     as isize,
    NoDevice                   = nng_sys::ENODEV           as isize,
    FileStateMismatch          = nng_sys::EFSM             as isize,
    Interrupted                = nng_sys::EINTR            as isize
}

impl Error {
    pub fn to_raw(&self) -> c_int {
        *self as c_int
    }

    pub fn from_raw(raw: c_int) -> Error {
        match raw {
            nng_sys::ENOTSUP         => Error::OperationNotSupported    ,
            nng_sys::EPROTONOSUPPORT => Error::ProtocolNotSupported     ,
            nng_sys::ENOBUFS         => Error::NoBufferSpace            ,
            nng_sys::ENETDOWN        => Error::NetworkDown              ,
            nng_sys::EADDRINUSE      => Error::AddressInUse             ,
            nng_sys::EADDRNOTAVAIL   => Error::AddressNotAvailable      ,
            nng_sys::ECONNREFUSED    => Error::ConnectionRefused        ,
            nng_sys::EINPROGRESS     => Error::OperationNowInProgress   ,
            nng_sys::ENOTSOCK        => Error::NotSocket                ,
            nng_sys::EAFNOSUPPORT    => Error::AddressFamilyNotSupported,
            #[cfg(not(target_os = "openbsd"))]
            nng_sys::EPROTO          => Error::WrongProtocol            ,
            #[cfg(target_os = "openbsd")]
            nng_sys::EPROTOTYPE      => Error::WrongProtocol            ,
            nng_sys::EAGAIN          => Error::TryAgain                 ,
            nng_sys::EBADF           => Error::BadFileDescriptor        ,
            nng_sys::EINVAL          => Error::InvalidInput             ,
            nng_sys::EMFILE          => Error::TooManyOpenFiles         ,
            nng_sys::EFAULT          => Error::BadAddress               ,
            nng_sys::EACCESS         => Error::PermissionDenied         ,
            nng_sys::ENETRESET       => Error::NetworkReset             ,
            nng_sys::ENETUNREACH     => Error::NetworkUnreachable       ,
            nng_sys::EHOSTUNREACH    => Error::HostUnreachable          ,
            nng_sys::ENOTCONN        => Error::NotConnected             ,
            nng_sys::EMSGSIZE        => Error::MessageTooLong           ,
            nng_sys::ETIMEDOUT       => Error::TimedOut                 ,
            nng_sys::ECONNABORTED    => Error::ConnectionAborted        ,
            nng_sys::ECONNRESET      => Error::ConnectionReset          ,
            nng_sys::ENOPROTOOPT     => Error::ProtocolNotAvailable     ,
            nng_sys::EISCONN         => Error::AlreadyConnected         ,
            nng_sys::ESOCKTNOSUPPORT => Error::SocketTypeNotSupported   ,
            nng_sys::ETERM           => Error::Terminating              ,
            nng_sys::ENAMETOOLONG    => Error::NameTooLong              ,
            nng_sys::ENODEV          => Error::NoDevice                 ,
            nng_sys::EFSM            => Error::FileStateMismatch        ,
            nng_sys::EINTR           => Error::Interrupted              ,
            _                            => Error::Unknown
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        unsafe {
            let nn_errno = *self as c_int;
            let c_ptr: *const libc::c_char = nng_sys::nn_strerror(nn_errno);
            let c_str = CStr::from_ptr(c_ptr);
            let bytes = c_str.to_bytes();
            let desc = str::from_utf8(bytes).unwrap_or("Error");

            desc
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        match err.kind() {
            io::ErrorKind::PermissionDenied    => Error::PermissionDenied,
            io::ErrorKind::ConnectionRefused   => Error::ConnectionRefused,
            io::ErrorKind::ConnectionReset     => Error::ConnectionReset,
            io::ErrorKind::ConnectionAborted   => Error::ConnectionAborted,
            io::ErrorKind::NotConnected        => Error::NotConnected,
            io::ErrorKind::AddrInUse           => Error::AddressInUse,
            io::ErrorKind::AddrNotAvailable    => Error::AddressNotAvailable,
            io::ErrorKind::AlreadyExists       => Error::AlreadyConnected,
            io::ErrorKind::WouldBlock          => Error::TryAgain,
            io::ErrorKind::InvalidInput        => Error::InvalidInput,
            io::ErrorKind::TimedOut            => Error::TimedOut,
            io::ErrorKind::Interrupted         => Error::Interrupted,
            _                                  => Error::Unknown
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        let as_std_error: &error::Error = &err;
        let description = as_std_error.description();
        match err {
            Error::PermissionDenied      => io::Error::new(io::ErrorKind::PermissionDenied,  description ),
            Error::ConnectionRefused     => io::Error::new(io::ErrorKind::ConnectionRefused, description ),
            Error::ConnectionReset       => io::Error::new(io::ErrorKind::ConnectionReset,   description ),
            Error::ConnectionAborted     => io::Error::new(io::ErrorKind::ConnectionAborted, description ),
            Error::NotConnected          => io::Error::new(io::ErrorKind::NotConnected,      description ),
            Error::AddressInUse          => io::Error::new(io::ErrorKind::AddrInUse,         description ),
            Error::AddressNotAvailable   => io::Error::new(io::ErrorKind::AddrNotAvailable,  description ),
            Error::AlreadyConnected      => io::Error::new(io::ErrorKind::AlreadyExists,     description ),
            Error::TryAgain              => io::Error::new(io::ErrorKind::WouldBlock,        description ),
            Error::InvalidInput          => io::Error::new(io::ErrorKind::InvalidInput,      description ),
            Error::TimedOut              => io::Error::new(io::ErrorKind::TimedOut,          description ),
            Error::Interrupted           => io::Error::new(io::ErrorKind::Interrupted,       description ),
            _                            => io::Error::new(io::ErrorKind::Other,             description )
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let as_std_error: &error::Error = self;
        let description = as_std_error.description();
        write!(formatter, "{}", description)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let as_std_error: &error::Error = self;
        let description = as_std_error.description();
        write!(formatter, "{}", description)
    }
}

pub fn last_nano_error() -> Error {
    let nn_errno = unsafe { nng_sys::nn_errno() };

    Error::from_raw(nn_errno)
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use nng_sys;
    use libc;
    use super::{Error};
    use std::io;
    use std::convert::From;

    fn assert_convert_error_code_to_error(error_code: libc::c_int, expected_error: Error) {
        let converted_error = Error::from_raw(error_code);
        assert_eq!(expected_error, converted_error)
    }

    #[test]
    fn can_convert_error_code_to_error() {
        assert_convert_error_code_to_error(nng_sys::ENOTSUP, Error::OperationNotSupported);
        assert_convert_error_code_to_error(nng_sys::EPROTONOSUPPORT, Error::ProtocolNotSupported);
        assert_convert_error_code_to_error(nng_sys::EADDRINUSE, Error::AddressInUse);
        assert_convert_error_code_to_error(nng_sys::EHOSTUNREACH, Error::HostUnreachable);
    }

    fn check_error_kind_match(nano_err: Error, io_err_kind: io::ErrorKind) {
        let io_err: io::Error = From::from(nano_err);

        assert_eq!(io_err_kind, io_err.kind())
    }

    #[test]
    fn nano_err_can_be_converted_to_io_err() {
        check_error_kind_match(Error::TimedOut, io::ErrorKind::TimedOut);
        check_error_kind_match(Error::PermissionDenied, io::ErrorKind::PermissionDenied);
        check_error_kind_match(Error::ConnectionRefused, io::ErrorKind::ConnectionRefused);
        check_error_kind_match(Error::OperationNotSupported, io::ErrorKind::Other);
        check_error_kind_match(Error::NotConnected, io::ErrorKind::NotConnected);
        check_error_kind_match(Error::Interrupted, io::ErrorKind::Interrupted);
    }

    #[test]
    fn nano_err_can_be_converted_from_io_err() {
        let io_err = io::Error::new(io::ErrorKind::TimedOut, "Timed out");
        let nano_err: Error = From::from(io_err);

        assert_eq!(Error::TimedOut, nano_err)
    }
}
