use std::alloc::{GlobalAlloc, Layout, System};
#[cfg(feature = "track-allocation")]
use std::backtrace::Backtrace;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAllocator;

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            ALLOC_COUNT.fetch_sub(1, Ordering::SeqCst);
            System.dealloc(ptr, layout)
        }
    }
}

#[cfg(feature = "track-allocation")]
#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

#[cfg(not(feature = "track-allocation"))]
#[global_allocator]
static GLOBAL: System = System;

#[cfg(feature = "track-allocation")]
pub fn allocation_count() -> usize {
    ALLOC_COUNT.load(Ordering::SeqCst)
}

#[cfg(feature = "track-allocation")]
#[macro_export]
macro_rules! allocation_counting {
    ($body:expr, $ident:ident) => {{
        let bt = std::backtrace::Backtrace::force_capture();
        let as_text = format!("{}", bt);
        let shorter_stack = as_text.lines().take(4).collect::<Vec<_>>().join("\n");
        println!(
            "Count before {} for {}   at:\n{}",
            crate::allocator::allocation_count(),
            $ident,
            shorter_stack
        );
        let result = $body;
        println!(
            "Count after {} for {} at:\n{}",
            crate::allocator::allocation_count(),
            $ident,
            shorter_stack
        );
        result
    }};
}

#[cfg(not(feature = "track-allocation"))]
#[macro_export]
macro_rules! allocation_counting {
    ($e:expr,$ident:ident) => {{ $e }};
}
