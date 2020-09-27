extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Let the cc crate handle all the C library compilation and linking
    cc::Build::new()
        .file("library/saw.c")
        .include("library")
        .flag("-mcpu=cortex-m4")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-mfloat-abi=hard")
        .compile("saw");

    // Use the bindgen builder to create a binding, adding options
    let bindings = bindgen::Builder::default()
        //.raw_line("#[allow(improper_ctypes)]") // what does this do?
        .generate_comments(true)
        // Output bindings for builtin definitions, e.g. __builtin_va_list
        .emit_builtins()
        .clang_arg("-mcpu=cortex-m4")
        .clang_arg("-mthumb")
        .clang_arg("-mfloat-abi=hard")
        .ctypes_prefix("libc")
        .use_core()
        .raw_line("#[no_std]")
        .raw_line("mod libc { pub type c_uint = u32;\
        pub type c_int = i32;\
        pub type c_char = i8;\
        pub type c_uchar = u8;\
        pub type c_short = i16;\
        pub type c_ushort = u16;\
        pub type c_long = i64;\
        pub type c_ulong = u64;\
        pub type c_schar = i8;\
        pub type c_longlong = i64;\
        pub type c_ulonglong = u64;\
        pub enum c_void {}\
        }")
        // The input header we would like to generate bindings for
        .header("library/saw.h")
        // Finish the builder and generate the bindings
        .generate()
        // Unwrap the Result and panic on failure
        .expect("Unable to generate bindings!");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
