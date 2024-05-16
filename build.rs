use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use pkg_config;
fn main() {

  let out = &PathBuf::from(env::var("OUT_DIR").unwrap());
  println!("cargo:rustc-link-search={}", out.display());
  println!("cargo:rustc-link-lib=opus");

  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings");
  
}