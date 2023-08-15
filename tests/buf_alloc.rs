//! SimpleAllocator code copied from the Standard Library:
//! [GlobalAlloc].
//!
//! [GlobalAlloc]: [https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html]

#![feature(allocator_api)]

use std::{
    alloc::{
        Allocator,
        GlobalAlloc,
        Layout,
    },
    cell::UnsafeCell,
    ptr::{
        null_mut,
        NonNull,
    },
    sync::atomic::{
        AtomicUsize,
        Ordering::SeqCst,
    },
};

use sn_dsp::{
    frame::St,
    Buf,
};

const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct SimpleAllocator {
    arena:     UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize, // we allocate from the top, counting down
}

// #[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena:     UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(
        &self,
        layout: Layout,
    ) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align
        // not power of 2. So we can safely use a mask to ensure
        // alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        self.arena.get().cast::<u8>().add(allocated)
    }

    unsafe fn dealloc(
        &self,
        _ptr: *mut u8,
        _layout: Layout,
    ) {
    }
}

unsafe impl Allocator for SimpleAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        let ptr = unsafe { self.alloc(layout) };
        if ptr.is_null() {
            Err(std::alloc::AllocError)
        } else {
            Ok(NonNull::slice_from_raw_parts(
                NonNull::new(ptr).unwrap(),
                layout.size(),
            ))
        }
    }

    unsafe fn deallocate(
        &self,
        _ptr: std::ptr::NonNull<u8>,
        _layout: Layout,
    ) {
    }
}

#[test]
fn buf_alloc_01() {
    let mut buf = Buf::<St, _>::alloc_new_in(4, &ALLOCATOR);
    let mut c = 0.;
    buf.iter_mut().for_each(|x| {
        x[0] = c;
        c += 1.
    });
}
