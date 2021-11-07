#![allow(clippy::redundant_static_lifetimes)]
#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clashing_extern_declarations)]

use std::ffi::c_void;
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// pub use root::NEAT::create_static_evaluator;
// namespace NEAT {
//     NetworkEvaluator *create_static_evaluator(const vector<Test> &tests) {
//         return new StaticNetworkEvaluator(tests);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_static_evaluator() {
        // let network_evaluator = create_static_evaluator()
        // println!("{:?}", get_env());
    }

}

pub trait NetworkEvaluator {

}


// #[repr(C)]
// {
//
// }
