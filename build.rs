use std::{env, path::PathBuf, process::Command};

fn main() {
  // Specify the directory containing the C files
  let c_dir = "extern/cjson";

  // Collect all C files in the directory
  let mut c_files = vec![
    "extern/cjson/cJSON.c",
    "extern/client.c",
    "extern/env.c",
    "extern/error.c",
    "extern/gpu.c",
    "extern/execute.c",
    "extern/info.c",
    "extern/jobs.c",
    "extern/list.c",
    "extern/mail.c",
    "extern/main.c",
    "extern/msg.c",
    "extern/msgdump.c",
    "extern/print.c",
    "extern/server.c",
    "extern/server_start.c",
    "extern/signals.c",
    "extern/tail.c",
  ];
  let mut object_files = Vec::new();
  for c_file in &c_files {
    let output = Command::new("gcc")
      //.arg("-pedantic")
      //.arg("-Wall")
      //.arg("-g")
      //.arg("-O2")
      //.arg("-std=c11")
      .args(&["-c", c_file, "-o"])
      .arg(c_file.replace(".c", ".o"))
      .status()
      .expect("Failed to compile C file");

    if !output.success() {
      panic!("C compilation failed for {}", c_file);
    }

    // Store the object file path
    object_files.push(c_file.replace(".c", ".o"));
  }

  // Create a static library from the object files
  let output_lib = PathBuf::from(c_dir).join("libextern.a");
  let output = Command::new("ar")
    .args(&["crs", output_lib.to_str().unwrap()])
    .args(object_files.iter().map(|p| p))
    .status()
    .expect("Failed to create static library");

  if !output.success() {
    panic!("Failed to create static library");
  }

  // Link the compiled library
  println!("cargo:rustc-link-lib=static=extern");
  println!("cargo:rustc-link-search=native={}", c_dir);
}
