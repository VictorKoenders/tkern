mod bump;

type Inner = self::bump::BumpAllocator;

use core::alloc::Layout;

#[cfg(not(any(target_os = "linux")))]
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Could not allocate memory for {:?}", layout);
}

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator::uninit();

pub struct Allocator {
    inner: Inner,
}

impl Allocator {
    const fn uninit() -> Self {
        Self {
            inner: Inner::uninit(),
        }
    }

    unsafe fn init(&mut self, boot_info: &multiboot2::BootInformation) {
        self.inner.init(boot_info);
    }
}

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
    }
}

pub unsafe fn init(boot_info: &multiboot2::BootInformation) {
    ALLOCATOR.init(boot_info);
}
