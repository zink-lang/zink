//! Addition example.
#![no_std]

/// Adds two numbers together.
#[no_mangle]
pub extern "C" fn add(x: u64, y: u64) -> u64 {
    x + y
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
