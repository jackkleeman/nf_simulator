/* automatically generated by rust-bindgen */

pub enum xtables_target {}

extern "C" {
    pub static mut xtables_pending_targets: *mut xtables_target;
    pub static mut xtables_targets: *mut xtables_target;
}

extern "C" {
    pub fn xtables_ip4_save_main(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn xtables_ip4_restore_main(
        arg1: ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}