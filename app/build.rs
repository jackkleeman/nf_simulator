fn main() {
    println!("cargo:rustc-link-lib=static=lkl");
    println!("cargo:rustc-link-lib=static=iptables");
    println!("cargo:rustc-link-lib=static=ext");
    println!("cargo:rustc-link-lib=static=ext4");
}