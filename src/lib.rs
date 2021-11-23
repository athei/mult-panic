#![no_std]

extern "C" {
    // used to inhibit optimization
    fn black_box(ptr: *const BigType);
}

// Decrease to <= 32 bit and the bug goes away because no memcpy is inserted
type BigType = [u8; 256];

#[no_mangle]
pub fn main(big: BigType) {
    copy(big)
}

// Just making sure that the compiler calls memcpy to pass the value
#[inline(never)]
fn copy(big: BigType) {
    unsafe {
        black_box(&big as *const BigType)
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
