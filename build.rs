use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    let srcs=["source.c","src.c"];
    let lib_name="myffi";
    for i in 0..srcs.len(){
        let src=Path::new(srcs[i]).file_stem().unwrap().to_str().unwrap();
        Command::new("gcc").args(&[&format!("src/ffi/{}.c",src),"-c", "-fPIC", "-o"])
                        .arg(&format!("{}/{}.o", out_dir,src))
                        .status().unwrap();
        Command::new("ar").args(&["crus", &format!("lib{}.a",lib_name), &format!("{}.o",src)])
                        .current_dir(&Path::new(&out_dir))
                        .status().unwrap();
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}",lib_name);
}