fn main() {
    println!("cargo:rustc-link-search=./library/");
    println!("cargo:rustc-link-lib=static=saw");
}
