#![no_std]

mod imports;

/// Storage interfaces
pub mod storage {
    pub use super::imports::{sload, sstore};
}

/// Event interfaces
pub mod events {
    pub use super::imports::{log0, log1, log2, log3, log4};
}

// Panic hook implementation
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
