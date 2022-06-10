use core::panic::PanicInfo;

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &'_ PanicInfo<'_>) -> ! {
    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    warn!(
        "Panic: {}\
            \n       {}:{}:{}\n",
        info.message().unwrap_or(&format_args!("explicit panic")),
        location,
        line,
        column
    );
    loop {}
}

#[alloc_error_handler]
fn oom(layout: core::alloc::Layout) -> ! {
    panic!("Could not allocate {:?}", layout);
}

#[no_mangle]
extern "C" fn memcpy(dest: *mut (), source: *const (), len: usize) -> *mut () {
    let dest = dest.cast::<u8>();
    let source = source.cast::<u8>();
    for offset in 0..len as isize {
        unsafe {
            core::ptr::write(dest.offset(offset), core::ptr::read(source.offset(offset)));
        }
    }
    dest.cast()
}

#[no_mangle]
extern "C" fn memcmp(ptr1: *const (), ptr2: *const (), len: usize) -> isize {
    let ptr1 = ptr1.cast::<u8>();
    let ptr2 = ptr2.cast::<u8>();
    for offset in 0..len as isize {
        let val1 = unsafe { core::ptr::read(ptr1.offset(offset)) };
        let val2 = unsafe { core::ptr::read(ptr2.offset(offset)) };
        if val1 != val2 {
            return offset;
        }
    }
    0
}

#[no_mangle]
extern "C" fn memset(ptr: *mut (), value: i32, len: usize) -> *mut () {
    let ptr = ptr.cast::<u8>();
    for offset in 0..len as isize {
        unsafe { core::ptr::write(ptr.offset(offset), value as u8) };
    }
    ptr.cast()
}

#[no_mangle]
extern "C" fn memmove(dest: *mut (), source: *const (), len: usize) -> *mut () {
    memcpy(dest, source, len)
}
