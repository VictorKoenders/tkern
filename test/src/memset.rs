use core::ffi::c_void;
use core::ptr;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let val = 0;
    let mut dest = [0u8; 1000];
    bencher.iter(|| {
        let len = black_box(dest.len() as isize);
        let val = black_box(val);
        let dest = black_box(dest.as_mut_ptr().cast());
        unsafe {
            memset(dest, val, len);
        }
    })
}

unsafe fn memset(dest: *mut c_void, val: i32, size: isize) -> *mut c_void {
    {
        let dest: *mut u8 = dest.cast();
        for i in 0..size {
            ptr::write(dest.offset(i), val as u8);
        }
    }

    dest
}

#[bench]
fn bench_bulk(bencher: &mut Bencher) {
    let val = 0;
    let mut dest = [0u8; 1000];
    bencher.iter(|| {
        let len = black_box(dest.len() as isize);
        let val = black_box(val);
        let dest = black_box(dest.as_mut_ptr().cast());
        unsafe {
            bulk(dest, val, len);
        }
    })
}

unsafe fn bulk(ptr: *mut c_void, value: i32, num: isize) -> *mut c_void {
    let mut i = 0;
    use core::ptr;
    macro_rules! impl_type {
        ($ty:ty) => {{
            const SIZE: isize = core::mem::size_of::<$ty>() as isize;
            while i + SIZE < num {
                let ptr: *mut $ty = ptr.cast();
                ptr::write(ptr.offset(i / SIZE), value as _);
                i += SIZE;
            }
        }};
    }

    // We get a value of i32, so do that one first
    impl_type!(i32);
    // No point in doing u16, as this can only get executed once
    // Go straight to i8 (can get executed up to 3 times)
    // This should reduce the amount of branching the CPU has to do
    impl_type!(i8);

    ptr
}
