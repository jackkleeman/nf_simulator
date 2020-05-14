use std;
use lkl;
use std::ffi::CString;
use std::thread;
use libc::{c_int, c_void, socklen_t, sockaddr, ssize_t, size_t, msghdr, fd_set, timeval, time_t};

// use errno;

#[macro_use]
mod wrapper;
mod syscalls;
mod dump;

use syscalls::*;

extern "C" {
    static mut lkl_host_ops: lkl::lkl_host_operations;
}

pub fn main() {
    let handle = thread::spawn(|| {
        unsafe {
            let linux_ops = &mut lkl_host_ops;
            let boot_arg = CString::new("mem=16M loglevel=8").expect("CString problem");
            let r = lkl::lkl_start_kernel(linux_ops, boot_arg.as_ptr() as *const u8);
            println!("lkl_start_kernel {}", r);
        }
    });

    handle.join().unwrap();

    let d = dump::save();
    println!("first: {}", d);
    // switch to lkl
    wrapper::switch_libc(false);

    dump::restore(d);
    let d = dump::save();
    println!("second: {}", d);

    // dump::restore(d);
}

wrap_syscall! {
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int
}

wrap_syscall! {
    fn close(fd: c_int) -> c_int
}

wrap_syscall! {
    fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int
}

wrap_syscall!{
    fn fcntl64(fd: c_int, cmd: c_int, arg: c_int) -> c_int
}

wrap_syscall!{
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int
}

wrap_syscall!{
    fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
}

wrap_syscall!{
    fn sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
}

wrap_syscall!{
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t
}

wrap_syscall!{
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t
}

wrap_syscall!{
    fn setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
}

static SELECT_DLFN: wrapper::DlSym<unsafe extern fn(c_int, *mut fd_set, *mut fd_set, *mut fd_set, *mut timeval) -> c_int> =
    wrapper::DlSym {
        name: "select\0",
        addr: ::std::sync::atomic::AtomicUsize::new(0),
        _marker: ::std::marker::PhantomData,
    }; 

const _LKL_NSIG: libc::c_long = 64;

// lkl doesn't define a select syscall, so we do some compat magic (based on sgx-lkl-musl)
#[no_mangle]
extern "C" fn select(nfds: c_int, readfs: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> c_int {
    let l = wrapper::with_libc();
    let w = wrapper::with_wrapping();

    if w && !l {
        // eprintln!("lkl call to select");
        let data: &[libc::c_long; 2] = &[0, _LKL_NSIG/8 ];
        let args = &mut[
            nfds as libc::c_long,
            readfs as libc::c_long,
            writefds as libc::c_long,
            errorfds as libc::c_long,
            0, // null pointer to timeout by default
            data.as_ptr() as libc::c_long,
        ];
        if !timeout.is_null() {
            // we know its not null, get it as a ref
            let _timeout = unsafe{&*timeout};
            let mut t = libc::timespec{tv_sec: 0, tv_nsec: 0};
            // let mut t: libc::timespec = unsafe { std::mem::zeroed() };
            if _timeout.tv_sec < 0 || _timeout.tv_usec < 0 {
                return -1;
            }
                
            let extra_secs: time_t = _timeout.tv_usec / 1000000;
            t.tv_nsec = _timeout.tv_usec % 1000000 * 1000;
            if  extra_secs > time_t::MAX - _timeout.tv_sec {
                t.tv_sec = time_t::MAX;
            } else {
                t.tv_sec = _timeout.tv_sec + extra_secs;
            }

            args[4] = &mut t as *mut libc::timespec as libc::c_long;
        }
        return wrapper::lkl_call(__NR_pselect6 as libc::c_long, args) as c_int;
    }
    
    return unsafe {SELECT_DLFN.get().unwrap()(nfds, readfs, writefds, errorfds, timeout)};
}
