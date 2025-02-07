use std::os::raw::c_char;

#[link(name = "extern", kind = "static")]
unsafe extern "C" {
    fn main_c(argc: i32, argv: *mut *mut c_char);
}
fn tsp_c() {
    unsafe { main_c(argc, argv.as_ptr() as *mut *mut c_char) };

}
use std::os::raw::c_char;

#[link(name = "extern", kind = "static")]
unsafe extern "C" {
    fn main_c(argc: i32, argv: *mut *mut c_char);
}
fn tsp_c() {
    unsafe { main_c(argc, argv.as_ptr() as *mut *mut c_char) };

}
