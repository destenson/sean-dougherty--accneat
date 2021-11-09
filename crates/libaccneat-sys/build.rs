extern crate cmake;

use std::fs;

use cmake::Config;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {

    // let src = &"../../vendor/accneat/src/".to_string();
    // cc::Build::new()
    //     .cpp(true)
    //     .cpp_link_stdlib("stdc++")
    //     .include(src)
    //     .include(src + "experiments")
    //     .include(src + "experiments/maze")
    //     .include(src + "experiments/static")
    //     .include(src + "innovgenome")
    //     .include(src + "network")
    //     .include(src + "network/cpu")
    //     .include(src + "network/cuda")
    //     .include(src + "species")
    //     .include(src + "util")
    //     .file(src + "experiments/maze/maze.cpp")
    //     .file(src + "experiments/maze/mazeevaluator.cxx")
    //     .file(src + "experiments/maze/mazeevaluator.h")
    //     .file(src + "experiments/static/cfg.cpp")
    //     .file(src + "experiments/static/regex.cpp")
    //     .file(src + "experiments/static/sequence.cpp")
    //     .file(src + "experiments/static/staticevaluator.cxx")
    //     .file(src + "experiments/static/staticevaluator.h")
    //     .file(src + "experiments/static/staticexperiment.h")
    //     .file(src + "experiments/static/xor.cpp")
    //     .file(src + "experiments/evaluatorexperiment.h")
    //     .file(src + "experiments/experiment.cpp")
    //     .file(src + "experiments/experiment.h")
    //     .file(src + "innovgenome/innovation.cpp")
    //     .file(src + "innovgenome/innovation.h")
    //     .file(src + "innovgenome/innovgenome.cpp")
    //     .file(src + "innovgenome/innovgenome.h")
    //     .file(src + "innovgenome/innovgenomemanager.cpp")
    //     .file(src + "innovgenome/innovgenomemanager.h")
    //     .file(src + "innovgenome/innovlinkgene.cpp")
    //     .file(src + "innovgenome/innovlinkgene.h")
    //     .file(src + "innovgenome/innovnodegene.cpp")
    //     .file(src + "innovgenome/innovnodegene.h")
    //     .file(src + "innovgenome/innovnodelookup.h")
    //     .file(src + "innovgenome/protoinnovlinkgene.h")
    //     .file(src + "innovgenome/recurrencychecker.h")
    //     .file(src + "innovgenome/trait.cpp")
    //     .file(src + "innovgenome/trait.h")
    // // src/network/cpu/cpunetwork.cpp
    // // src/network/cpu/cpunetwork.h
    // // src/network/cpu/cpunetworkexecutor.h
    // // src/network/cuda/cudanetwork.cu
    // // src/network/cuda/cudanetwork.h
    // // src/network/cuda/cudanetworkbatch.h
    // // src/network/cuda/cudanetworkexecutor.h
    // // src/network/cuda/cudanetworkkernel.h
    // // src/network/cuda/cudautil.h
    // // src/network/network.h
    // // src/network/networkexecutor.h
    // // src/species/species.cpp
    // // src/species/species.h
    // // src/species/speciesorganism.cpp
    // // src/species/speciesorganism.h
    // // src/species/speciespopulation.cpp
    // // src/species/speciespopulation.h
    // // src/util/map.cpp
    // // src/util/map.h
    // // src/util/organismsbuffer.h
    // // src/util/resource.cpp
    // // src/util/resource.h
    // // src/util/rng.cpp
    // // src/util/rng.h
    // // src/util/stats.h
    // // src/util/std.h
    // // src/util/std.hxx
    // // src/util/timer.cpp
    // // src/util/timer.h
    // // src/util/util.cpp
    // // src/util/util.h
    // // src/genome.h
    // // src/genomemanager.cpp
    // // src/genomemanager.h
    // // src/main.cpp
    // // src/neat.cpp
    // // src/neat.h
    // // src/neattypes.h
    // // src/organism.cpp
    // // src/organism.h
    // // src/population.cpp
    //     .compile("libaccneat.a");

    println!("cargo:rerun-if-env-changed=TARGET");
    println!(
        "cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock")
    );

    let dst = Config::new(".")
        .uses_cxx11()
        .no_build_target(true)
        .build();

    println!("cargo:rustc-link-search=native={}\\build\\Debug", dst.display());
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static={}", link_lib_base());

    dotenv::dotenv().ok();

    // Command::new("make")
    //     .args(&["-f", "../../Makefile"])
    //     .output()
    //     .expect("failed to run make");

    build::main();
}

fn link_lib_base() -> &'static str {
    "accneatlib"
}

mod bindings {
    extern crate bindgen;

    use std::env;
    use std::path::{Path, PathBuf};

    pub fn place_bindings(inc_dir: &Path) {
        println!("debug:Using bindgen for accneat");
        let cver = bindgen::clang_version();
        println!("debug:clang version: {}", cver.full);

        let inc_search = [
            format!("-I{}", inc_dir.display()),
            format!("-I{}/experiments", inc_dir.display()),
            format!("-I{}/experiments/maze", inc_dir.display()),
            format!("-I{}/experiments/static", inc_dir.display()),
            format!("-I{}/innovgenome", inc_dir.display()),
            format!("-I{}/network", inc_dir.display()),
            format!("-I{}/network/cpu", inc_dir.display()),
            format!("-I{}/network/cuda", inc_dir.display()),
            format!("-I{}/species", inc_dir.display()),
            format!("-I{}/util", inc_dir.display()),
        ];
        for p in &inc_search {
            println!("debug:bindgen include path: {}", p);
        }

        let src = &"-I../../src/".to_string();
        // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        let bindings = bindgen::Builder::default()
            .conservative_inline_namespaces()
            .trust_clang_mangling(false)
            .enable_cxx_namespaces()
            .enable_function_attribute_detection()
            .generate_block(true)
            .generate_inline_functions(true)
            .derive_default(true)
            .impl_debug(true)
            .impl_partialeq(true)
            .derive_eq(true)
            .derive_hash(true)
            .derive_ord(true)
            .header("wrapper.hpp")
            .clang_arg(src.to_owned())
            .clang_arg(src.to_owned() + "experiments")
            .clang_arg(src.to_owned() + "experiments/maze")
            .clang_arg(src.to_owned() + "experiments/static")
            .clang_arg(src.to_owned() + "innovgenome")
            .clang_arg(src.to_owned() + "network")
            .clang_arg(src.to_owned() + "network/cpu")
            .clang_arg(src.to_owned() + "network/cuda")
            .clang_arg(src.to_owned() + "species")
            .clang_arg(src.to_owned() + "util")
            .clang_arg(format!("-L{}/build/Debug", env::var("OUT_DIR").unwrap()))
            .clang_arg(format!("-L{}/build", env::var("OUT_DIR").unwrap()))
            // .clang_arg(format!("-L{}/build/Release", env::var("OUT_DIR").unwrap()))
            .clang_arg("-fopenmp")
            // .clang_arg("-lgomp")
            .clang_arg("-MMD")
            .clang_arg("-Wall")
            .clang_arg("-Werror")
            .clang_arg("-std=c++17")
            .allowlist_type("root::std::default_random_engine")
            .allowlist_type("NEAT::.*")
            // .opaque_type("root::std::.*")
            // .opaque_type("::std::.*")
            // .opaque_type("std::.*")
            // .blocklist_type("root::std::.*")
            // .blocklist_type("std::.*")
            .generate()
            .expect("Unable to generate bindings");

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let out_path = out_dir.join("bindings.rs");

        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}

mod build {
    use super::*;
    use std::env;
    use std::path::Path;

    fn find_include_dir() -> Option<String> {
        let link_lib = link_lib_base();

        // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        println!("cargo:rerun-if-env-changed=ACCNEAT_INC_DIR");
        println!("cargo:rerun-if-env-changed=ACCNEAT_LIB_DIR");

        println!("debug:Building with existing library: {}", link_lib);
        println!("cargo:rustc-link-lib={}", link_lib);

        if let Ok(lib_dir) = env::var("ACCNEAT_LIB_DIR") {
            if let Ok(inc_dir) = env::var("ACCNEAT_INC_DIR") {
                println!("debug:inc_dir={}", inc_dir);
                println!("debug:lib_dir={}", lib_dir);

                println!("cargo:rustc-link-search={}", lib_dir);
                Some(inc_dir)
            } else {
                panic!("If specifying lib dir, must also specify inc dir");
            }
        } else if let Ok(pwd) = env::var("PWD") {
            let inc_dir = pwd + "/../../src";
            println!("debug:inc_dir={}/", &inc_dir);
            // println!("debug:lib_dir={}/", &inc_dir);
            Some(inc_dir)
        } else {
            let inc_dir = "../../src".to_string();
            println!("debug:inc_dir={}/", &inc_dir);
            println!("debug:inc_dir={}/experiments", inc_dir);
            println!("debug:inc_dir={}/experiments/maze", inc_dir);
            println!("debug:inc_dir={}/experiments/static", inc_dir);
            println!("debug:inc_dir={}/innovgenome", inc_dir);
            println!("debug:inc_dir={}/network", inc_dir);
            println!("debug:inc_dir={}/network/cpu", inc_dir);
            println!("debug:inc_dir={}/network/cuda", inc_dir);
            println!("debug:inc_dir={}/species", inc_dir);
            println!("debug:inc_dir={}/util", inc_dir);
            // println!("debug:lib_dir={}/", &inc_dir);

            // println!("cargo:rustc-link-search={}", out_dir.join("/build/Debug").to_str().unwrap());
            // println!("cargo:rustc-link-search=native={}", out_dir.join("/build/Debug").to_str().unwrap());
            // println!("cargo:rustc-link-search={}", out_dir.join("/build/Release").to_str().unwrap());
            // println!("cargo:rustc-link-search=native={}", out_dir.join("/build/Release").to_str().unwrap());
            Some(inc_dir)
        }
    }

    pub fn main() {
        println!("debug:Running the build for accneat");

        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=CMakeLists.txt");
        println!("cargo:rerun-if-changed=wrapper.hpp");
        for src_file in get_src_dependencies_from_cmakelists() {
            println!("cargo:rerun-if-changed={}", src_file);
        }

        let inc_dir = find_include_dir().unwrap_or_default();
        if inc_dir.is_empty() {
            panic!("Can't generate bindings. Unknown library location");
        }

        bindings::place_bindings(Path::new(&inc_dir));
    }
}


fn get_src_dependencies_from_cmakelists() -> Vec<String> {

    // read CMakeLists.txt file
    // return all the strings that start with "../../src" and end with .cxx, .h, .hxx

    let contents = fs::read_to_string("CMakeLists.txt").expect("Something went wrong reading CMakeLists.txt");

    println!("found contents: {}", &contents);
    lazy_static! {
        // TODO: FIX THIS
        static ref RE: Regex = Regex::new(r"^ *\.\./\.\./src/.*\.[ch].*$").unwrap();
    }
    // contents
    RE.find_iter(contents.as_str())
        .filter_map(|x| {
            let m =x.as_str().to_string();
            println!("Found Match! {}", &m);
            Some(m)
        })
        .collect()


    // todo!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_deps() {
        assert_eq!(0,1);

    }
}
