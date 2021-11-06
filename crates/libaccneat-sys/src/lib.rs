#![allow(clippy::redundant_static_lifetimes)]
#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clashing_extern_declarations)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use root::NEAT;
pub use root::GenomeManager;
pub use root::NEAT::*;


pub fn get_env() -> NeatEnv {
    root::NEAT::NeatEnv::default()
}

