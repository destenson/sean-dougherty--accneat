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
pub use root::NEAT::*;


pub fn get_env() -> Option<&'static mut NeatEnv> {
    unsafe {
        if env.is_null() {
            unreachable!();
        } else {
            env.as_mut()
        }
    }
}

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
// pub mod root {

    // pub mod NEAT {
    //     #[allow(unused_imports)]
    //     use crate::NEAT::NeatEnv;

        extern "C" {
            #[link_name = "NEAT::env"]
            pub static mut env: *mut NeatEnv;
        }
    // }
// }

// struct Experiment {
//
// }

// fn get_xor_exp<'a>() -> Option<&'a mut Experiment> {
//     let s: Vec<i8> = "xor".as_bytes().iter().map(|&x| x as i8).collect();
//     let e: *mut root::NEAT::Experiment = unsafe { Experiment::get(s.as_ptr()) };
//     unsafe { e.as_mut() }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_env_initialized() {
        println!("{:?}", get_env());
    }

    // #[test]
    // fn test_get_xor_exp() {
    //     let exp = get_xor_exp();
    //     assert!(exp.is_some());
    //     let name: Vec<i8> = "xor".as_bytes().iter().map(|&x| x as i8).collect();
    //     assert!(exp.unwrap().name == name.as_ptr());
    // }
}