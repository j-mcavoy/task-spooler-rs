#![feature(never_type)]
mod cli;
mod ffi;
mod process;

fn main() {
  env_logger::init();
  cli::cli()
}
