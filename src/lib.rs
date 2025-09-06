mod benchtool;

use std::os::raw::c_char;
use std::ffi::CString;

#[unsafe(no_mangle)]
pub extern "C" fn benchtool_rust_call(iteration: i32, repeat: i32, bench_type: i32) -> *const c_char{
    let result = CString::new( benchtool::executor::benchtool_rust_execute(iteration, repeat, bench_type));
    
    match result {
        Ok(value) => return value.into_raw(),
        Err(_error) => return CString::from(c"ERROR").into_raw(),
    }
}