#![no_std]
#![feature(lang_items)]

extern crate alloc;

#[cfg(feature = "provide_oom")]
extern crate panic_halt;

// Implement an allocator with libc malloc and free
// Import malloc and free from libc:
#[link(name = "c")]
extern "C" {
    pub fn malloc( new_size: usize ) -> *mut u8;
    pub fn free( ptr: *mut u8 );
}

// Define allocator that just calls down to libc malloc/free
struct LibCAllocator;

unsafe impl alloc::alloc::GlobalAlloc for LibCAllocator {
    unsafe fn alloc(&self, _layout: alloc::alloc::Layout) -> *mut u8 {
        malloc( _layout.size() )
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: alloc::alloc::Layout)
    {
        free( _ptr );
    }
}

// Assign our wrapper as the global allocator
#[global_allocator]
static A: LibCAllocator = LibCAllocator;

// Optionally, define how Out Of Memory (OOM) conditions should be handled, *if* no other crate has
// already defined `oom`.
// This is currently specific to Cortex-M.
#[cfg(feature = "provide_oom")]
#[lang = "oom"]
#[no_mangle]
pub fn rust_oom( _layout : alloc::alloc::Layout ) -> ! {
    // Set a breakpoint to catch the debugger
    cortex_m::asm::bkpt();

    loop {};
}
