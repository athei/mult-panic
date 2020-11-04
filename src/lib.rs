#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn multer(a: i128, b: i128) -> i128 {
    // trigger usage of the __multi3 compiler intrinsic which seems to be necessary
    // in order to reproduce the bug.
    a * b
}
