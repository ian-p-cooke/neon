extern crate cc;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;

/// Set up the build environment by setting Cargo configuration variables.
pub fn setup() {
    if cfg!(windows) {
        let debug = env::var("DEBUG").ok().map_or(false, |s| s == "true");
        let configuration = if debug { "Debug" } else { "Release" };
        let node_root_dir = env::var("DEP_NEON_RUNTIME_NODE_ROOT_DIR").unwrap();
        let node_lib_file = env::var("DEP_NEON_RUNTIME_NODE_LIB_FILE").unwrap();
        let node_arch = env::var("DEP_NEON_RUNTIME_NODE_ARCH").unwrap();
        let node_lib_file_path = Path::new(&node_lib_file);
        let mut node_lib_path = Path::new(&node_root_dir).to_path_buf();
        node_lib_path.push(&node_arch);
        println!("cargo:rustc-link-search={}\\{}", node_root_dir, configuration);
        println!("cargo:rustc-link-search=native={}", &node_lib_path.display());
        println!("cargo:rustc-link-lib={}", &node_lib_file_path.file_stem().unwrap().to_str().unwrap());

        //for win_delay_load_hook.cc
        let win_delay_load_hook_cc = include_str!("win_delay_load_hook.cc");
        let out_dir = env::var("OUT_DIR").unwrap();
        let out_dir_path = Path::new(&out_dir);
        let path = out_dir_path.join("win_delay_load_hook.cc");
        {
            let mut file = File::create(&path).unwrap();
            file.write_all(win_delay_load_hook_cc.as_bytes()).unwrap();
        }        
        cc::Build::new().define("HOST_BINARY", "\"node.exe\"").file(&path).compile("win_delay_load_hook");
        //println!("cargo:rustc-link-lib=win_delay_load_hook");        
        println!("cargo:rustc-cdylib-link-arg=/DELAYLOAD:node.exe");
        //println!("cargo:rustc-cdylib-link-arg=win_delay_load_hook.o");
        println!("cargo:rustc-link-lib=delayimp");        
    }
}
