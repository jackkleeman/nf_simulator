use iptables::*;
use libc;
use std::io::Read;
use std::io::Write;
use tempfile::NamedTempFile;
use std::os::unix::io::AsRawFd;
use super::super::wrapper;

#[used]
#[no_mangle]
static DEBUG: i32 = 10;

extern "C" {
    static mut optind: libc::c_int;
}

pub fn save() -> String {
    let mut args = [
        std::ffi::CString::new("iptables-save").unwrap().into_raw(),
    ];

    let dup = unsafe { libc::dup(1) };
    if dup < 0 {
        panic!("bad dup")
    }

    let tempfile = NamedTempFile::new().unwrap();
    let inner = tempfile.reopen().unwrap();
    let mut outer = tempfile.reopen().unwrap();

    if unsafe { libc::dup2(inner.as_raw_fd(), 1) } < 0 {
        panic!("bad dup2")
    };

    unsafe {
        wrapped!{
            xtables_ip4_save_main(args.len() as i32, args.as_mut_ptr());
        }
        optind = 0;
        libc::fflush(std::ptr::null_mut());
        xtables_pending_targets = std::ptr::null_mut();
        xtables_targets = std::ptr::null_mut();
    }

    if unsafe { libc::dup2(dup, 1) } < 0 {
        panic!("bad dup2")
    };
    drop(inner);

    let mut output = String::new();
    outer.read_to_string(&mut output).unwrap();
    output
}
pub fn restore(d: String) {
    let mut args = [
        std::ffi::CString::new("iptables-restore").unwrap().into_raw(),
        // std::ffi::CString::new("-t").unwrap().into_raw(),
    ];

    let dup = unsafe { libc::dup(0) };
    if dup < 0 {
        panic!("bad dup")
    }

    let tempfile = NamedTempFile::new().unwrap();
    let inner = tempfile.reopen().unwrap();
    let mut outer = tempfile.reopen().unwrap();

    outer.write_all(d.as_bytes()).unwrap();
    drop(outer);

    if unsafe { libc::dup2(inner.as_raw_fd(), 0) } < 0 {
        panic!("bad dup2")
    };
    
    unsafe {
        wrapped!{
            xtables_ip4_restore_main(args.len() as i32, args.as_mut_ptr());
        }
        optind = 0;
        libc::fflush(std::ptr::null_mut());
        xtables_pending_targets = std::ptr::null_mut();
        xtables_targets = std::ptr::null_mut();
    }

    if unsafe { libc::dup2(dup, 0) } < 0 {
        panic!("bad dup2")
    };
    drop(inner);
}