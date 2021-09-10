use std::path::Path;

fn main() {
  let include_path = Path::new("../mipsel-sony-psp/psp/include");
  let lib_path = Path::new("../mipsel-sony-psp/psp/lib");

  cc::Build::new()
    .file("src/demo.c")
    .include(include_path)
    .compile("demo");

  println!("cargo:rerun-if-changed=src/test.c");
  println!("cargo:rustc-link-lib=static=c");
  println!("cargo:rustc-link-lib=static=m");
  println!("cargo:rustc-link-lib=static=pthread-psp");
  println!("cargo:rustc-link-search=native={}", lib_path.display());
}
