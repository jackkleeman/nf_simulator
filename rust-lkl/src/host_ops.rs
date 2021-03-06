/* automatically generated by rust-bindgen */
// bindgen --use-core ~/linux/arch/lkl/include/uapi/asm/host_ops.h > host_ops.rs

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_mutex {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_sem {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_tls_key {
    _unused: [u8; 0],
}
pub type lkl_thread_t = c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_jmp_buf {
    pub buf: [c_ulong; 32usize],
}
#[test]
fn bindgen_test_layout_lkl_jmp_buf() {
    assert_eq!(
        ::core::mem::size_of::<lkl_jmp_buf>(),
        256usize,
        concat!("Size of: ", stringify!(lkl_jmp_buf))
    );
    assert_eq!(
        ::core::mem::align_of::<lkl_jmp_buf>(),
        8usize,
        concat!("Alignment of ", stringify!(lkl_jmp_buf))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_jmp_buf>())).buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_jmp_buf),
            "::",
            stringify!(buf)
        )
    );
}
#[doc = " lkl_host_operations - host operations used by the Linux kernel"]
#[doc = ""]
#[doc = " These operations must be provided by a host library or by the application"]
#[doc = " itself."]
#[doc = ""]
#[doc = " @virtio_devices - string containg the list of virtio devices in virtio mmio"]
#[doc = " command line format. This string is appended to the kernel command line and"]
#[doc = " is provided here for convenience to be implemented by the host library."]
#[doc = ""]
#[doc = " @print - optional operation that receives console messages"]
#[doc = ""]
#[doc = " @panic - called during a kernel panic"]
#[doc = ""]
#[doc = " @sem_alloc - allocate a host semaphore an initialize it to count"]
#[doc = " @sem_free - free a host semaphore"]
#[doc = " @sem_up - perform an up operation on the semaphore"]
#[doc = " @sem_down - perform a down operation on the semaphore"]
#[doc = ""]
#[doc = " @mutex_alloc - allocate and initialize a host mutex; the recursive parameter"]
#[doc = " determines if the mutex is recursive or not"]
#[doc = " @mutex_free - free a host mutex"]
#[doc = " @mutex_lock - acquire the mutex"]
#[doc = " @mutex_unlock - release the mutex"]
#[doc = ""]
#[doc = " @thread_create - create a new thread and run f(arg) in its context; returns a"]
#[doc = " thread handle or 0 if the thread could not be created"]
#[doc = " @thread_detach - on POSIX systems, free up resources held by"]
#[doc = " pthreads. Noop on Win32."]
#[doc = " @thread_exit - terminates the current thread"]
#[doc = " @thread_join - wait for the given thread to terminate. Returns 0"]
#[doc = " for success, -1 otherwise"]
#[doc = ""]
#[doc = " @tls_alloc - allocate a thread local storage key; returns 0 if successful; if"]
#[doc = " destructor is not NULL it will be called when a thread terminates with its"]
#[doc = " argument set to the current thread local storage value"]
#[doc = " @tls_free - frees a thread local storage key; returns 0 if succesful"]
#[doc = " @tls_set - associate data to the thread local storage key; returns 0 if"]
#[doc = " successful"]
#[doc = " @tls_get - return data associated with the thread local storage key or NULL"]
#[doc = " on error"]
#[doc = ""]
#[doc = " @mem_alloc - allocate memory"]
#[doc = " @mem_free - free memory"]
#[doc = ""]
#[doc = " @timer_create - allocate a host timer that runs fn(arg) when the timer"]
#[doc = " fires."]
#[doc = " @timer_free - disarms and free the timer"]
#[doc = " @timer_set_oneshot - arm the timer to fire once, after delta ns."]
#[doc = " @timer_set_periodic - arm the timer to fire periodically, with a period of"]
#[doc = " delta ns."]
#[doc = ""]
#[doc = " @ioremap - searches for an I/O memory region identified by addr and size and"]
#[doc = " returns a pointer to the start of the address range that can be used by"]
#[doc = " iomem_access"]
#[doc = " @iomem_acess - reads or writes to and I/O memory region; addr must be in the"]
#[doc = " range returned by ioremap"]
#[doc = ""]
#[doc = " @gettid - returns the host thread id of the caller, which need not"]
#[doc = " be the same as the handle returned by thread_create"]
#[doc = ""]
#[doc = " @jmp_buf_set - runs the give function and setups a jump back point by saving"]
#[doc = " the context in the jump buffer; jmp_buf_longjmp can be called from the give"]
#[doc = " function or any callee in that function to return back to the jump back"]
#[doc = " point"]
#[doc = ""]
#[doc = " NOTE: we can\'t return from jmp_buf_set before calling jmp_buf_longjmp or"]
#[doc = " otherwise the saved context (stack) is not going to be valid, so we must pass"]
#[doc = " the function that will eventually call longjmp here"]
#[doc = ""]
#[doc = " @jmp_buf_longjmp - perform a jump back to the saved jump buffer"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lkl_host_operations {
    pub virtio_devices: *const c_char,
    pub print: ::core::option::Option<unsafe extern "C" fn(str: *const c_char, len: c_int)>,
    pub panic: ::core::option::Option<unsafe extern "C" fn()>,
    pub sem_alloc: ::core::option::Option<unsafe extern "C" fn(count: c_int) -> *mut lkl_sem>,
    pub sem_free: ::core::option::Option<unsafe extern "C" fn(sem: *mut lkl_sem)>,
    pub sem_up: ::core::option::Option<unsafe extern "C" fn(sem: *mut lkl_sem)>,
    pub sem_down: ::core::option::Option<unsafe extern "C" fn(sem: *mut lkl_sem)>,
    pub mutex_alloc:
        ::core::option::Option<unsafe extern "C" fn(recursive: c_int) -> *mut lkl_mutex>,
    pub mutex_free: ::core::option::Option<unsafe extern "C" fn(mutex: *mut lkl_mutex)>,
    pub mutex_lock: ::core::option::Option<unsafe extern "C" fn(mutex: *mut lkl_mutex)>,
    pub mutex_unlock: ::core::option::Option<unsafe extern "C" fn(mutex: *mut lkl_mutex)>,
    pub thread_create: ::core::option::Option<
        unsafe extern "C" fn(
            f: ::core::option::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            arg: *mut c_void,
        ) -> lkl_thread_t,
    >,
    pub thread_detach: ::core::option::Option<unsafe extern "C" fn()>,
    pub thread_exit: ::core::option::Option<unsafe extern "C" fn()>,
    pub thread_join: ::core::option::Option<unsafe extern "C" fn(tid: lkl_thread_t) -> c_int>,
    pub thread_self: ::core::option::Option<unsafe extern "C" fn() -> lkl_thread_t>,
    pub thread_equal:
        ::core::option::Option<unsafe extern "C" fn(a: lkl_thread_t, b: lkl_thread_t) -> c_int>,
    pub tls_alloc: ::core::option::Option<
        unsafe extern "C" fn(
            destructor: ::core::option::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
        ) -> *mut lkl_tls_key,
    >,
    pub tls_free: ::core::option::Option<unsafe extern "C" fn(key: *mut lkl_tls_key)>,
    pub tls_set: ::core::option::Option<
        unsafe extern "C" fn(key: *mut lkl_tls_key, data: *mut c_void) -> c_int,
    >,
    pub tls_get: ::core::option::Option<unsafe extern "C" fn(key: *mut lkl_tls_key) -> *mut c_void>,
    pub mem_alloc: ::core::option::Option<unsafe extern "C" fn(arg1: c_ulong) -> *mut c_void>,
    pub mem_free: ::core::option::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub time: ::core::option::Option<unsafe extern "C" fn() -> c_ulonglong>,
    pub timer_alloc: ::core::option::Option<
        unsafe extern "C" fn(
            fn_: ::core::option::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            arg: *mut c_void,
        ) -> *mut c_void,
    >,
    pub timer_set_oneshot:
        ::core::option::Option<unsafe extern "C" fn(timer: *mut c_void, delta: c_ulong) -> c_int>,
    pub timer_free: ::core::option::Option<unsafe extern "C" fn(timer: *mut c_void)>,
    pub ioremap:
        ::core::option::Option<unsafe extern "C" fn(addr: c_long, size: c_int) -> *mut c_void>,
    pub iomem_access: ::core::option::Option<
        unsafe extern "C" fn(
            addr: *const c_void,
            val: *mut c_void,
            size: c_int,
            write: c_int,
        ) -> c_int,
    >,
    pub gettid: ::core::option::Option<unsafe extern "C" fn() -> c_long>,
    pub jmp_buf_set: ::core::option::Option<
        unsafe extern "C" fn(
            jmpb: *mut lkl_jmp_buf,
            f: ::core::option::Option<unsafe extern "C" fn()>,
        ),
    >,
    pub jmp_buf_longjmp:
        ::core::option::Option<unsafe extern "C" fn(jmpb: *mut lkl_jmp_buf, val: c_int)>,
}
#[test]
fn bindgen_test_layout_lkl_host_operations() {
    assert_eq!(
        ::core::mem::size_of::<lkl_host_operations>(),
        256usize,
        concat!("Size of: ", stringify!(lkl_host_operations))
    );
    assert_eq!(
        ::core::mem::align_of::<lkl_host_operations>(),
        8usize,
        concat!("Alignment of ", stringify!(lkl_host_operations))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).virtio_devices as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(virtio_devices)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).print as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(print)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).panic as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(panic)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).sem_alloc as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(sem_alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).sem_free as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(sem_free)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).sem_up as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(sem_up)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).sem_down as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(sem_down)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).mutex_alloc as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(mutex_alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).mutex_free as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(mutex_free)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).mutex_lock as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(mutex_lock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).mutex_unlock as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(mutex_unlock)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).thread_create as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(thread_create)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).thread_detach as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(thread_detach)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).thread_exit as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(thread_exit)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).thread_join as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(thread_join)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).thread_self as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(thread_self)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).thread_equal as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(thread_equal)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).tls_alloc as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(tls_alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).tls_free as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(tls_free)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).tls_set as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(tls_set)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).tls_get as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(tls_get)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).mem_alloc as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(mem_alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).mem_free as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(mem_free)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).time as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).timer_alloc as *const _ as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(timer_alloc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).timer_set_oneshot as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(timer_set_oneshot)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).timer_free as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(timer_free)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).ioremap as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(ioremap)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).iomem_access as *const _ as usize
        },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(iomem_access)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lkl_host_operations>())).gettid as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(gettid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).jmp_buf_set as *const _ as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(jmp_buf_set)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<lkl_host_operations>())).jmp_buf_longjmp as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(lkl_host_operations),
            "::",
            stringify!(jmp_buf_longjmp)
        )
    );
}
extern "C" {
    #[doc = " lkl_start_kernel - registers the host operations and starts the kernel"]
    #[doc = ""]
    #[doc = " The function returns only after the kernel is shutdown with lkl_sys_halt."]
    #[doc = ""]
    #[doc = " @lkl_ops - pointer to host operations"]
    #[doc = " @cmd_line - format for command line string that is going to be used to"]
    #[doc = " generate the Linux kernel command line"]
    pub fn lkl_start_kernel(
        lkl_ops: *mut lkl_host_operations,
        cmd_line: *const c_char,
        ...
    ) -> c_int;
}
extern "C" {
    #[doc = " lkl_is_running - returns 1 if the kernel is currently running"]
    pub fn lkl_is_running() -> c_int;
}
extern "C" {
    pub fn lkl_printf(arg1: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn lkl_bug(arg1: *const c_char, ...);
}