use std::env;
use std::fs;
use std::path::Path;

fn main() {
  // OUT_DIR is set by Cargo during a build.
  let out_dir = env::var("OUT_DIR").unwrap();
  println!("OUT_DIR = {}", out_dir);

  // Root folder where we will build pixel game engine.
  let root = Path::new(&out_dir);

  // Copy C++ binding files into OUT_DIR to build a library.
  fs::copy("cpp/olcPixelGameEngine.h", root.join("olcPixelGameEngine.h")).unwrap();
  fs::copy("cpp/olcRustBindingApp.h", root.join("olcRustBindingApp.h")).unwrap();
  fs::copy("cpp/olcRustBindingApp.cpp", root.join("olcRustBindingApp.cpp")).unwrap();

  // Build Rust binding together with olcPixelGameEngine.h.
  build_rust_binding(root);
}

#[cfg(target_os = "macos")]
fn build_rust_binding(root: &Path) {
  cc::Build::new()
    .cpp(true)
    .include("/usr/X11/include")
    .flag("-std=c++17")
    .flag("-Wno-delete-non-virtual-dtor") // warnings from the olcPixelGameEngine, need to be fixed upstream
    .file(root.join("olcRustBindingApp.cpp"))
    .warnings(false)
    .compile("olcRustBindingApp");

  println!("cargo:rustc-link-search={}", "/usr/X11/lib");
  println!("cargo:rustc-link-lib=X11");
  println!("cargo:rustc-link-lib=GL");
  println!("cargo:rustc-link-lib=png");
  println!("cargo:rustc-link-lib=pthread");
}

#[cfg(target_os = "linux")]
fn build_rust_binding(root: &Path) {
  cc::Build::new()
    .cpp(true)
    .file(root.join("olcRustBindingApp.cpp"))
    .warnings(false)
    .compile("olcRustBindingApp");

  println!("cargo:rustc-link-lib=X11");
  println!("cargo:rustc-link-lib=GL");
  println!("cargo:rustc-link-lib=png");
  println!("cargo:rustc-link-lib=pthread");
  println!("cargo:rustc-link-lib=stdc++fs");
}

// macos:
// g++ -o olcExampleProgram olcExampleProgram.cpp -I/usr/X11/include -L/usr/X11/lib -lX11 -lGL -lpng -lpthread -std=c++17
//
// linux:
// g++ -o olcExampleProgram olcExampleProgram.cpp -lX11 -lGL -lpthread -lpng -lstdc++fs -std=c++17
