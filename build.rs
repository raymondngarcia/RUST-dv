fn main() {
    println!("cargo:rustc-link-search=native=obj_dir");
    println!("cargo:rustc-link-lib=static=vltick");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
