use cbindgen;

fn main() {
    cbindgen::Builder::new()
    .with_language(cbindgen::Language::C)
      .with_crate(".")
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}