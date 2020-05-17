fn main() {
  cc::Build::new()
    .cpp(true)
    .include("/usr/X11/include")
    .flag("-std=c++17")
    .flag("-Wno-delete-non-virtual-dtor") // warnings from the olcPixelGameEngine, need to be fixed upstream
    .file("src/olcRustBindingApp.cpp")
    .warnings(false)
    .compile("olcRustBindingApp");

  println!("cargo:rustc-link-search={}", "/usr/X11/lib");
  println!("cargo:rustc-link-lib=X11");
  println!("cargo:rustc-link-lib=GL");
  println!("cargo:rustc-link-lib=png");
  println!("cargo:rustc-link-lib=pthread");
  println!("cargo:rustc-link-lib=olcRustBindingApp");
}

// g++ -o olcExampleProgram olcExampleProgram.cpp -I/usr/X11/include -L/usr/X11/lib -lX11 -lGL -lpng -lpthread -std=c++17
