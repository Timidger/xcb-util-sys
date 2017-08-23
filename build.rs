extern crate bindgen;

fn main() {
    let generated = bindgen::builder()
        .header("src/xcb_keysyms.h")
        //.header("util/src/xcb_util.h")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .clang_arg("-I")
        //.clang_arg("util/src")
        .clang_arg("src")
        .generate().unwrap();
    generated.write_to_file("src/gen.rs").unwrap();
    // TODO Static linking feature
}
