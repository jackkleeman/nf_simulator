use std;
use lkl;
use std::ffi::CString;
use std::thread;
use libc::{c_int, c_void, socklen_t};

// use errno;

#[macro_use]
mod wrapper;
mod syscalls;

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

    unsafe {
        socket_test();
        wrapper::switch_libc(false);
        socket_test();
        socket_test();
    }
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

unsafe fn socket_test() {

    let sockfd = libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_RAW);
	if sockfd < 0 {
		panic!("bad sockfd")
    }
    let sockfd_2 = libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_RAW);
	if sockfd < 0 {
		panic!("bad sockfd")
    }
    libc::close(sockfd);
    libc::close(sockfd_2);
}