extern crate libc;

use std::io;
use std::ffi::CStr;

use libc::{utsname, c_char};

#[derive(Debug)]
pub struct Info {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    _priv: (),
}

impl Info {
    pub fn new() -> io::Result<Self> {
        let mut n = unsafe { std::mem::zeroed() };
        let r = unsafe { libc::uname(&mut n) };
        if r == 0 {
            Ok(From::from(n))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[inline]
fn parse(buf: &[c_char]) -> String {
    let buf: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) };
    let s = CStr::from_bytes_with_nul(buf).unwrap();
    s.to_string_lossy().into_owned()
}

impl From<utsname> for Info {
    fn from(x: utsname) -> Self {
        let info = Info {
            sysname: parse(&x.sysname[..]),
            nodename: parse(&x.nodename[..]),
            release: parse(&x.release[..]),
            version: parse(&x.version[..]),
            machine: parse(&x.machine[..]),
            _priv: (),
        };
        info
    }
}

pub fn uname() -> io::Result<Info> {
    Info::new()
}
