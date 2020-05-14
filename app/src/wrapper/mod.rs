// dlsym.rs is taken from mio
// https://github.com/carllerche/mio/blob/master/src/sys/unix/dlsym.rs

use std::marker;
use std::mem;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::ffi::CStr;

use libc;

static USE_LIBC: AtomicBool = AtomicBool::new(true);
static WRAPPING: AtomicBool = AtomicBool::new(false);

pub fn switch_libc(switch: bool) {
    USE_LIBC.store(switch, Ordering::SeqCst);
}

pub fn with_libc() -> bool {
    USE_LIBC.load(Ordering::SeqCst)
}

pub fn switch_wrapping(switch: bool) {
    WRAPPING.store(switch, Ordering::SeqCst);
}

pub fn with_wrapping() -> bool {
    WRAPPING.load(Ordering::SeqCst)
}

extern "C" {
    fn lkl_syscall(no: libc::c_long, params: *const libc::c_long) -> libc::c_long;
    fn lkl_strerror(no: libc::c_int) -> *const libc::c_char;
}

pub fn lkl_call(nr: libc::c_long, params: &[libc::c_long]) -> libc::c_long {
    let mut array: [libc::c_long; 6] = [0; 6];
    array[0..params.len()].copy_from_slice(params);

    let err = unsafe {lkl_syscall(nr, array.as_ptr())};
    if err >= 0 {
        return err;
    }
    let e_str = unsafe {_lkl_strerror(err as libc::c_int)};
    eprintln!("err: {} with params {:?}", e_str, params);
    -1
}

unsafe fn _lkl_strerror(errnum: libc::c_int) -> String {
    let cs = lkl_strerror(errnum);

    return CStr::from_ptr(cs).to_str().unwrap().to_string();
}

macro_rules! wrapped {
    (
        $x:expr;
    ) => (
        wrapper::switch_wrapping(true);
        $x;
        wrapper::switch_wrapping(false);
    )
}

macro_rules! wrap_syscall {
    (
        fn $name:ident($($n:ident: $t:ty),*) -> $ret:ty
    ) => (
        wrap_syscall_with! {
            fn $name($($n: $t),*) -> $ret;
            $name
        }
    )
}

macro_rules! wrap_syscall_with {
    (
        fn $name:ident($($n:ident: $t:ty),*) -> $ret:ty;
        $nr:ident
    ) => (
        paste::item! {
            #[allow(bad_style)]
            static [<$name _dlfn>]: wrapper::DlSym<unsafe extern fn($($t),*) -> $ret> =
                wrapper::DlSym {
                    name: concat!(stringify!($name), "\0"),
                    addr: ::std::sync::atomic::AtomicUsize::new(0),
                    _marker: ::std::marker::PhantomData,
                }; 
        }
    #[no_mangle]
    unsafe extern "C" fn $name($($n: $t),*) -> $ret {
        let l = wrapper::with_libc();
        let w = wrapper::with_wrapping();

        if w && !l {
            // eprintln!("lkl call to {}", stringify!($name));
            paste::expr! {
                return wrapper::lkl_call([<__NR_ $nr>] as libc::c_long, &[$($n as libc::c_long),*][..]) as $ret;
            }
        }
        
        paste::expr! {
            return [<$name _dlfn>].get().unwrap()($($n),*);
        }
    }
    )
}

pub struct DlSym<F> {
    pub name: &'static str,
    pub addr: AtomicUsize,
    pub _marker: marker::PhantomData<F>,
}

impl<F> DlSym<F> {
    pub fn get(&self) -> Option<&F> {
        assert_eq!(mem::size_of::<F>(), mem::size_of::<usize>());
        unsafe {
            if self.addr.load(Ordering::SeqCst) == 0 {
                self.addr.store(fetch(self.name), Ordering::SeqCst);
            }
            if self.addr.load(Ordering::SeqCst) == 1 {
                None
            } else {
                mem::transmute::<&AtomicUsize, Option<&F>>(&self.addr)
            }
        }
    }
}

unsafe fn fetch(name: &str) -> usize {
    assert_eq!(name.as_bytes()[name.len() - 1], 0);
    match libc::dlsym(libc::RTLD_NEXT, name.as_ptr() as *const _) as usize {
        0 => 1,
        n => n,
    }
}