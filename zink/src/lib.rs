#![no_std]

mod imports;

/// Storage interfaces
pub mod storage {
    pub use super::imports::{sload, sstore};
}

// Panic hook implementation
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
