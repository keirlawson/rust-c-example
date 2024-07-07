use std::{env, path::PathBuf};

fn main() {
    cc::Build::new().file("plus.c").compile("plus");

    let bindings = bindgen::Builder::default()
        .header("plus.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
