use std::os::raw::c_void;

#[no_mangle]
pub extern fn describe_code_address(
    image: *const c_void,
    pc: *const c_void,
) {
    println!("describe_code_address({:?}, {:?})", image, pc);
}
