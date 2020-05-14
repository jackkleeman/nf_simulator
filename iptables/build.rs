use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use pkg_config;

//use bindgen;
use num_cpus;

fn artefacts_built(build_dir: &Path) -> bool {
   build_dir.join("libiptables.a").exists()
}

fn get_env(var: &'static str) -> Option<PathBuf> {
    println!("cargo:rerun-if-env-changed={}", var);
    env::var_os(var).map(PathBuf::from)
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = PathBuf::from(out_dir.clone());

    println!("OUT_DIR {:?}", out_dir);
    let libs_built = artefacts_built(out_dir_path.as_path());

    if !libs_built {
        println!("MKDIR {:?}", out_dir);
        Command::new(format!("mkdir",))
            .args(&["-p", out_dir.as_str()])
            .status()
            .unwrap();

        println!("CLONE {:?}", out_dir);
        let options = vec![
            "clone",
            "--branch",
            "v1.6.2",
            "--depth",
            "1",
            "git://git.netfilter.org/iptables",
            out_dir.as_str(),
        ];
        Command::new("git")
            .args(options.as_slice())
            .status()
            .unwrap();

        println!("BUILD {:?}", out_dir);
        Command::new("./autogen.sh")
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();

        let options = vec!["--enable-static", "--disable-shared"];
        // let options = vec!["--enable-static", "--disable-shared", "CFLAGS=-DDEBUG -ggdb3 -O0"];
        Command::new("./configure")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();


        let cpus = format!("{}", num_cpus::get());
        let options = vec!["-j", cpus.as_str()];

        Command::new("make")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();

        // I'm sorry.   
        Command::new("make")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();

        let options = vec!["rcs", "libiptables.a", 
                        "iptables/xtables_compat_multi-xtables-save.o", 
                        "iptables/xtables_compat_multi-xtables-restore.o",
                        "iptables/xtables_compat_multi-xtables-eb.o",
                        "iptables/xtables_compat_multi-nft.o",
                        "iptables/xtables_compat_multi-xtables.o",
                        "iptables/xtables_compat_multi-nft-shared.o",
                        "iptables/xtables_compat_multi-xshared.o",
                        "iptables/xtables_compat_multi-nft-bridge.o",
                        "iptables/xtables_compat_multi-nft-ipv6.o",
                        "iptables/xtables_compat_multi-nft-arp.o",
                        "iptables/xtables_compat_multi-nft-ipv4.o",
                        "iptables/xtables_compat_multi-getethertype.o",
                        "iptables/xtables_compat_multi-xtables-config-parser.o",
                        "iptables/xtables_compat_multi-xtables-config-syntax.o",
                        "iptables/xtables_compat_multi-xtables-translate.o",
                        "libxtables/libxtables_la-xtables.o",
                        "libxtables/libxtables_la-xtoptions.o",
                        "libiptc/libip4tc.o"
                        ];
        Command::new("ar")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();

        let options = vec![
            "extensions/libext.a",
            "extensions/libext4.a",
            "extensions/libext6.a", 
            "."];
        Command::new("cp")
            .args(options.as_slice())
            .current_dir(&Path::new(&out_dir))
            .status()
            .unwrap();


        println!("OUT_DIR {:?}", out_dir);
    }
    assert!(artefacts_built(out_dir_path.as_path()));
    println!("cargo:rustc-link-search=native={}", out_dir);

    if let Some(lib_dir) = get_env("LIBNFTNL_LIB_DIR") {
        if !lib_dir.is_dir() {
            panic!(
                "libnftnl library directory does not exist: {}",
                lib_dir.display()
            );
        }
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=nftnl");
    } else {
        // Trying with pkg-config instead
        println!("Minimum libnftnl version: {}", "1.0.6");
        pkg_config::Config::new()
            .atleast_version("1.0.6")
            .probe("libnftnl")
            .unwrap();
    }

    if let Some(lib_dir) = get_env("LIBMNL_LIB_DIR") {
        if !lib_dir.is_dir() {
            panic!(
                "libmnl library directory does not exist: {}",
                lib_dir.display()
            );
        }
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=mnl");
    } else {
        // Trying with pkg-config instead
        pkg_config::Config::new()
            .atleast_version("1.0.0")
            .probe("libmnl")
            .unwrap();
    }
}
