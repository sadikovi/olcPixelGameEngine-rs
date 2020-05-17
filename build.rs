use std::io;
use std::io::Write;
use std::process;

fn main() {
  // Downloads olcPixelGameEngine.h file.
  // If you have issues downloading the file, you can add the file in the project root
  // and comment out this step.
  let output = process::Command::new("curl")
    .arg("-O")
    .arg("--fail")
    .arg("https://raw.githubusercontent.com/sadikovi/olcPixelGameEngine-macos/master/olcPixelGameEngine.h")
    .output()
    .expect("Failed to execute process");

  println!("status: {}", output.status);
  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
  assert!(output.status.success());

  // Builds Rust binding together with olcPixelGameEngine.h.
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
