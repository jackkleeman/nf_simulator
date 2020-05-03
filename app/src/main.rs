use std;
use lkl;
use std::ffi::CString;
use std::thread;

#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types)]
pub type c_long = i64;
#[allow(non_camel_case_types)]
pub type c_ulong = u64;
#[allow(non_camel_case_types)]
pub type c_ulonglong = u64;
#[allow(non_camel_case_types)]
pub type c_void = u64;
#[allow(non_camel_case_types)]
pub type c_char = u8;

extern "C" {
    // int __init lkl_start_kernel(struct lkl_host_operations *ops, const char *fmt, ...)
    // fn lkl_start_kernel(ops: *const lkl::lkl_host_operations, fmt: *const i8) -> i32;
    // fn lkl_sys_halt();
    static mut lkl_host_ops: lkl::lkl_host_operations;
}

pub fn main() {
    println!("ab");
    
    

    let handle = thread::spawn(|| {
        unsafe {
            println!("starting...");
            let linux_ops = get_host_ops();
            let boot_arg = CString::new("mem=16M loglevel=8").expect("CString problem");
            let r = lkl::lkl_start_kernel(linux_ops, boot_arg.as_ptr() as *const u8);
            println!("lkl_start_kernel {}", r);
        }
    });

    handle.join().unwrap();
    println!("aa");
}

// unsafe extern "C" fn print(str: *const c_char, len: c_int) {
//     let cs = CString::from_raw(str as *mut i8);
//     println!("{}", cs.into_string().expect("into string problem"));
// }


fn get_host_ops() -> *mut lkl::lkl_host_operations {
    unsafe {
        return &mut lkl_host_ops;
    }
    // return &lkllkl_host_operations{
    //     virtio_devices: CString::new("").expect("CString problem").as_ptr() as *const u8,
    //     print: Option::Some(print),
    //     panic: Option::None,
    //     sem_alloc: Option::None,
    //     sem_free: Option::None,
    //     sem_up: Option::None,
    //     sem_down: Option::None,
    //     mutex_alloc: Option::None,
    //     mutex_free: Option::None,
    //     mutex_lock: Option::None,
    //     mutex_unlock: Option::None,
    //     thread_create: Option::None,
    //     thread_detach: Option::None,
    //     thread_exit: Option::None,
    //     thread_join: Option::None,
    //     thread_self: Option::None,
    //     thread_equal: Option::None,
    //     tls_alloc: Option::None,
    //     tls_free: Option::None,
    //     tls_set: Option::None,
    //     tls_get: Option::None,
    //     mem_alloc: Option::None,
    //     mem_free: Option::None,
    //     time: Option::None,
    //     timer_alloc: Option::None,
    //     timer_set_oneshot: Option::None,
    //     timer_free: Option::None,
    //     ioremap: Option::None,
    //     iomem_access: Option::None,
    //     gettid: Option::None,
    //     jmp_buf_set: Option::None,
    //     jmp_buf_longjmp: Option::None,
    // };
}

// #[no_mangle]
// pub unsafe extern "C" fn lkl_bug(fmt: *const i8) {
//     println!("lkl_bug");
// }

// #[no_mangle]
// pub unsafe extern "C" fn lkl_printf(fmt: *const i8) {
//     println!("lkl_printf");
// }