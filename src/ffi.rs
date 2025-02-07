use clap::Args;
use std::os::raw::c_char;

#[link(name = "extern", kind = "static")]
unsafe extern {
  fn main_c(argc: i32, argv: *mut *mut c_char);
}
pub fn tsp_c(args: Vec<&str>) {
  let argc = args.len();
  let argv = args;
  unsafe { main_c(argc as i32, argv.as_ptr() as *mut *mut c_char) };
}
