mod benchtool;

use std::os::raw::c_char;

#[no_mangle]
pub extern fn benchtool_rust_call(iteration: i32, repeat: i32, bench_type: i32) -> *const c_char{
    benchtool::executor::benchtool_rust_execute(iteration, repeat, bench_type).as_ptr() as *const c_char
}