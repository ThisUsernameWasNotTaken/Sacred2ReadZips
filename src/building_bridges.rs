#[allow(unsafe_code)]
use cxx;
use std;

#[cxx::bridge]
mod ffi {
    // include!("../GrannyConverterLibrary/examples/converter/main.h");
    unsafe extern "C++" {
        fn extractFbx(baseFilepath: &String, list: &Vec<String>) -> usize;
    }
}