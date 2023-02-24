use std::{env, path::PathBuf};

fn main() {
    // Tell cargo to rerun this build script whenever any file in the `lib/` directory changes
    for entry in glob::glob("lib/**/*").unwrap() {
        if let Ok(path) = entry {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Tell cargo to rerun this build script whenever the wrapper header changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("lib/info/macos/cpu.c")
        .compile("cpu");

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("lib/info/macos/caller.c")
        .compile("caller");

    let bindings = bindgen::Builder::default()
        .header("lib/wrapper.h")
        .clang_arg("c")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
