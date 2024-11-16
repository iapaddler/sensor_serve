fn main() {
    println!("cargo:rustc-link-search=../rserve/bmp388/.");
    println!("cargo:rustc-link-lib=static=rsd");
}
