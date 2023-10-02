//! Allocator implementation for the zink language

#![no_std]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};

pub const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;

/// Zink allocator
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
pub struct Zallocator {
    /// The arena is a fixed-size array of bytes, aligned to MAX_SUPPORTED_ALIGN.
    pub arena: UnsafeCell<[u8; ARENA_SIZE]>,
    /// Remaining bytes in the arena.
    pub remaining: AtomicUsize, // we allocate from the top, counting down
}

unsafe impl GlobalAlloc for Zallocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        let remaining = self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            });

        if remaining.is_err() {
            return null_mut();
        };

        self.arena.get().cast::<u8>().add(allocated)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

unsafe impl Sync for Zallocator {}
